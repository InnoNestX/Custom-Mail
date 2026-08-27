use crate::config::{mail_logo_url, mail_origin, MailConfig};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct EmailAttachment {
    pub name: String,
    pub content: String,
    pub size: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct SendEmailInput {
    pub to: Vec<String>,
    pub subject: String,
    pub body: String,
    pub from_name: Option<String>,
    pub html: bool,
    pub attachments: Vec<EmailAttachment>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SendEmailResult {
    pub ok: bool,
    pub status: u16,
    pub message: String,
    #[serde(rename = "messageId", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

const MAX_ATTACHMENT_BYTES: u64 = 8 * 1024 * 1024;
const MAX_TOTAL_ATTACHMENT_BYTES: u64 = 15 * 1024 * 1024;
const MAX_ATTACHMENTS: usize = 8;

pub fn fixed_from_email(cfg: &MailConfig) -> String {
    cfg.mail.from_email.clone()
}

pub fn resolve_from_name(cfg: &MailConfig, override_name: Option<&str>) -> String {
    let candidate = override_name
        .unwrap_or(cfg.mail.from_name_default.as_str())
        .trim();
    let cleaned: String = candidate
        .chars()
        .filter(|c| *c != '<' && *c != '>' && *c != '\r' && *c != '\n')
        .collect::<String>()
        .trim()
        .chars()
        .take(80)
        .collect();
    if cleaned.is_empty() {
        cfg.mail.from_name_default.clone()
    } else {
        cleaned
    }
}

pub fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub fn strip_html_tags(html: &str) -> String {
    let mut result = String::new();
    let bytes = html.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'<' {
            if let Some(rel) = html[i + 1..].find('>') {
                result.push(' ');
                i += rel + 2;
            } else {
                result.push_str(&html[i..]);
                break;
            }
        } else {
            result.push(html[i..].chars().next().unwrap());
            i += html[i..].chars().next().unwrap().len_utf8();
        }
    }
    result
}

fn sanitize_filename(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| match c {
            '/' | '\\' | '<' | '>' | ':' | '"' | '|' | '?' | '*' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect();
    let base = cleaned.trim();
    let base = if base.is_empty() { "file" } else { base };
    base.chars().take(120).collect()
}

fn validate_attachments(raw: &[EmailAttachment]) -> Result<Vec<EmailAttachment>, SendEmailResult> {
    if raw.is_empty() {
        return Ok(vec![]);
    }
    if raw.len() > MAX_ATTACHMENTS {
        return Err(SendEmailResult {
            ok: false,
            status: 400,
            message: format!("最多 {MAX_ATTACHMENTS} 个附件"),
            message_id: None,
        });
    }
    let mut out = Vec::new();
    let mut total = 0u64;
    for item in raw {
        let name = sanitize_filename(&item.name);
        let content: String = item.content.chars().filter(|c| !c.is_whitespace()).collect();
        if content.is_empty() {
            return Err(SendEmailResult {
                ok: false,
                status: 400,
                message: format!("附件 {name} 内容为空"),
                message_id: None,
            });
        }
        let size = item
            .size
            .unwrap_or(((content.len() as u64) * 3) / 4);
        if size > MAX_ATTACHMENT_BYTES {
            return Err(SendEmailResult {
                ok: false,
                status: 400,
                message: format!("附件 {name} 超过 8MB 限制"),
                message_id: None,
            });
        }
        total += size;
        if total > MAX_TOTAL_ATTACHMENT_BYTES {
            return Err(SendEmailResult {
                ok: false,
                status: 400,
                message: "附件总大小超过 15MB 限制".into(),
                message_id: None,
            });
        }
        out.push(EmailAttachment {
            name,
            content,
            size: Some(size),
        });
    }
    Ok(out)
}

/// Lightweight markdown-ish body renderer (fenced code + basic inline).
pub fn render_body_html(body: &str) -> String {
    let normalized = body.replace("\r\n", "\n");
    let mut out = String::new();
    let mut rest = normalized.as_str();
    while let Some(start) = rest.find("```") {
        let before = &rest[..start];
        if !before.is_empty() {
            out.push_str(&render_plain_lines(before));
        }
        rest = &rest[start + 3..];
        let lang_end = rest.find('\n').unwrap_or(0);
        let lang = rest[..lang_end].trim();
        rest = if lang_end < rest.len() {
            &rest[lang_end + 1..]
        } else {
            ""
        };
        if let Some(end) = rest.find("```") {
            let code = rest[..end].trim_end_matches('\n');
            out.push_str(&code_block_html(code, if lang.is_empty() { None } else { Some(lang) }));
            rest = &rest[end + 3..];
        } else {
            out.push_str(&render_plain_lines(rest));
            rest = "";
        }
    }
    if !rest.is_empty() {
        out.push_str(&render_plain_lines(rest));
    }
    if out.is_empty() {
        out.push_str(&render_plain_lines(&normalized));
    }
    out
}

fn render_plain_lines(text: &str) -> String {
    text.split('\n')
        .map(|line| {
            if line.trim().is_empty() {
                "<br>".to_string()
            } else {
                render_inline(line)
            }
        })
        .collect::<Vec<_>>()
        .join("")
}

fn render_inline(line: &str) -> String {
    // very small subset: escape then restore `code` and **bold** and [text](url)
    let mut s = escape_html(line);
    // inline code
    while let Some(a) = s.find('`') {
        if let Some(b) = s[a + 1..].find('`') {
            let end = a + 1 + b;
            let inner = s[a + 1..end].to_string();
            let repl = format!("<code>{}</code>", inner);
            s.replace_range(a..=end, &repl);
        } else {
            break;
        }
    }
    format!("<div>{s}</div>")
}

fn code_block_html(code: &str, lang: Option<&str>) -> String {
    let lang_attr = lang
        .map(|l| format!(" data-lang=\"{}\"", escape_html(l)))
        .unwrap_or_default();
    format!(
        "<pre class=\"code\"{lang_attr}><code>{}</code></pre>",
        escape_html(code)
    )
}

pub fn wrap_email_html(cfg: &MailConfig, subject: &str, body_html: &str, from_name: &str) -> String {
    let logo = mail_logo_url(cfg);
    let origin = mail_origin(cfg);
    format!(
        r##"<!DOCTYPE html><html><head><meta charset="utf-8"><title>{subject}</title></head>
<body style="margin:0;background:{cream};font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif;color:#1a1c19;">
  <table role="presentation" width="100%" cellspacing="0" cellpadding="0" style="background:{cream};padding:24px 12px;">
    <tr><td align="center">
      <table role="presentation" width="100%" style="max-width:640px;background:#fff;border-radius:16px;overflow:hidden;border:1px solid #e5ebe3;">
        <tr><td style="padding:20px 24px;background:{tile};color:#fff;">
          <img src="{logo}" alt="" width="36" height="36" style="vertical-align:middle;border-radius:10px;">
          <span style="margin-left:10px;font-weight:700;vertical-align:middle;">{brand}</span>
        </td></tr>
        <tr><td style="padding:28px 24px;">
          <div style="font-size:13px;color:#6f776c;margin-bottom:8px;">From {from_name}</div>
          <h1 style="margin:0 0 16px;font-size:22px;">{subject}</h1>
          <div style="line-height:1.65;font-size:15px;">{body}</div>
        </td></tr>
        <tr><td style="padding:16px 24px;background:#f7f4ee;font-size:12px;color:#6f776c;">
          Sent via <a href="{origin}" style="color:{accent};">{host}</a>
        </td></tr>
      </table>
    </td></tr>
  </table>
</body></html>"##,
        subject = escape_html(subject),
        cream = cfg.brand.cream,
        tile = cfg.brand.tile,
        logo = logo,
        brand = escape_html(&cfg.site.brand_name),
        from_name = escape_html(from_name),
        body = body_html,
        origin = origin,
        accent = cfg.brand.accent,
        host = escape_html(&cfg.host),
    )
}

pub fn build_email_preview_html(
    cfg: &MailConfig,
    subject: &str,
    body: &str,
    from_name: Option<&str>,
    has_attachments: bool,
) -> (String, String, String) {
    let from = resolve_from_name(cfg, from_name);
    let body_trim = body.trim();
    let body_for_render = if body_trim.is_empty() && has_attachments {
        "（附件邮件，无正文）"
    } else {
        body_trim
    };
    let html = wrap_email_html(cfg, subject.trim(), &render_body_html(body_for_render), &from);
    (from, html, body_for_render.to_string())
}

pub async fn send_via_brevo(
    cfg: &MailConfig,
    api_key: &str,
    input: SendEmailInput,
) -> SendEmailResult {
    if api_key.is_empty() {
        return SendEmailResult {
            ok: false,
            status: 500,
            message: "BREVO_API_KEY is not configured".into(),
            message_id: None,
        };
    }

    let mut to: Vec<String> = input
        .to
        .iter()
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();
    to.sort();
    to.dedup();
    if to.is_empty() {
        return SendEmailResult {
            ok: false,
            status: 400,
            message: "At least one recipient is required".into(),
            message_id: None,
        };
    }
    for addr in &to {
        if !addr.contains('@') || addr.starts_with('@') || addr.ends_with('@') {
            return SendEmailResult {
                ok: false,
                status: 400,
                message: format!("Invalid recipient: {addr}"),
                message_id: None,
            };
        }
    }

    let subject = input.subject.trim().to_string();
    if subject.is_empty() {
        return SendEmailResult {
            ok: false,
            status: 400,
            message: "Subject is required".into(),
            message_id: None,
        };
    }

    let attachments = match validate_attachments(&input.attachments) {
        Ok(a) => a,
        Err(e) => return e,
    };

    let body = input.body.trim().to_string();
    if body.is_empty() && attachments.is_empty() {
        return SendEmailResult {
            ok: false,
            status: 400,
            message: "正文或附件至少填写一项".into(),
            message_id: None,
        };
    }

    let from_name = resolve_from_name(cfg, input.from_name.as_deref());
    let body_for_render = if body.is_empty() && !attachments.is_empty() {
        "（附件邮件，无正文）".to_string()
    } else {
        body.clone()
    };
    let html_content = if input.html {
        body_for_render.clone()
    } else {
        wrap_email_html(
            cfg,
            &subject,
            &render_body_html(&body_for_render),
            &from_name,
        )
    };
    let text_content = if input.html {
        strip_html_tags(&body_for_render)
    } else {
        body_for_render
    };

    let mut payload = serde_json::json!({
        "sender": { "email": cfg.mail.from_email, "name": from_name },
        "to": to.iter().map(|email| serde_json::json!({ "email": email })).collect::<Vec<_>>(),
        "subject": subject,
        "htmlContent": html_content,
        "textContent": text_content,
        "tags": [cfg.mail.brevo_tag],
    });
    if !attachments.is_empty() {
        payload["attachment"] = serde_json::json!(
            attachments
                .iter()
                .map(|a| serde_json::json!({ "name": a.name, "content": a.content }))
                .collect::<Vec<_>>()
        );
    }

    let result = gloo_net_post(api_key, &payload).await;
    match result {
        Ok((status, raw)) => {
            let parsed: Option<serde_json::Value> = serde_json::from_str(&raw).ok();
            let message_id = parsed.as_ref().and_then(|p| {
                p.get("messageId")
                    .and_then(|v| v.as_str())
                    .map(str::to_string)
                    .or_else(|| {
                        p.get("messageIds")
                            .and_then(|v| v.as_array())
                            .and_then(|a| a.first())
                            .and_then(|v| v.as_str())
                            .map(str::to_string)
                    })
            });
            if (200..300).contains(&status) && message_id.is_some() {
                SendEmailResult {
                    ok: true,
                    status,
                    message: "Delivered".into(),
                    message_id,
                }
            } else {
                let err = parsed
                    .as_ref()
                    .and_then(|p| {
                        p.get("message")
                            .or_else(|| p.get("msg"))
                            .and_then(|v| v.as_str())
                    })
                    .map(str::to_string)
                    .unwrap_or_else(|| raw.chars().take(300).collect());
                SendEmailResult {
                    ok: false,
                    status,
                    message: if err.is_empty() {
                        "Brevo request failed".into()
                    } else {
                        err
                    },
                    message_id: None,
                }
            }
        }
        Err(e) => SendEmailResult {
            ok: false,
            status: 502,
            message: e,
            message_id: None,
        },
    }
}

async fn gloo_net_post(api_key: &str, payload: &serde_json::Value) -> Result<(u16, String), String> {
    use worker::{Fetch, Method, Request, RequestInit, Headers};
    let mut headers = Headers::new();
    headers.set("api-key", api_key).map_err(|e| e.to_string())?;
    headers
        .set("Content-Type", "application/json")
        .map_err(|e| e.to_string())?;
    let mut init = RequestInit::new();
    init.with_method(Method::Post)
        .with_headers(headers)
        .with_body(Some(wasm_bindgen::JsValue::from_str(
            &serde_json::to_string(payload).unwrap(),
        )));
    let req = Request::new_with_init("https://api.brevo.com/v3/smtp/email", &init)
        .map_err(|e| e.to_string())?;
    let mut resp = Fetch::Request(req)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = resp.status_code();
    let text = resp.text().await.map_err(|e| e.to_string())?;
    Ok((status, text))
}

pub fn decode_snippet_param(encoded: &str) -> Result<String, String> {
    let s = encoded.replace('-', "+").replace('_', "/");
    let pad = match s.len() % 4 {
        0 => "",
        2 => "==",
        3 => "=",
        _ => return Err("Invalid snippet".into()),
    };
    let s = format!("{s}{pad}");
    let bytes = base64_decode(&s)?;
    String::from_utf8(bytes).map_err(|_| "Invalid snippet".into())
}

fn base64_decode(data: &str) -> Result<Vec<u8>, String> {
    const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = Vec::new();
    let mut buf = 0u32;
    let mut bits = 0u32;
    for c in data.chars() {
        if c == '=' {
            break;
        }
        let v = T
            .iter()
            .position(|&x| x == c as u8)
            .ok_or_else(|| "Invalid snippet".to_string())? as u32;
        buf = (buf << 6) | v;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buf >> bits) as u8);
            buf &= (1 << bits) - 1;
        }
    }
    Ok(out)
}

pub fn snippet_page_html(code: &str) -> String {
    format!(
        r##"<!DOCTYPE html><html><head><meta charset="utf-8"><title>Snippet</title>
<style>body{{font-family:ui-monospace,monospace;background:#111;color:#e8e8e8;padding:24px}}pre{{white-space:pre-wrap;word-break:break-word}}</style>
</head><body><pre>{}</pre></body></html>"##,
        escape_html(code)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_tags_linearly() {
        assert_eq!(strip_html_tags("<b>hi</b>"), " hi ");
        assert_eq!(strip_html_tags("a<br>b"), "a b");
    }

    #[test]
    fn escape_works() {
        assert_eq!(escape_html("a<b>&\""), "a&lt;b&gt;&amp;&quot;");
    }

    #[test]
    fn resolve_name_cleans() {
        let cfg = crate::config::load_config();
        let n = resolve_from_name(&cfg, Some("  Foo<script> "));
        assert!(!n.contains('<'));
    }
}
