import {
  buildEmailPreviewHtml,
  decodeSnippetParam,
  fixedFromEmail,
  resolveFromName,
  sendViaBrevo,
  snippetPageHtml,
  type EmailAttachment,
  type Env,
} from "./email";
import { mailConfig } from "./config";
import { appendSendLog, getSendLog, listSendLogs } from "./history";
import {
  checkLoginAllowed,
  clearLoginFailures,
  formatLockoutMessage,
  recordLoginFailure,
} from "./login-guard";
import {
  clearSessionCookieHeader,
  createSession,
  readSessionToken,
  revokeSession,
  sessionCookieHeader,
  validateSession,
} from "./sessions";
import { faviconSvg } from "./brand";
import { renderAppHtml } from "./ui";

function json(data: unknown, status = 200, extraHeaders?: Record<string, string>): Response {
  return new Response(JSON.stringify(data, null, 2), {
    status,
    headers: {
      "Content-Type": "application/json; charset=utf-8",
      "Cache-Control": "no-store",
      ...extraHeaders,
    },
  });
}

function assertAllowedHost(request: Request, env: Env): void {
  if (env.ALLOW_ANY_HOST === "1") return;
  const raw = request.headers.get("Host") || new URL(request.url).host;
  const host = raw.split(":")[0]?.toLowerCase() ?? "";
  if (host !== mailConfig.host) {
    throw new HttpError(403, `This service is only available at ${mailConfig.host}`);
  }
}

async function assertAuth(request: Request, env: Env): Promise<void> {
  if (!env.MAIL_LOG_KV) {
    throw new HttpError(500, "Session store is not configured");
  }
  const token = readSessionToken(request);
  if (!token || !(await validateSession(env, token))) {
    throw new HttpError(401, "Unauthorized");
  }
}

function timingSafeEqual(a: string, b: string): boolean {
  if (a.length !== b.length) {
    let diff = a.length ^ b.length;
    const max = Math.max(a.length, b.length);
    for (let i = 0; i < max; i++) {
      diff |= (a.charCodeAt(i) || 0) ^ (b.charCodeAt(i) || 0);
    }
    return false;
  }
  let out = 0;
  for (let i = 0; i < a.length; i++) {
    out |= a.charCodeAt(i) ^ b.charCodeAt(i);
  }
  return out === 0;
}

class HttpError extends Error {
  constructor(
    readonly status: number,
    message: string,
    readonly extra?: Record<string, unknown>,
  ) {
    super(message);
  }
}

async function readJson(request: Request): Promise<Record<string, unknown>> {
  const text = await request.text();
  if (!text.trim()) return {};
  try {
    const parsed = JSON.parse(text) as unknown;
    if (parsed && typeof parsed === "object" && !Array.isArray(parsed)) {
      return parsed as Record<string, unknown>;
    }
    return {};
  } catch {
    throw new HttpError(400, "Invalid JSON body");
  }
}

