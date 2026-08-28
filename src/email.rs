use crate::config::{mail_logo_url, mail_origin, MailConfig};
use crate::markdown::{self, MarkdownOptions};
use serde::{Deserialize, Serialize};

pub use markdown::{decode_snippet_param, escape_html};

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
        let content: String = item
            .content
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        if content.is_empty() {
            return Err(SendEmailResult {
                ok: false,
                status: 400,
                message: format!("附件 {name} 内容为空"),
                message_id: None,
            });
        }
        let size = item.size.unwrap_or(((content.len() as u64) * 3) / 4);
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

pub fn render_body_html(body: &str, opts: &MarkdownOptions) -> String {
    markdown::render_markdown(body, opts)
}

pub fn wrap_email_html(
    cfg: &MailConfig,
    subject: &str,
    body_html: &str,
    from_name: &str,
    interactive: bool,
) -> String {
    let logo = mail_logo_url(cfg);
    let title = escape_html(subject);
    let brand = escape_html(from_name);
    let contact = escape_html(&cfg.mail.contact_email);
    let site_url = escape_html(&cfg.site.url);
    let site_label = escape_html(&cfg.site.label);
    let brand_name = escape_html(&cfg.site.brand_name);
    let header_bg = format!(
        "linear-gradient(135deg,{} 0%,{} 52%,{} 100%)",
        cfg.brand.tile, cfg.brand.tile_edge, cfg.brand.accent
    );
    let copy_script = if interactive {
        r#"<script>(function(){document.querySelectorAll("a.xxm-copy-btn").forEach(function(a){a.addEventListener("click",function(e){e.preventDefault();var t=a.getAttribute("data-copy")||"";if(navigator.clipboard&&navigator.clipboard.writeText){navigator.clipboard.writeText(t).then(function(){var p=a.textContent;a.textContent="Copied";setTimeout(function(){a.textContent=p;},1200);});}});});})();</script>"#
    } else {
        ""
    };
    format!(
        r##"<!DOCTYPE html>
<html lang="zh">
<head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>{title}</title></head>
<body style="margin:0;padding:0;background:{cream};color:#1c1917;font-family:ui-sans-serif,system-ui,-apple-system,Segoe UI,sans-serif;font-size:14px;line-height:1.65;">
  <div style="max-width:640px;margin:0 auto;padding:28px 16px;">
    <div style="background:#fffdf9;border-radius:14px;border:1px solid #e7e0d6;overflow:hidden;box-shadow:0 12px 40px rgba(21,98,79,.08);">
      <div style="padding:22px 24px;border-bottom:1px solid rgba(255,255,255,.14);background:{header_bg};">
        <div style="font-size:13px;font-weight:600;color:rgba(255,255,255,.82);letter-spacing:.01em;">{brand}</div>
        <div style="margin-top:8px;font-size:17px;font-weight:700;color:#ffffff;line-height:1.35;letter-spacing:-.02em;">{title}</div>
      </div>
      <div style="padding:22px 24px;">{body}</div>
      <div style="padding:22px 24px 24px;border-top:1px solid #ebe8e1;background:#f6f8f6;">
        <div style="max-width:380px;margin:0 auto;background:#ffffff;border:1px solid #e6ece8;border-radius:16px;padding:18px 20px 16px;">
          <a href="{site_url}" target="_blank" rel="noopener noreferrer" style="text-decoration:none;display:block;margin-bottom:16px;">
            <table role="presentation" cellpadding="0" cellspacing="0" border="0" style="margin:0 auto;border-collapse:collapse;">
              <tr>
                <td style="padding-right:14px;vertical-align:middle;">
                  <img src="{logo}" width="40" height="40" alt="{brand_name}" style="display:block;border:0;outline:none;border-radius:11px;"/>
                </td>
                <td style="vertical-align:middle;text-align:left;">
                  <div style="font-size:15px;font-weight:800;color:#1a1c19;letter-spacing:-.03em;line-height:1.2;">{brand_name}</div>
                  <div style="margin-top:5px;font-size:12px;font-weight:700;color:{site_blue};letter-spacing:.01em;">{site_label}</div>
                </td>
              </tr>
            </table>
          </a>
          <div style="height:1px;margin:0 2px 14px;background:#dde5df;"></div>
          <table role="presentation" cellpadding="0" cellspacing="0" border="0" style="margin:0 auto;border-collapse:collapse;">
            <tr>
              <td style="padding-right:12px;vertical-align:middle;font-size:11px;font-weight:700;color:#9aa89f;letter-spacing:.06em;text-transform:uppercase;">Contact</td>
              <td style="vertical-align:middle;">
                <a href="mailto:{contact}" style="display:inline-block;font-size:12px;font-weight:600;color:#3f463d;text-decoration:none;padding:7px 14px;border-radius:999px;background:#f3f6f4;border:1px solid #e2e9e4;">{contact}</a>
              </td>
            </tr>
          </table>
        </div>
      </div>
    </div>
  </div>{copy_script}
</body>
</html>"##,
        cream = cfg.brand.cream,
        site_blue = cfg.brand.site_blue,
        body = body_html,
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
    let html = wrap_email_html(
        cfg,
        subject.trim(),
        &render_body_html(body_for_render, &MarkdownOptions::preview(mail_origin(cfg))),
        &from,
        true,
    );
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
            &render_body_html(&body_for_render, &MarkdownOptions::email(mail_origin(cfg))),
            &from_name,
            false,
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
        payload["attachment"] = serde_json::json!(attachments
            .iter()
            .map(|a| serde_json::json!({ "name": a.name, "content": a.content }))
            .collect::<Vec<_>>());
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

async fn gloo_net_post(
    api_key: &str,
    payload: &serde_json::Value,
) -> Result<(u16, String), String> {
    use worker::{Fetch, Headers, Method, Request, RequestInit};
    let headers = Headers::new();
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

pub fn snippet_page_html(code: &str) -> String {
    let escaped = escape_html(code);
    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>Code snippet</title>
<style>
  body {{ margin:0; padding:24px 16px; background:#f7f4ee; font-family:ui-sans-serif,system-ui,sans-serif; color:#1c1917; }}
  .wrap {{ max-width:720px; margin:0 auto; }}
  .bar {{ display:flex; align-items:center; justify-content:space-between; gap:12px; margin-bottom:12px; }}
  h1 {{ margin:0; font-size:15px; font-weight:800; }}
  button {{ font:inherit; cursor:pointer; border:1px solid #e7e5e4; background:#fff; border-radius:8px; padding:8px 14px; font-weight:700; font-size:13px; }}
  button:hover {{ border-color:#8dcfb8; color:#15624f; }}
  pre {{ margin:0; padding:16px; background:#fff; border:1px solid #e7e0d6; border-radius:12px;
    font-family:Consolas,Courier,monospace; font-size:12px; line-height:1.55; white-space:pre-wrap; word-break:break-word; }}
</style>
</head>
<body>
  <div class="wrap">
    <div class="bar">
      <h1>Code snippet</h1>
      <button type="button" id="copyBtn">Copy</button>
    </div>
    <pre id="code">{escaped}</pre>
  </div>
  <script>
    (function () {{
      var text = {json};
      var btn = document.getElementById("copyBtn");
      btn.addEventListener("click", function () {{
        navigator.clipboard.writeText(text).then(function () {{
          btn.textContent = "Copied";
          setTimeout(function () {{ btn.textContent = "Copy"; }}, 1200);
        }});
      }});
    }})();
  </script>
</body>
</html>"##,
        json = serde_json::to_string(code).unwrap_or_else(|_| "\"\"".into()),
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

    #[test]
    fn render_body_html_uses_commonmark() {
        let html = render_body_html("# Title\n\n**hello**", &MarkdownOptions::default());
        assert!(html.contains("<h1"));
        assert!(html.contains("<strong>hello</strong>"));
    }
}
