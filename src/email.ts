import { BRAND_ACCENT, BRAND_CREAM, BRAND_TILE, BRAND_TILE_EDGE, SITE_BRAND_BLUE } from "./brand";
import { mailConfig, mailLogoUrl, mailOrigin } from "./config";

export interface Env {
  BREVO_API_KEY: string;
  ADMIN_PASSWORD: string;
  MAIL_LOG_KV?: KVNamespace;
  ASSETS?: Fetcher;
}

export type { AddressBookEntry } from "./config";

export function fixedFromEmail(): string {
  return mailConfig.mail.fromEmail;
}

export function resolveFromName(override?: string): string {
  const candidate = (override ?? mailConfig.mail.fromNameDefault).trim();
  const cleaned = candidate.replace(/[<>\r\n]/g, "").trim();
  return cleaned.slice(0, 80) || mailConfig.mail.fromNameDefault;
}

export function buildSender(fromName: string): { email: string; name: string } {
  return { email: mailConfig.mail.fromEmail, name: fromName };
}

function escapeHtml(str: string): string {
  return str
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

/** Strip HTML tags without regex — avoids ReDoS on malicious `<` sequences. */
function stripHtmlTags(html: string): string {
  let result = "";
  let i = 0;
  while (i < html.length) {
    if (html[i] === "<") {
      const end = html.indexOf(">", i + 1);
      if (end === -1) {
        result += html.slice(i);
        break;
      }
      result += " ";
      i = end + 1;
    } else {
      result += html[i];
      i++;
    }
  }
  return result;
}

export function renderBodyHtml(body: string, opts?: { interactive?: boolean }): string {
  const normalized = body.replace(/\r\n/g, "\n");
  const segments: Array<{ type: "text"; value: string } | { type: "code"; lang?: string; code: string }> = [];
  const fencedRe = /```([a-zA-Z][a-zA-Z0-9_+-]*)?\s*\n?([\s\S]*?)```/g;
  let lastIndex = 0;
  let match: RegExpExecArray | null;
  while ((match = fencedRe.exec(normalized)) !== null) {
    if (match.index > lastIndex) {
      segments.push({ type: "text", value: normalized.slice(lastIndex, match.index) });
    }
    const code = match[2].replace(/\n$/, "");
    segments.push({ type: "code", lang: match[1] || undefined, code });
    lastIndex = fencedRe.lastIndex;
  }
  if (lastIndex < normalized.length) {
    segments.push({ type: "text", value: normalized.slice(lastIndex) });
  }
  if (!segments.length) {
    segments.push({ type: "text", value: normalized });
  }

  return segments
    .map((seg) =>
      seg.type === "code"
        ? codeBlockHtml(seg.code, seg.lang, opts?.interactive)
        : renderPlainText(seg.value, opts),
    )
    .join("");
}

function renderPlainText(text: string, opts?: { interactive?: boolean }): string {
  const lines = text.split("\n");
  const blocks: string[] = [];
  let i = 0;
  while (i < lines.length) {
    const raw = lines[i];
    const line = raw.trimEnd();
    if (!line.trim()) {
      blocks.push("<br>");
      i++;
      continue;
    }

    if (tickCount(line) % 2 === 1) {
      let chunk = line;
      i++;
      while (i < lines.length && tickCount(chunk) % 2 === 1) {
        chunk += "\n" + lines[i].trimEnd();
        i++;
      }
      blocks.push(renderMarkdownLine(chunk, opts));
      continue;
    }

    blocks.push(renderMarkdownLine(line, opts));
    i++;
  }
  return blocks.join("");
}

function tickCount(value: string): number {
  return (value.match(/`/g) || []).length;
}

function renderMarkdownLine(line: string, opts?: { interactive?: boolean }): string {
  if (line.startsWith("### ")) {
    return `<h3 style="margin:14px 0 6px;font-size:15px;">${inline(line.slice(4), opts)}</h3>`;
  }
  if (line.startsWith("## ")) {
    return `<h2 style="margin:14px 0 6px;font-size:16px;">${inline(line.slice(3), opts)}</h2>`;
  }
  if (line.startsWith("# ")) {
    return `<h1 style="margin:14px 0 6px;font-size:18px;">${inline(line.slice(2), opts)}</h1>`;
  }
  if (/^[-*]\s+/.test(line)) {
    return `<div style="margin:2px 0 2px 16px;">• ${inline(line.replace(/^[-*]\s+/, ""), opts)}</div>`;
  }
  return wrapLine(line, opts);
}

function wrapLine(line: string, opts?: { interactive?: boolean }): string {
  const content = inline(line, opts);
  if (content.includes("class=\"xxm-code-block\"")) {
    return content;
  }
  return `<p style="margin:6px 0;">${content}</p>`;
}

const INLINE_CODE_STYLE =
  "display:inline-block;font-family:ui-monospace,SFMono-Regular,Menlo,Consolas,monospace;font-size:13px;background:#f5f4f1;border:1px solid #e7e5e4;border-radius:6px;padding:2px 7px;color:#1c1917;user-select:all;-webkit-user-select:all;";

const COPY_BTN_STYLE =
  "font-size:10px;font-weight:700;color:#57534e;background:#fff;border:1px solid #e7e5e4;border-radius:5px;padding:3px 10px;text-decoration:none;line-height:1.4;font-family:Arial,Helvetica,sans-serif;";

const SNIPPET_MAX_CHARS = 1400;

function encodeBase64Url(text: string): string {
  const bytes = new TextEncoder().encode(text);
  let binary = "";
  for (const byte of bytes) binary += String.fromCharCode(byte);
  return btoa(binary).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/g, "");
}

export function decodeSnippetParam(encoded: string): string {
  const padLen = (4 - (encoded.length % 4)) % 4;
  const b64 = (encoded + "=".repeat(padLen)).replace(/-/g, "+").replace(/_/g, "/");
  const binary = atob(b64);
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) bytes[i] = binary.charCodeAt(i);
  return new TextDecoder().decode(bytes);
}

function snippetUrl(code: string): string | null {
  if (code.length > SNIPPET_MAX_CHARS) return null;
  return `${mailOrigin()}/snippet?e=${encodeBase64Url(code)}`;
}

export function snippetPageHtml(code: string): string {
  const escaped = escapeHtml(code);
  return `<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>Code snippet</title>
<style>
  body { margin:0; padding:24px 16px; background:#f7f4ee; font-family:ui-sans-serif,system-ui,sans-serif; color:#1c1917; }
  .wrap { max-width:720px; margin:0 auto; }
  .bar { display:flex; align-items:center; justify-content:space-between; gap:12px; margin-bottom:12px; }
  h1 { margin:0; font-size:15px; font-weight:800; }
  button { font:inherit; cursor:pointer; border:1px solid #e7e5e4; background:#fff; border-radius:8px; padding:8px 14px; font-weight:700; font-size:13px; }
  button:hover { border-color:#8dcfb8; color:#15624f; }
  pre { margin:0; padding:16px; background:#fff; border:1px solid #e7e0d6; border-radius:12px;
    font-family:Consolas,Courier,monospace; font-size:12px; line-height:1.55; white-space:pre-wrap; word-break:break-word; }
</style>
</head>
<body>
  <div class="wrap">
    <div class="bar">
      <h1>Code snippet</h1>
      <button type="button" id="copyBtn">Copy</button>
    </div>
    <pre id="code">${escaped}</pre>
  </div>
  <script>
    (function () {
      var text = ${JSON.stringify(code)};
      var btn = document.getElementById("copyBtn");
      btn.addEventListener("click", function () {
        navigator.clipboard.writeText(text).then(function () {
          btn.textContent = "Copied";
          setTimeout(function () { btn.textContent = "Copy"; }, 1200);
        });
      });
    })();
  </script>
</body>
</html>`;
}

function isLargeCode(code: string): boolean {
  return code.includes("\n") || code.length > 96;
}

function copyActionHtml(code: string, interactive?: boolean): string {
  if (interactive) {
    const copyAttr = escapeHtml(code).replace(/"/g, "&quot;");
    return `<a href="#" class="xxm-copy-btn" data-copy="${copyAttr}" style="${COPY_BTN_STYLE}">Copy</a>`;
  }
  const url = snippetUrl(code);
  if (url) {
    return `<a href="${url}" target="_blank" rel="noopener noreferrer" style="${COPY_BTN_STYLE}">Copy</a>`;
  }
  return `<span style="font-size:10px;font-weight:600;color:#9aa89f;font-family:Arial,Helvetica,sans-serif;">Select to copy</span>`;
}

function codeBlockHeaderHtml(code: string, lang?: string, interactive?: boolean): string {
  const langLabel = lang ? escapeHtml(lang) : "";
  const langCell = langLabel
    ? `<td align="right" valign="middle" style="font-size:10px;font-weight:700;color:#78716c;text-transform:lowercase;font-family:Arial,Helvetica,sans-serif;white-space:nowrap;">${langLabel}</td>`
    : "";
  const inner =
    `<table role="presentation" align="right" cellpadding="0" cellspacing="0" border="0" style="border-collapse:collapse;mso-table-lspace:0;mso-table-rspace:0;margin:0;">` +
    `<tr>` +
    `<td align="right" valign="middle" style="padding:0 8px 0 0;font-family:Arial,Helvetica,sans-serif;">${copyActionHtml(code, interactive)}</td>` +
    langCell +
    `</tr></table>`;
  return (
    `<table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0" style="border-collapse:collapse;mso-table-lspace:0;mso-table-rspace:0;">` +
    `<tr><td align="right" style="text-align:right;padding:0;">${inner}</td></tr></table>`
  );
}

function codeBlockHtml(code: string, lang?: string, interactive?: boolean): string {
  const escaped = escapeHtml(code);
  return (
    `<table role="presentation" class="xxm-code-block" width="100%" cellpadding="0" cellspacing="0" border="0" style="margin:12px 0;border:1px solid #e7e5e4;border-radius:8px;background:#f5f4f1;border-collapse:separate;mso-table-lspace:0;mso-table-rspace:0;">` +
    `<tr><td style="padding:8px 12px;border-bottom:1px solid #e7e5e4;background:#eef3ea;">` +
    codeBlockHeaderHtml(code, lang, interactive) +
    `</td></tr>` +
    `<tr><td style="padding:12px 14px;font-family:Consolas,Courier,monospace;font-size:12px;line-height:1.55;color:#1c1917;white-space:pre-wrap;word-wrap:break-word;word-break:break-word;-webkit-user-select:all;user-select:all;">${escaped}</td></tr>` +
    `</table>`
  );
}

function inlineCodeHtml(code: string, lang?: string, interactive?: boolean): string {
  if (isLargeCode(code) || lang) {
    return codeBlockHtml(code, lang, interactive);
  }
  const escaped = escapeHtml(code);
  return `<code style="${INLINE_CODE_STYLE}">${escaped}</code>`;
}

function inline(value: string, opts?: { interactive?: boolean }): string {
  const escaped = escapeHtml(value);
  return escaped
    .replace(/\[([^\]]+)\]\((https?:\/\/[^)\s]+)\)/g, (_m, text, url) => `<a href="${url}" style="color:#0f766e;">${text}</a>`)
    .replace(/\*\*(.*?)\*\*/g, "<strong>$1</strong>")
    .replace(/`([a-zA-Z][a-zA-Z0-9_+-]*):([^`]+)`/g, (_m, lang, code) => inlineCodeHtml(code, lang, opts?.interactive))
    .replace(/`([^`]+)`/g, (_m, code) => inlineCodeHtml(code, undefined, opts?.interactive));
}

function emailFooterHtml(siteUrl: string, siteLabel: string, logoUrl: string, contact: string): string {
  const brandName = escapeHtml(mailConfig.site.brandName);
  const siteBlue = SITE_BRAND_BLUE;
  return (
    `<div style="padding:22px 24px 24px;border-top:1px solid #ebe8e1;background:#f6f8f6;">` +
    `<div style="max-width:380px;margin:0 auto;background:#ffffff;border:1px solid #e6ece8;border-radius:16px;padding:18px 20px 16px;box-shadow:0 4px 18px rgba(26,28,25,.04);">` +
    `<a href="${siteUrl}" target="_blank" rel="noopener noreferrer" style="text-decoration:none;display:block;margin-bottom:16px;">` +
    `<table role="presentation" cellpadding="0" cellspacing="0" border="0" style="margin:0 auto;border-collapse:collapse;">` +
    `<tr>` +
    `<td style="padding-right:14px;vertical-align:middle;">` +
    `<img src="${logoUrl}" width="40" height="40" alt="${brandName}" style="display:block;border:0;outline:none;border-radius:11px;"/>` +
    `</td>` +
    `<td style="vertical-align:middle;text-align:left;">` +
    `<div style="font-size:15px;font-weight:800;color:#1a1c19;letter-spacing:-.03em;line-height:1.2;">${brandName}</div>` +
    `<div style="margin-top:5px;font-size:12px;font-weight:700;color:${siteBlue};letter-spacing:.01em;">${siteLabel}</div>` +
    `</td>` +
    `</tr>` +
    `</table>` +
    `</a>` +
    `<div style="height:1px;margin:0 2px 14px;background:linear-gradient(90deg,rgba(221,227,216,0),#dde5df 20%,#dde5df 80%,rgba(221,227,216,0));"></div>` +
    `<table role="presentation" cellpadding="0" cellspacing="0" border="0" style="margin:0 auto;border-collapse:collapse;">` +
    `<tr>` +
    `<td style="padding-right:12px;vertical-align:middle;font-size:11px;font-weight:700;color:#9aa89f;letter-spacing:.06em;text-transform:uppercase;">Contact</td>` +
    `<td style="vertical-align:middle;">` +
    `<a href="mailto:${contact}" style="display:inline-block;font-size:12px;font-weight:600;color:#3f463d;text-decoration:none;padding:7px 14px;border-radius:999px;background:#f3f6f4;border:1px solid #e2e9e4;">${contact}</a>` +
    `</td>` +
    `</tr>` +
    `</table>` +
    `</div>` +
    `</div>`
  );
}

export function wrapEmailHtml(
  subject: string,
  bodyHtml: string,
  fromName: string,
  opts?: { interactive?: boolean },
): string {
  const title = escapeHtml(subject);
  const brand = escapeHtml(fromName);
  const contact = escapeHtml(mailConfig.mail.contactEmail);
  const siteUrl = escapeHtml(mailConfig.site.url);
  const siteLabel = escapeHtml(mailConfig.site.label);
  const logoUrl = escapeHtml(mailLogoUrl());
  const headerBg = `linear-gradient(135deg,${BRAND_TILE} 0%,${BRAND_TILE_EDGE} 52%,${BRAND_ACCENT} 100%)`;
  const copyScript = opts?.interactive
    ? `<script>(function(){document.querySelectorAll("a.xxm-copy-btn").forEach(function(a){a.addEventListener("click",function(e){e.preventDefault();var t=a.getAttribute("data-copy")||"";if(navigator.clipboard&&navigator.clipboard.writeText){navigator.clipboard.writeText(t).then(function(){var p=a.textContent;a.textContent="Copied";setTimeout(function(){a.textContent=p;},1200);});}});});})();</script>`
    : "";
  return `<!DOCTYPE html>
<html lang="zh">
<head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>${title}</title></head>
<body style="margin:0;padding:0;background:${BRAND_CREAM};color:#1c1917;font-family:ui-sans-serif,system-ui,-apple-system,Segoe UI,sans-serif;font-size:14px;line-height:1.65;">
  <div style="max-width:640px;margin:0 auto;padding:28px 16px;">
    <div style="background:#fffdf9;border-radius:14px;border:1px solid #e7e0d6;overflow:hidden;box-shadow:0 12px 40px rgba(21,98,79,.08);">
      <div style="padding:22px 24px;border-bottom:1px solid rgba(255,255,255,.14);background:${headerBg};">
        <div style="font-size:13px;font-weight:600;color:rgba(255,255,255,.82);letter-spacing:.01em;">${brand}</div>
        <div style="margin-top:8px;font-size:17px;font-weight:700;color:#ffffff;line-height:1.35;letter-spacing:-.02em;">${title}</div>
      </div>
      <div style="padding:22px 24px;">${bodyHtml}</div>
      ${emailFooterHtml(siteUrl, siteLabel, logoUrl, contact)}
    </div>
  </div>${copyScript}
</body>
</html>`;
}

export interface EmailAttachment {
  name: string;
  content: string;
  size?: number;
}

export interface SendEmailInput {
  to: string[];
  subject: string;
  body: string;
  fromName?: string;
  html?: boolean;
  attachments?: EmailAttachment[];
}

export interface SendEmailResult {
  ok: boolean;
  status: number;
  message: string;
  messageId?: string;
}

const MAX_ATTACHMENT_BYTES = 8 * 1024 * 1024;
const MAX_TOTAL_ATTACHMENT_BYTES = 15 * 1024 * 1024;
const MAX_ATTACHMENTS = 8;

function sanitizeFilename(name: string): string {
  const base = name.replace(/[/\\<>:"|?*\x00-\x1f]/g, "_").trim() || "file";
  return base.slice(0, 120);
}

function validateAttachments(raw: EmailAttachment[] | undefined): EmailAttachment[] | SendEmailResult {
  if (!raw?.length) return [];
  if (raw.length > MAX_ATTACHMENTS) {
    return { ok: false, status: 400, message: `最多 ${MAX_ATTACHMENTS} 个附件` };
  }

  const out: EmailAttachment[] = [];
  let total = 0;

  for (const item of raw) {
    const name = sanitizeFilename(String(item.name ?? "file"));
    const content = String(item.content ?? "").replace(/\s/g, "");
    if (!content) {
      return { ok: false, status: 400, message: `附件 ${name} 内容为空` };
    }

    const size = item.size ?? Math.floor((content.length * 3) / 4);
    if (size > MAX_ATTACHMENT_BYTES) {
      return { ok: false, status: 400, message: `附件 ${name} 超过 8MB 限制` };
    }
    total += size;
    if (total > MAX_TOTAL_ATTACHMENT_BYTES) {
      return { ok: false, status: 400, message: "附件总大小超过 15MB 限制" };
    }

    out.push({ name, content, size });
  }

  return out;
}

export function buildEmailPreviewHtml(
  input: { subject: string; body: string; fromName?: string; hasAttachments?: boolean },
): { fromName: string; html: string; textPreview: string } {
  const subject = input.subject.trim();
  const fromName = resolveFromName(input.fromName);
  const body = input.body.trim();
  const bodyForRender = body || (input.hasAttachments ? "（附件邮件，无正文）" : "");
  const html = wrapEmailHtml(subject, renderBodyHtml(bodyForRender, { interactive: true }), fromName, {
    interactive: true,
  });
  return { fromName, html, textPreview: bodyForRender };
}

export async function sendViaBrevo(env: Env, input: SendEmailInput): Promise<SendEmailResult> {
  if (!env.BREVO_API_KEY) {
    return { ok: false, status: 500, message: "BREVO_API_KEY is not configured" };
  }

  const to = [...new Set(input.to.map((s) => s.trim().toLowerCase()).filter(Boolean))];
  if (!to.length) {
    return { ok: false, status: 400, message: "At least one recipient is required" };
  }
  for (const addr of to) {
    if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(addr)) {
      return { ok: false, status: 400, message: `Invalid recipient: ${addr}` };
    }
  }

  const subject = input.subject.trim();
  if (!subject) {
    return { ok: false, status: 400, message: "Subject is required" };
  }

  const attachmentCheck = validateAttachments(input.attachments);
  if (!Array.isArray(attachmentCheck)) {
    return attachmentCheck;
  }
  const attachments = attachmentCheck;

  const body = input.body.trim();
  if (!body && !attachments.length) {
    return { ok: false, status: 400, message: "正文或附件至少填写一项" };
  }

  const fromName = resolveFromName(input.fromName);
  const sender = buildSender(fromName);
  const bodyForRender = body || (attachments.length ? "（附件邮件，无正文）" : "");
  const htmlContent = input.html
    ? bodyForRender
    : wrapEmailHtml(subject, renderBodyHtml(bodyForRender), fromName);
  const textContent = input.html ? stripHtmlTags(bodyForRender) : bodyForRender;

  const payload: Record<string, unknown> = {
    sender,
    to: to.map((email) => ({ email })),
    subject,
    htmlContent,
    textContent,
    tags: [mailConfig.mail.brevoTag],
  };

  if (attachments.length) {
    payload.attachment = attachments.map((a) => ({ name: a.name, content: a.content }));
  }

  const response = await fetch("https://api.brevo.com/v3/smtp/email", {
    method: "POST",
    headers: {
      "api-key": env.BREVO_API_KEY,
      "Content-Type": "application/json",
    },
    body: JSON.stringify(payload),
  });

  const raw = await response.text().catch(() => "");
  let parsed: Record<string, unknown> | undefined;
  try {
    parsed = JSON.parse(raw) as Record<string, unknown>;
  } catch {
    parsed = undefined;
  }

  const messageId =
    typeof parsed?.messageId === "string"
      ? parsed.messageId
      : Array.isArray(parsed?.messageIds)
        ? String(parsed.messageIds[0] ?? "")
        : undefined;

  if (response.ok && messageId) {
    return { ok: true, status: response.status, message: "Delivered", messageId };
  }

  const errMsg =
    (parsed && String(parsed.message ?? parsed.msg ?? "")) ||
    raw.slice(0, 300) ||
    "Brevo request failed";

  return { ok: false, status: response.status, message: errMsg };
}
