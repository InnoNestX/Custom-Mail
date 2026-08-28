mod brand;
mod config;
mod email;
mod history;
mod login_guard;
mod markdown;
mod plugins;
mod sessions;
mod ui;

use brand::favicon_svg;
use config::load_config;
use email::{
    build_email_preview_html, decode_snippet_param, fixed_from_email, resolve_from_name, send_mail,
    snippet_page_html, EmailAttachment, SendEmailInput,
};
use history::{append_send_log, get_send_log, list_send_logs, NewLog};
use login_guard::{
    check_login_allowed, clear_login_failures, format_lockout_message, get_client_ip,
    record_login_failure,
};
use plugins::{ProviderId, ProviderSecrets};
use sessions::{
    clear_session_cookie_header, create_session, read_session_token, revoke_session,
    session_cookie_header, validate_session,
};
use ui::render_app_html;
use worker::*;

fn json(value: serde_json::Value, status: u16) -> Result<Response> {
    let headers = Headers::new();
    headers.set("Content-Type", "application/json; charset=utf-8")?;
    headers.set("Cache-Control", "no-store")?;
    Ok(Response::from_json(&value)?
        .with_status(status)
        .with_headers(headers))
}

fn json_with_cookie(value: serde_json::Value, status: u16, cookie: &str) -> Result<Response> {
    let mut resp = json(value, status)?;
    resp.headers_mut().set("Set-Cookie", cookie)?;
    Ok(resp)
}

fn html(body: String, cache: &str) -> Result<Response> {
    let headers = Headers::new();
    headers.set("Content-Type", "text/html; charset=utf-8")?;
    headers.set("Cache-Control", cache)?;
    Ok(Response::from_html(body)?.with_headers(headers))
}

fn svg(body: String) -> Result<Response> {
    let headers = Headers::new();
    headers.set("Content-Type", "image/svg+xml; charset=utf-8")?;
    headers.set("Cache-Control", "public, max-age=604800")?;
    Ok(Response::from_bytes(body.into_bytes())?.with_headers(headers))
}

fn timing_safe_eq(a: &str, b: &str) -> bool {
    let ab = a.as_bytes();
    let bb = b.as_bytes();
    if ab.len() != bb.len() {
        let mut _diff = ab.len() ^ bb.len();
        let max = ab.len().max(bb.len());
        for i in 0..max {
            let x = ab.get(i).copied().unwrap_or(0);
            let y = bb.get(i).copied().unwrap_or(0);
            _diff |= (x ^ y) as usize;
        }
        return false;
    }
    let mut out = 0u8;
    for i in 0..ab.len() {
        out |= ab[i] ^ bb[i];
    }
    out == 0
}

fn secret_or_var(env: &Env, name: &str) -> String {
    env.secret(name)
        .map(|s| s.to_string())
        .or_else(|_| env.var(name).map(|v| v.to_string()))
        .unwrap_or_default()
}

fn provider_secrets(env: &Env, cfg: &config::MailConfig) -> ProviderSecrets {
    let id = ProviderId::parse(&cfg.plugins.provider);
    let mut api_key = String::new();
    for name in id.secret_names() {
        let v = secret_or_var(env, name);
        if !v.is_empty() {
            api_key = v;
            break;
        }
    }
    let extra_domain = if id == ProviderId::Mailgun {
        let d = secret_or_var(env, "MAILGUN_DOMAIN");
        if d.is_empty() {
            cfg.mail.provider_domain.clone()
        } else {
            d
        }
    } else {
        String::new()
    };
    ProviderSecrets {
        api_key,
        extra_domain,
    }
}

async fn require_auth(req: &Request, kv: &kv::KvStore) -> std::result::Result<(), String> {
    let token = read_session_token(req.headers().get("Cookie").ok().flatten().as_deref());
    let Some(token) = token else {
        return Err("Unauthorized".into());
    };
    if !validate_session(kv, &token).await? {
        return Err("Unauthorized".into());
    }
    Ok(())
}

