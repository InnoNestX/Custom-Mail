import type { Env } from "./email";

const SESSION_PREFIX = "session:";
const REGISTRY_KEY = "session:registry";
export const SESSION_COOKIE = "xxm_session";
export const SESSION_TTL_SEC = 60 * 60 * 24; // 24 hours

export interface SessionRecord {
  createdAt: string;
  expiresAt: string;
}

type SessionEnv = Env & { MAIL_LOG_KV?: KVNamespace };

function sessionKey(token: string): string {
  return `${SESSION_PREFIX}${token}`;
}

function randomToken(): string {
  const bytes = new Uint8Array(32);
  crypto.getRandomValues(bytes);
  return Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");
}

function parseRegistry(raw: string | null): string[] {
  if (!raw) return [];
  try {
    const parsed = JSON.parse(raw) as unknown;
    if (!Array.isArray(parsed)) return [];
    return parsed.map((x) => String(x)).filter(Boolean);
  } catch {
    return [];
  }
}

export function readSessionToken(request: Request): string | undefined {
  const cookie = request.headers.get("Cookie") || "";
  for (const part of cookie.split(";")) {
    const trimmed = part.trim();
    if (trimmed.startsWith(`${SESSION_COOKIE}=`)) {
      const value = trimmed.slice(SESSION_COOKIE.length + 1).trim();
      if (value) return value;
    }
  }
  return undefined;
}

export function sessionCookieHeader(token: string): string {
  return `${SESSION_COOKIE}=${token}; Path=/; HttpOnly; Secure; SameSite=Strict; Max-Age=${SESSION_TTL_SEC}`;
}

export function clearSessionCookieHeader(): string {
  return `${SESSION_COOKIE}=; Path=/; HttpOnly; Secure; SameSite=Strict; Max-Age=0`;
}

export async function revokeAllSessions(env: SessionEnv): Promise<void> {
  const kv = env.MAIL_LOG_KV;
  if (!kv) return;
  const tokens = parseRegistry(await kv.get(REGISTRY_KEY));
  for (const token of tokens) {
    await kv.delete(sessionKey(token));
  }
  await kv.delete(REGISTRY_KEY);
}

export async function revokeSession(env: SessionEnv, token: string): Promise<void> {
  const kv = env.MAIL_LOG_KV;
  if (!kv || !token) return;
  await kv.delete(sessionKey(token));
  const tokens = parseRegistry(await kv.get(REGISTRY_KEY)).filter((t) => t !== token);
  if (tokens.length) {
    await kv.put(REGISTRY_KEY, JSON.stringify(tokens));
  } else {
    await kv.delete(REGISTRY_KEY);
  }
}

export async function createSession(env: SessionEnv): Promise<{ token: string; expiresAt: string }> {
  const kv = env.MAIL_LOG_KV;
  if (!kv) {
    throw new Error("MAIL_LOG_KV is not configured");
  }

  await revokeAllSessions(env);

  const token = randomToken();
  const now = Date.now();
  const expiresAt = new Date(now + SESSION_TTL_SEC * 1000).toISOString();
  const record: SessionRecord = { createdAt: new Date(now).toISOString(), expiresAt };

  await kv.put(sessionKey(token), JSON.stringify(record), { expirationTtl: SESSION_TTL_SEC });
  await kv.put(REGISTRY_KEY, JSON.stringify([token]), { expirationTtl: SESSION_TTL_SEC });

  return { token, expiresAt };
}

export async function validateSession(env: SessionEnv, token: string): Promise<boolean> {
  const kv = env.MAIL_LOG_KV;
  if (!kv || !token) return false;

  const raw = await kv.get(sessionKey(token));
  if (!raw) return false;

  try {
    const record = JSON.parse(raw) as SessionRecord;
    if (new Date(record.expiresAt).getTime() <= Date.now()) {
      await revokeSession(env, token);
      return false;
    }
    return true;
  } catch {
    await revokeSession(env, token);
    return false;
  }
}
