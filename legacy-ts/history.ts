import type { Env } from "./email";

export interface SendLogEntry {
  id: string;
  createdAt: string;
  fromName: string;
  fromEmail: string;
  to: string[];
  subject: string;
  body: string;
  attachmentNames: string[];
  attachmentSizes: number[];
  ok: boolean;
  messageId?: string;
  error?: string;
}

export interface SendLogSummary {
  id: string;
  createdAt: string;
  fromName: string;
  to: string[];
  subject: string;
  bodyPreview: string;
  attachmentNames: string[];
  ok: boolean;
  messageId?: string;
  error?: string;
}

const INDEX_KEY = "log:index";
const MAX_ENTRIES = 200;
const MAX_BODY_CHARS = 12000;

type HistoryEnv = Env & { MAIL_LOG_KV?: KVNamespace };

function randomId(): string {
  const bytes = new Uint8Array(12);
  crypto.getRandomValues(bytes);
  return Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");
}

function previewBody(body: string): string {
  const t = body.trim();
  if (t.length <= 160) return t;
  return t.slice(0, 160) + "…";
}

async function readIndex(kv: KVNamespace): Promise<string[]> {
  const raw = await kv.get(INDEX_KEY);
  if (!raw) return [];
  try {
    const parsed = JSON.parse(raw) as unknown;
    if (!Array.isArray(parsed)) return [];
    return parsed.map((x) => String(x)).filter(Boolean);
  } catch {
    return [];
  }
}

export async function appendSendLog(
  env: HistoryEnv,
  entry: Omit<SendLogEntry, "id" | "createdAt">,
): Promise<SendLogEntry | null> {
  const kv = env.MAIL_LOG_KV;
  if (!kv) return null;

  const record: SendLogEntry = {
    id: randomId(),
    createdAt: new Date().toISOString(),
    ...entry,
    body: entry.body.slice(0, MAX_BODY_CHARS),
  };

  const ids = await readIndex(kv);
  ids.unshift(record.id);
  const trimmed = ids.slice(0, MAX_ENTRIES);

  await kv.put(`log:${record.id}`, JSON.stringify(record));
  await kv.put(INDEX_KEY, JSON.stringify(trimmed));

  // prune old keys beyond max (best effort)
  for (const oldId of ids.slice(MAX_ENTRIES)) {
    await kv.delete(`log:${oldId}`);
  }

  return record;
}

export async function listSendLogs(env: HistoryEnv, limit = 50): Promise<SendLogSummary[]> {
  const kv = env.MAIL_LOG_KV;
  if (!kv) return [];

  const ids = await readIndex(kv);
  const take = ids.slice(0, Math.min(limit, 100));
  const out: SendLogSummary[] = [];

  for (const id of take) {
    const raw = await kv.get(`log:${id}`);
    if (!raw) continue;
    try {
      const e = JSON.parse(raw) as SendLogEntry;
      out.push({
        id: e.id,
        createdAt: e.createdAt,
        fromName: e.fromName,
        to: e.to,
        subject: e.subject,
        bodyPreview: previewBody(e.body),
        attachmentNames: e.attachmentNames ?? [],
        ok: e.ok,
        messageId: e.messageId,
        error: e.error,
      });
    } catch {
      continue;
    }
  }

  return out;
}

export async function getSendLog(env: HistoryEnv, id: string): Promise<SendLogEntry | null> {
  const kv = env.MAIL_LOG_KV;
  if (!kv || !id) return null;
  const raw = await kv.get(`log:${id}`);
  if (!raw) return null;
  try {
    return JSON.parse(raw) as SendLogEntry;
  } catch {
    return null;
  }
}