#[event(fetch)]
async fn main(mut req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    let cfg = load_config();

    let allow_any = secret_or_var(&env, "ALLOW_ANY_HOST") == "1";
    if !allow_any {
        let host = req
            .headers()
            .get("Host")?
            .unwrap_or_default()
            .split(':')
            .next()
            .unwrap_or("")
            .to_lowercase();
        if host != cfg.host {
            return json(
                serde_json::json!({ "error": format!("This service is only available at {}", cfg.host) }),
                403,
            );
        }
    }

    let url = req.url()?;
    let path = url.path().to_string();
    let method = req.method();
    let from_name = resolve_from_name(&cfg, None);
    let from_email = fixed_from_email(&cfg);

    if method == Method::Get && path == "/favicon.svg" {
        return svg(favicon_svg(&cfg, 32));
    }
    if method == Method::Get && path == "/apple-touch-icon.svg" {
        return svg(favicon_svg(&cfg, 180));
    }

    if method == Method::Get && path == "/snippet" {
        let encoded = url
            .query_pairs()
            .find(|(k, _)| k == "e")
            .map(|(_, v)| v.into_owned());
        let Some(encoded) = encoded else {
            return Response::error("Missing snippet parameter.", 400);
        };
        return match decode_snippet_param(&encoded) {
            Ok(code) if code.len() <= 8000 => html(snippet_page_html(&code), "no-store"),
            Ok(_) => Response::error("Snippet too large.", 400),
            Err(_) => Response::error("Invalid snippet.", 400),
        };
    }

    if method == Method::Get && (path == "/" || path == "/index.html") {
        return html(render_app_html(&cfg, &from_name, &from_email), "no-store");
    }

    let kv = env.kv("MAIL_LOG_KV")?;
    let admin_password = secret_or_var(&env, "ADMIN_PASSWORD");
    let mail_secrets = provider_secrets(&env, &cfg);
    let provider_id = ProviderId::parse(&cfg.plugins.provider);

    if method == Method::Get && path == "/api/health" {
        return json(
            serde_json::json!({
                "ok": true,
                "service": "mail",
                "runtime": "rust",
                "from": format!("{from_name} <{from_email}>"),
                "provider": provider_id.as_str(),
                "theme": cfg.plugins.theme,
                "layout": cfg.plugins.layout,
                "configured": !mail_secrets.api_key.is_empty(),
                "history": cfg.features.history
            }),
            200,
        );
    }

    if method == Method::Post && path == "/api/login" {
        let body: serde_json::Value = req.json().await.unwrap_or(serde_json::json!({}));
        let password = body.get("password").and_then(|v| v.as_str()).unwrap_or("");
        if admin_password.is_empty() {
            return json(
                serde_json::json!({ "error": "ADMIN_PASSWORD is not configured" }),
                500,
            );
        }
        let ip = get_client_ip(
            req.headers().get("CF-Connecting-IP")?.as_deref(),
            req.headers().get("X-Forwarded-For")?.as_deref(),
        );
        let guard = check_login_allowed(&kv, &ip)
            .await
            .map_err(Error::RustError)?;
        if !guard.allowed {
            let retry = guard.retry_after_sec.unwrap_or(0);
            return json(
                serde_json::json!({
                    "error": format_lockout_message(retry),
                    "locked": true,
                    "retryAfterSec": retry,
                    "lockedUntil": guard.locked_until,
                }),
                429,
            );
        }
        if !timing_safe_eq(password, &admin_password) {
            let failure = record_login_failure(&kv, &ip)
                .await
                .map_err(Error::RustError)?;
            if !failure.allowed {
                let retry = failure.retry_after_sec.unwrap_or(0);
                return json(
                    serde_json::json!({
                        "error": format_lockout_message(retry),
                        "locked": true,
                        "retryAfterSec": retry,
                        "lockedUntil": failure.locked_until,
                    }),
                    429,
                );
            }
            let remaining = failure.attempts_remaining.unwrap_or(0);
            let hint = if remaining > 0 {
                format!("Incorrect password. {remaining} attempt(s) remaining.")
            } else {
                "Incorrect password.".into()
            };
            return json(
                serde_json::json!({ "error": hint, "attemptsRemaining": remaining }),
                401,
            );
        }
        clear_login_failures(&kv, &ip)
            .await
            .map_err(Error::RustError)?;
        let (token, expires_at) = create_session(&kv).await.map_err(Error::RustError)?;
        return json_with_cookie(
            serde_json::json!({ "ok": true, "expiresAt": expires_at }),
            200,
            &session_cookie_header(&token),
        );
    }

    if method == Method::Post && path == "/api/logout" {
        if let Some(token) = read_session_token(req.headers().get("Cookie")?.as_deref()) {
            let _ = revoke_session(&kv, &token).await;
        }
        return json_with_cookie(
            serde_json::json!({ "ok": true }),
            200,
            &clear_session_cookie_header(),
        );
    }

    if method == Method::Post && path == "/api/session" {
        if require_auth(&req, &kv).await.is_err() {
            return json(serde_json::json!({ "error": "Unauthorized" }), 401);
        }
        return json(
            serde_json::json!({
                "ok": true,
                "fromName": resolve_from_name(&cfg, None),
                "fromEmail": from_email,
                "addressBook": cfg.address_book,
            }),
            200,
        );
    }

    if method == Method::Post && path == "/api/history" {
        if require_auth(&req, &kv).await.is_err() {
            return json(serde_json::json!({ "error": "Unauthorized" }), 401);
        }
        if !cfg.features.history {
            return json(serde_json::json!({ "ok": true, "items": [] }), 200);
        }
        let body: serde_json::Value = req.json().await.unwrap_or(serde_json::json!({}));
        let limit = body.get("limit").and_then(|v| v.as_u64()).unwrap_or(50) as usize;
        let items = list_send_logs(&kv, limit).await.map_err(Error::RustError)?;
        return json(serde_json::json!({ "ok": true, "items": items }), 200);
    }

    if method == Method::Post && path == "/api/history/detail" {
        if require_auth(&req, &kv).await.is_err() {
            return json(serde_json::json!({ "error": "Unauthorized" }), 401);
        }
        let body: serde_json::Value = req.json().await.unwrap_or(serde_json::json!({}));
        let id = body.get("id").and_then(|v| v.as_str()).unwrap_or("").trim();
        if id.is_empty() {
            return json(serde_json::json!({ "error": cfg.i18n.missing_id }), 400);
        }
        return match get_send_log(&kv, id).await.map_err(Error::RustError)? {
            Some(entry) => json(serde_json::json!({ "ok": true, "entry": entry }), 200),
            None => json(serde_json::json!({ "error": cfg.i18n.missing_record }), 404),
        };
    }

    if method == Method::Post && path == "/api/preview" {
        if require_auth(&req, &kv).await.is_err() {
            return json(serde_json::json!({ "error": "Unauthorized" }), 401);
        }
        let body: serde_json::Value = req.json().await.unwrap_or(serde_json::json!({}));
        let to = parse_recipients(&body);
        let subject = body.get("subject").and_then(|v| v.as_str()).unwrap_or("");
        let text = body.get("body").and_then(|v| v.as_str()).unwrap_or("");
        let name_override = body.get("fromName").and_then(|v| v.as_str());
        let has_attachments = body
            .get("attachments")
            .and_then(|v| v.as_array())
            .map(|a| !a.is_empty())
            .unwrap_or(false);
        if to.is_empty() {
            return json(
                serde_json::json!({ "error": cfg.i18n.err_need_recipient }),
                400,
            );
        }
        if subject.trim().is_empty() {
            return json(
                serde_json::json!({ "error": cfg.i18n.err_need_subject }),
                400,
            );
        }
        if text.trim().is_empty() && !has_attachments {
            return json(
                serde_json::json!({ "error": cfg.i18n.err_need_body_or_attach }),
                400,
            );
        }
        let (pn, html_body, text_preview) =
            build_email_preview_html(&cfg, subject, text, name_override, has_attachments);
        return json(
            serde_json::json!({
                "ok": true,
                "fromName": pn,
                "fromEmail": from_email,
                "to": to,
                "subject": subject.trim(),
                "textPreview": text_preview,
                "html": html_body,
            }),
            200,
        );
    }

    if method == Method::Post && path == "/api/send" {
        if require_auth(&req, &kv).await.is_err() {
            return json(serde_json::json!({ "error": "Unauthorized" }), 401);
        }
        let body: serde_json::Value = req.json().await.unwrap_or(serde_json::json!({}));
        let to = parse_recipients(&body);
        let subject = body
            .get("subject")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let text = body
            .get("body")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let html_flag = body.get("html").and_then(|v| v.as_bool()).unwrap_or(false);
        let name_override = body
            .get("fromName")
            .and_then(|v| v.as_str())
            .map(str::to_string);
        let attachments = parse_attachments(body.get("attachments"));
        let resolved = resolve_from_name(&cfg, name_override.as_deref());
        let names: Vec<String> = attachments.iter().map(|a| a.name.clone()).collect();
        let sizes: Vec<u64> = attachments.iter().map(|a| a.size.unwrap_or(0)).collect();
        let result = send_mail(
            &cfg,
            &mail_secrets,
            SendEmailInput {
                to: to.clone(),
                subject: subject.clone(),
                body: text.clone(),
                from_name: name_override,
                html: html_flag,
                attachments: attachments.clone(),
            },
        )
        .await;
        if cfg.features.history {
            let _ = append_send_log(
                &kv,
                NewLog {
                    from_name: &resolved,
                    from_email: &from_email,
                    to: &to,
                    subject: &subject,
                    body: &text,
                    attachment_names: &names,
                    attachment_sizes: &sizes,
                    ok: result.ok,
                    message_id: result.message_id.as_deref(),
                    error: if result.ok {
                        None
                    } else {
                        Some(result.message.as_str())
                    },
                },
            )
            .await;
        }
        if !result.ok {
            let status = if result.status >= 400 {
                result.status
            } else {
                502
            };
            return json(serde_json::json!({ "error": result.message }), status);
        }
        return json(
            serde_json::json!({
                "ok": true,
                "messageId": result.message_id,
                "message": result.message
            }),
            200,
        );
    }

    json(serde_json::json!({ "error": "Not found" }), 404)
}

fn parse_recipients(body: &serde_json::Value) -> Vec<String> {
    match body.get("to") {
        Some(serde_json::Value::Array(arr)) => arr
            .iter()
            .filter_map(|v| v.as_str().map(str::to_string))
            .filter(|s| !s.is_empty())
            .collect(),
        Some(serde_json::Value::String(s)) => s
            .split(|c: char| c == ',' || c == ';' || c.is_whitespace())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect(),
        _ => vec![],
    }
}

fn parse_attachments(raw: Option<&serde_json::Value>) -> Vec<EmailAttachment> {
    let Some(serde_json::Value::Array(arr)) = raw else {
        return vec![];
    };
    let mut out = Vec::new();
    for item in arr {
        let name = item
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_string();
        let content = item
            .get("content")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_string();
        let size = item.get("size").and_then(|v| v.as_u64());
        if name.is_empty() || content.is_empty() {
            continue;
        }
        out.push(EmailAttachment {
            name,
            content,
            size,
        });
    }
    out
}
