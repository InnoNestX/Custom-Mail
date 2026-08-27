import type { Env } from "./email";

const FAIL_PREFIX = "login:fail:";
export const MAX_LOGIN_ATTEMPTS = 3;
export const LOGIN_LOCKOUT_SEC = 15 * 60; // 15 minutes
const FAILURE_TTL_SEC = 60 * 60; // decay idle failure counters after 1 hour

type GuardEnv = Env & { MAIL_LOG_KV?: KVNamespace };

interface LoginGuardRecord {
  failures: number;
  lockedUntil?: string;
}

export interface LoginGuardStatus {
  allowed: boolean;
  retryAfterSec?: number;
  lockedUntil?: string;
  attemptsRemaining?: number;
}

async function sha256Prefix(value: string, len = 24): Promise<string> {
  const data = new TextEncoder().encode(value);
  const hash = await crypto.subtle.digest("SHA-256", data);
  const hex = Array.from(new Uint8Array(hash), (b) => b.toString(16).padStart(2, "0")).join("");
  return hex.slice(0, len);
}

export function getClientIp(request: Request): string {
  const cf = request.headers.get("CF-Connecting-IP");
  if (cf?.trim()) return cf.trim();
  const forwarded = request.headers.get("X-Forwarded-For");
  if (forwarded) {
    const first = forwarded.split(",")[0]?.trim();
    if (first) return first;
  }
  return "unknown";
}

async function guardKey(request: Request): Promise<string> {
  const ip = getClientIp(request);
  const hash = await sha256Prefix(`login-guard:${ip}`);
  return `${FAIL_PREFIX}${hash}`;
}

function parseRecord(raw: string | null): LoginGuardRecord {
  if (!raw) return { failures: 0 };
  try {
    const parsed = JSON.parse(raw) as LoginGuardRecord;
    const failures =
      typeof parsed.failures === "number" && Number.isFinite(parsed.failures)
        ? Math.max(0, Math.floor(parsed.failures))
        : 0;
    const lockedUntil =
      typeof parsed.lockedUntil === "string" && parsed.lockedUntil.trim()
        ? parsed.lockedUntil
        : undefined;
    return { failures, lockedUntil };
  } catch {
    return { failures: 0 };
  }
}

function lockedRetrySec(lockedUntil: string): number {
  const ms = new Date(lockedUntil).getTime() - Date.now();
  return Math.max(1, Math.ceil(ms / 1000));
}

function statusFromRecord(record: LoginGuardRecord): LoginGuardStatus {
  if (record.lockedUntil) {
    const retryAfterSec = lockedRetrySec(record.lockedUntil);
    if (retryAfterSec > 0) {
      return {
        allowed: false,
        retryAfterSec,
        lockedUntil: record.lockedUntil,
        attemptsRemaining: 0,
      };
    }
  }

  const failures = record.failures;
  const attemptsRemaining = Math.max(0, MAX_LOGIN_ATTEMPTS - failures);
  return { allowed: true, attemptsRemaining };
}

async function readStatus(env: GuardEnv, request: Request): Promise<LoginGuardStatus> {
  const kv = env.MAIL_LOG_KV;
  if (!kv) return { allowed: true };

  const key = await guardKey(request);
  const record = parseRecord(await kv.get(key));
  return statusFromRecord(record);
}

export async function checkLoginAllowed(env: GuardEnv, request: Request): Promise<LoginGuardStatus> {
  return readStatus(env, request);
}

export async function recordLoginFailure(env: GuardEnv, request: Request): Promise<LoginGuardStatus> {
  const kv = env.MAIL_LOG_KV;
  if (!kv) {
    return { allowed: true, attemptsRemaining: MAX_LOGIN_ATTEMPTS - 1 };
  }

  const key = await guardKey(request);
  const record = parseRecord(await kv.get(key));

  if (record.lockedUntil && lockedRetrySec(record.lockedUntil) > 0) {
    return statusFromRecord(record);
  }

  const failures = record.lockedUntil ? 1 : record.failures + 1;
  const next: LoginGuardRecord = { failures };

  if (failures >= MAX_LOGIN_ATTEMPTS) {
    const lockedUntil = new Date(Date.now() + LOGIN_LOCKOUT_SEC * 1000).toISOString();
    next.lockedUntil = lockedUntil;
    next.failures = MAX_LOGIN_ATTEMPTS;
    await kv.put(key, JSON.stringify(next), {
      expirationTtl: LOGIN_LOCKOUT_SEC + 300,
    });
    return {
      allowed: false,
      retryAfterSec: LOGIN_LOCKOUT_SEC,
      lockedUntil,
      attemptsRemaining: 0,
    };
  }

  await kv.put(key, JSON.stringify(next), { expirationTtl: FAILURE_TTL_SEC });
  return {
    allowed: true,
    attemptsRemaining: Math.max(0, MAX_LOGIN_ATTEMPTS - failures),
  };
}

export async function clearLoginFailures(env: GuardEnv, request: Request): Promise<void> {
  const kv = env.MAIL_LOG_KV;
  if (!kv) return;
  await kv.delete(await guardKey(request));
}

export function formatLockoutMessage(retryAfterSec: number): string {
  const total = Math.max(1, Math.ceil(retryAfterSec));
  const minutes = Math.ceil(total / 60);
  if (minutes <= 1) return "Too many failed attempts. Try again in about 1 minute.";
  return `Too many failed attempts. Try again in about ${minutes} minutes.`;
}
