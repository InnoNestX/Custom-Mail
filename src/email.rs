use crate::config::MailConfig;
use crate::markdown::{self, MarkdownOptions};
use crate::plugins::{
    resolve_theme, send_via_provider, wrap_email_html as layout_wrap, JobAttachment,
    ProviderSecrets, SendJob,
};
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

fn validate_attachments(
    cfg: &MailConfig,
    raw: &[EmailAttachment],
) -> Result<Vec<EmailAttachment>, SendEmailResult> {
    if raw.is_empty() {
        return Ok(vec![]);
    }
    if !cfg.features.attachments {
        return Err(SendEmailResult {
            ok: false,
            status: 400,
            message: cfg.i18n.err_need_body_or_attach.clone(),
            message_id: None,
        });
    }
    if raw.len() > MAX_ATTACHMENTS {
        return Err(SendEmailResult {
            ok: false,
            status: 400,
            message: cfg
                .i18n
                .fmt(&cfg.i18n.err_max_attach, "n", &MAX_ATTACHMENTS.to_string()),
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
                message: cfg.i18n.fmt(&cfg.i18n.err_empty_attach, "name", &name),
                message_id: None,
            });
        }
        let size = item.size.unwrap_or(((content.len() as u64) * 3) / 4);
        if size > MAX_ATTACHMENT_BYTES {
            return Err(SendEmailResult {
                ok: false,
                status: 400,
                message: cfg.i18n.fmt(&cfg.i18n.err_file_too_big, "name", &name),
                message_id: None,
            });
        }
        total += size;
        if total > MAX_TOTAL_ATTACHMENT_BYTES {
            return Err(SendEmailResult {
                ok: false,
                status: 400,
                message: cfg.i18n.err_total_too_big.clone(),
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

pub fn render_body_html(cfg: &MailConfig, body: &str, opts: &MarkdownOptions) -> String {
    if cfg.features.markdown {
        markdown::render_markdown(body, opts)
    } else {
        let escaped = escape_html(body);
        format!(
            "<p>{}</p>",
            escaped.replace("\r\n", "\n").replace('\n', "<br>")
        )
    }
}

pub fn wrap_email_html(
    cfg: &MailConfig,
    subject: &str,
    body_html: &str,
    from_name: &str,
    interactive: bool,
) -> String {
    let pal = resolve_theme(&cfg.plugins, &cfg.brand);
    layout_wrap(cfg, &pal, subject, body_html, from_name, interactive)
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
        cfg.i18n.body_empty_attach.as_str()
    } else {
        body_trim
    };
    let html = wrap_email_html(
        cfg,
        subject.trim(),
        &render_body_html(
            cfg,
            body_for_render,
            &MarkdownOptions::preview(crate::config::mail_origin(cfg)),
        ),
        &from,
        true,
    );
    (from, html, body_for_render.to_string())
}

pub async fn send_mail(
    cfg: &MailConfig,
    secrets: &ProviderSecrets,
    input: SendEmailInput,
) -> SendEmailResult {
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
            message: cfg.i18n.err_need_recipient.clone(),
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
            message: cfg.i18n.err_need_subject.clone(),
            message_id: None,
        };
    }

    let attachments = match validate_attachments(cfg, &input.attachments) {
        Ok(a) => a,
        Err(e) => return e,
    };

    let body = input.body.trim().to_string();
    if body.is_empty() && attachments.is_empty() {
        return SendEmailResult {
            ok: false,
            status: 400,
            message: cfg.i18n.err_need_body_or_attach.clone(),
            message_id: None,
        };
    }

    let from_name = resolve_from_name(cfg, input.from_name.as_deref());
    let body_for_render = if body.is_empty() && !attachments.is_empty() {
        cfg.i18n.body_empty_attach.clone()
    } else {
        body.clone()
    };
    let origin = crate::config::mail_origin(cfg);
    let html_content = if input.html {
        body_for_render.clone()
    } else {
        wrap_email_html(
            cfg,
            &subject,
            &render_body_html(cfg, &body_for_render, &MarkdownOptions::email(origin)),
            &from_name,
            false,
        )
    };
    let text_content = if input.html {
        strip_html_tags(&body_for_render)
    } else {
        body_for_render
    };

    let job = SendJob {
        from_email: cfg.mail.from_email.clone(),
        from_name,
        to,
        subject,
        html: html_content,
        text: text_content,
        tag: cfg.mail.tag.clone(),
        attachments: attachments
            .into_iter()
            .map(|a| JobAttachment {
                name: a.name,
                content: a.content,
            })
            .collect(),
        provider_domain: cfg.mail.provider_domain.clone(),
    };

    let result = send_via_provider(cfg, secrets, job).await;
    SendEmailResult {
        ok: result.ok,
        status: result.status,
        message: result.message,
        message_id: result.message_id,
    }
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
        let cfg = crate::config::load_config();
        let html = render_body_html(&cfg, "# Title\n\n**hello**", &MarkdownOptions::default());
        assert!(html.contains("<h1"));
        assert!(html.contains("<strong>hello</strong>"));
    }

    #[test]
    fn wrap_omits_unconfigured_footer() {
        let mut cfg = crate::config::load_config();
        cfg.layout.show_footer_contact = false;
        cfg.layout.show_footer_site = false;
        cfg.mail.contact_email.clear();
        cfg.site.url.clear();
        let html = wrap_email_html(&cfg, "Hi", "<p>x</p>", "From", false);
        assert!(html.contains("<p>x</p>"));
        assert!(!html.contains("mailto:"));
    }
}