function parseAttachments(raw: unknown): EmailAttachment[] {
  if (!Array.isArray(raw)) return [];
  const out: EmailAttachment[] = [];
  for (const item of raw) {
    if (!item || typeof item !== "object") continue;
    const name = String((item as { name?: unknown }).name ?? "").trim();
    const content = String((item as { content?: unknown }).content ?? "").trim();
    const sizeRaw = (item as { size?: unknown }).size;
    const size = typeof sizeRaw === "number" && Number.isFinite(sizeRaw) ? sizeRaw : undefined;
    if (!name || !content) continue;
    out.push({ name, content, size });
  }
  return out;
}

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    try {
      assertAllowedHost(request, env);
      const url = new URL(request.url);
      const fromName = resolveFromName();

      if (request.method === "GET" && url.pathname === "/favicon.svg") {
        return new Response(faviconSvg(32), {
          headers: {
            "Content-Type": "image/svg+xml; charset=utf-8",
            "Cache-Control": "public, max-age=604800",
          },
        });
      }

      if (request.method === "GET" && url.pathname === "/apple-touch-icon.svg") {
        return new Response(faviconSvg(180), {
          headers: {
            "Content-Type": "image/svg+xml; charset=utf-8",
            "Cache-Control": "public, max-age=604800",
          },
        });
      }

      if (request.method === "GET" && url.pathname === "/snippet") {
        const encoded = url.searchParams.get("e");
        if (!encoded) {
          return new Response("Missing snippet parameter.", { status: 400 });
        }
        try {
          const code = decodeSnippetParam(encoded);
          if (code.length > 8000) {
            return new Response("Snippet too large.", { status: 400 });
          }
          return new Response(snippetPageHtml(code), {
            headers: {
              "Content-Type": "text/html; charset=utf-8",
              "Cache-Control": "no-store",
            },
          });
        } catch {
          return new Response("Invalid snippet.", { status: 400 });
        }
      }

      if (request.method === "GET" && (url.pathname === "/" || url.pathname === "/index.html")) {
        return new Response(
          renderAppHtml({
            fromName,
            fromEmail: fixedFromEmail(),
            addressBook: mailConfig.addressBook,
          }),
          {
            headers: {
              "Content-Type": "text/html; charset=utf-8",
              "Cache-Control": "no-store",
            },
          },
        );
      }

      if (request.method === "GET" && url.pathname === "/api/health") {
        return json({
          ok: true,
          service: "mail",
          from: `${fromName} <${fixedFromEmail()}>`,
          brevo: Boolean(env.BREVO_API_KEY),
          history: Boolean(env.MAIL_LOG_KV),
        });
      }

      if (request.method === "POST" && url.pathname === "/api/login") {
        const body = await readJson(request);
        const password = typeof body.password === "string" ? body.password : "";
        if (!env.ADMIN_PASSWORD) throw new HttpError(500, "ADMIN_PASSWORD is not configured");

        const guard = await checkLoginAllowed(env, request);
        if (!guard.allowed) {
          const retryAfterSec = guard.retryAfterSec ?? 0;
          throw new HttpError(429, formatLockoutMessage(retryAfterSec), {
            locked: true,
            retryAfterSec,
            lockedUntil: guard.lockedUntil,
          });
        }

        if (!timingSafeEqual(password, env.ADMIN_PASSWORD)) {
          const failure = await recordLoginFailure(env, request);
          if (!failure.allowed) {
            const retryAfterSec = failure.retryAfterSec ?? 0;
            throw new HttpError(429, formatLockoutMessage(retryAfterSec), {
              locked: true,
              retryAfterSec,
              lockedUntil: failure.lockedUntil,
            });
          }
          const remaining = failure.attemptsRemaining ?? 0;
          const hint =
            remaining > 0
              ? `Incorrect password. ${remaining} attempt(s) remaining.`
              : "Incorrect password.";
          throw new HttpError(401, hint, { attemptsRemaining: remaining });
        }

        await clearLoginFailures(env, request);
        const session = await createSession(env);
        return json(
          { ok: true, expiresAt: session.expiresAt },
          200,
          { "Set-Cookie": sessionCookieHeader(session.token) },
        );
      }

      if (request.method === "POST" && url.pathname === "/api/logout") {
        const token = readSessionToken(request);
        if (token) await revokeSession(env, token);
        return json({ ok: true }, 200, { "Set-Cookie": clearSessionCookieHeader() });
      }

      if (request.method === "POST" && url.pathname === "/api/session") {
        await assertAuth(request, env);
        return json({
          ok: true,
          fromName: resolveFromName(),
          fromEmail: fixedFromEmail(),
          addressBook: mailConfig.addressBook,
        });
      }

      if (request.method === "POST" && url.pathname === "/api/history") {
        await assertAuth(request, env);
        const body = await readJson(request);
        const limitRaw = body.limit;
        const limit = typeof limitRaw === "number" ? limitRaw : 50;
        const items = await listSendLogs(env, limit);
        return json({ ok: true, items });
      }

      if (request.method === "POST" && url.pathname === "/api/history/detail") {
        await assertAuth(request, env);
        const body = await readJson(request);
        const id = typeof body.id === "string" ? body.id.trim() : "";
        if (!id) throw new HttpError(400, "缺少记录 id");
        const entry = await getSendLog(env, id);
        if (!entry) throw new HttpError(404, "记录不存在");
        return json({ ok: true, entry });
      }

      if (request.method === "POST" && url.pathname === "/api/preview") {
        await assertAuth(request, env);
        const body = await readJson(request);
        const toRaw = body.to;
        const to = Array.isArray(toRaw)
          ? toRaw.map((x) => String(x)).filter(Boolean)
          : typeof toRaw === "string"
            ? toRaw.split(/[,;\s]+/).map((s) => s.trim()).filter(Boolean)
            : [];
        const subject = typeof body.subject === "string" ? body.subject : "";
        const text = typeof body.body === "string" ? body.body : "";
        const nameOverride = typeof body.fromName === "string" ? body.fromName : undefined;
        const attachMeta = body.attachments;
        const hasAttachments = Array.isArray(attachMeta) && attachMeta.length > 0;

        if (!to.length) throw new HttpError(400, "请至少添加一个收件人");
        if (!subject.trim()) throw new HttpError(400, "请填写主题");
        if (!text.trim() && !hasAttachments) throw new HttpError(400, "正文或附件至少一项");

        const preview = buildEmailPreviewHtml({
          subject,
          body: text,
          fromName: nameOverride,
          hasAttachments,
        });

        return json({
          ok: true,
          fromName: preview.fromName,
          fromEmail: fixedFromEmail(),
          to,
          subject: subject.trim(),
          textPreview: preview.textPreview,
          html: preview.html,
        });
      }

      if (request.method === "POST" && url.pathname === "/api/send") {
        await assertAuth(request, env);
        const body = await readJson(request);
        const toRaw = body.to;
        const to = Array.isArray(toRaw)
          ? toRaw.map((x) => String(x)).filter(Boolean)
          : typeof toRaw === "string"
            ? toRaw.split(/[,;\s]+/).map((s) => s.trim()).filter(Boolean)
            : [];
        const subject = typeof body.subject === "string" ? body.subject : "";
        const text = typeof body.body === "string" ? body.body : "";
        const html = body.html === true;
        const nameOverride = typeof body.fromName === "string" ? body.fromName : undefined;
        const attachments = parseAttachments(body.attachments);
        const resolvedFromName = resolveFromName(nameOverride);

        const result = await sendViaBrevo(env, {
          to,
          subject,
          body: text,
          html,
          fromName: nameOverride,
          attachments,
        });

        await appendSendLog(env, {
          fromName: resolvedFromName,
          fromEmail: fixedFromEmail(),
          to,
          subject,
          body: text,
          attachmentNames: attachments.map((a) => a.name),
          attachmentSizes: attachments.map((a) => a.size ?? 0),
          ok: result.ok,
          messageId: result.messageId,
          error: result.ok ? undefined : result.message,
        });

        if (!result.ok) {
          return json({ error: result.message }, result.status >= 400 ? result.status : 502);
        }
        return json({ ok: true, messageId: result.messageId, message: result.message });
      }

      if (env.ASSETS) {
        const assetResponse = await env.ASSETS.fetch(request);
        if (assetResponse.status !== 404) {
          return assetResponse;
        }
      }

      return json({ error: "Not found" }, 404);
    } catch (err) {
      if (err instanceof HttpError) {
        return json({ error: err.message, ...err.extra }, err.status);
      }
      return json({ error: err instanceof Error ? err.message : "Internal error" }, 500);
    }
  },
} satisfies ExportedHandler<Env>;
