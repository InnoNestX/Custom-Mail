use crate::config::MailConfig;

#[derive(Clone, Debug)]
pub struct JobAttachment {
    pub name: String,
    pub content: String,
}

#[derive(Clone, Debug)]
pub struct ProviderSendResult {
    pub ok: bool,
    pub status: u16,
    pub message: String,
    pub message_id: Option<String>,
}

/// Built-in HTTP mail APIs. Operators pick one with `plugins.provider`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProviderId {
    Brevo,
    Resend,
    SendGrid,
    Mailgun,
    Postmark,
    MailerSend,
    Smtp2Go,
    SparkPost,
}

impl ProviderId {
    pub fn parse(raw: &str) -> Self {
        match raw.trim().to_ascii_lowercase().as_str() {
            "resend" => Self::Resend,
            "sendgrid" => Self::SendGrid,
            "mailgun" => Self::Mailgun,
            "postmark" => Self::Postmark,
            "mailersend" | "mailer-send" => Self::MailerSend,
            "smtp2go" | "smtp-2go" => Self::Smtp2Go,
            "sparkpost" | "spark-post" => Self::SparkPost,
            _ => Self::Brevo,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Brevo => "brevo",
            Self::Resend => "resend",
            Self::SendGrid => "sendgrid",
            Self::Mailgun => "mailgun",
            Self::Postmark => "postmark",
            Self::MailerSend => "mailersend",
            Self::Smtp2Go => "smtp2go",
            Self::SparkPost => "sparkpost",
        }
    }

    pub fn secret_names(self) -> &'static [&'static str] {
        match self {
            Self::Brevo => &["BREVO_API_KEY", "MAIL_API_KEY"],
            Self::Resend => &["RESEND_API_KEY", "MAIL_API_KEY"],
            Self::SendGrid => &["SENDGRID_API_KEY", "MAIL_API_KEY"],
            Self::Mailgun => &["MAILGUN_API_KEY", "MAIL_API_KEY"],
            Self::Postmark => &["POSTMARK_SERVER_TOKEN", "MAIL_API_KEY"],
            Self::MailerSend => &["MAILERSEND_API_KEY", "MAIL_API_KEY"],
            Self::Smtp2Go => &["SMTP2GO_API_KEY", "MAIL_API_KEY"],
            Self::SparkPost => &["SPARKPOST_API_KEY", "MAIL_API_KEY"],
        }
    }
}

pub fn provider_env_hint(id: ProviderId) -> String {
    id.secret_names()[0].to_string()
}

pub struct ProviderSecrets {
    pub api_key: String,
    pub extra_domain: String,
}

pub struct SendJob {
    pub from_email: String,
    pub from_name: String,
    pub to: Vec<String>,
    pub subject: String,
    pub html: String,
    pub text: String,
    pub tag: String,
    pub attachments: Vec<JobAttachment>,
    pub provider_domain: String,
}

pub async fn send_via_provider(
    cfg: &MailConfig,
    secrets: &ProviderSecrets,
    job: SendJob,
) -> ProviderSendResult {
    if secrets.api_key.is_empty() {
        let hint = provider_env_hint(ProviderId::parse(&cfg.plugins.provider));
        return ProviderSendResult {
            ok: false,
            status: 500,
            message: format!("{hint} is not configured"),
            message_id: None,
        };
    }
    let id = ProviderId::parse(&cfg.plugins.provider);
    match id {
        ProviderId::Brevo => send_brevo(secrets, &job).await,
        ProviderId::Resend => send_resend(secrets, &job).await,
        ProviderId::SendGrid => send_sendgrid(secrets, &job).await,
        ProviderId::Mailgun => send_mailgun(secrets, &job).await,
        ProviderId::Postmark => send_postmark(secrets, &job).await,
        ProviderId::MailerSend => send_mailersend(secrets, &job).await,
        ProviderId::Smtp2Go => send_smtp2go(secrets, &job).await,
        ProviderId::SparkPost => send_sparkpost(secrets, &job).await,
    }
}

fn from_header(job: &SendJob) -> String {
    if job.from_name.trim().is_empty() {
        job.from_email.clone()
    } else {
        format!("{} <{}>", job.from_name.trim(), job.from_email)
    }
}

fn json_ok_id(status: u16, raw: &str, keys: &[&str]) -> ProviderSendResult {
    let parsed: Option<serde_json::Value> = serde_json::from_str(raw).ok();
    let message_id = parsed.as_ref().and_then(|p| first_id(p, keys));
    if (200..300).contains(&status) {
        ProviderSendResult {
            ok: true,
            status,
            message: "Delivered".into(),
            message_id,
        }
    } else {
        let err = parsed
            .as_ref()
            .and_then(extract_error)
            .unwrap_or_else(|| raw.chars().take(300).collect());
        ProviderSendResult {
            ok: false,
            status,
            message: if err.is_empty() {
                "Provider request failed".into()
            } else {
                err
            },
            message_id: None,
        }
    }
}

fn first_id(value: &serde_json::Value, keys: &[&str]) -> Option<String> {
    for key in keys {
        if let Some(s) = value.get(*key).and_then(|v| v.as_str()) {
            if !s.is_empty() {
                return Some(s.to_string());
            }
        }
    }
    if let Some(arr) = value.get("messageIds").and_then(|v| v.as_array()) {
        if let Some(s) = arr.first().and_then(|v| v.as_str()) {
            return Some(s.to_string());
        }
    }
    if let Some(data) = value.get("data") {
        for key in ["id", "email_id", "message_id"] {
            if let Some(s) = data.get(key).and_then(|v| v.as_str()) {
                if !s.is_empty() {
                    return Some(s.to_string());
                }
            }
        }
    }
    if let Some(results) = value.get("results") {
        if let Some(s) = results.get("id").and_then(|v| v.as_str()) {
            return Some(s.to_string());
        }
    }
    None
}

fn extract_error(value: &serde_json::Value) -> Option<String> {
    for key in ["message", "msg", "error", "Message", "ErrorCode"] {
        if let Some(s) = value.get(key).and_then(|v| v.as_str()) {
            if !s.is_empty() {
                return Some(s.to_string());
            }
        }
    }
    if let Some(errs) = value.get("errors").and_then(|v| v.as_array()) {
        if let Some(first) = errs.first() {
            if let Some(s) = first.get("message").and_then(|v| v.as_str()) {
                return Some(s.to_string());
            }
            if let Some(s) = first.as_str() {
                return Some(s.to_string());
            }
        }
    }
    None
}

async fn send_brevo(secrets: &ProviderSecrets, job: &SendJob) -> ProviderSendResult {
    let mut payload = serde_json::json!({
        "sender": { "email": job.from_email, "name": job.from_name },
        "to": job.to.iter().map(|email| serde_json::json!({ "email": email })).collect::<Vec<_>>(),
        "subject": job.subject,
        "htmlContent": job.html,
        "textContent": job.text,
    });
    if !job.tag.is_empty() {
        payload["tags"] = serde_json::json!([job.tag]);
    }
    if !job.attachments.is_empty() {
        payload["attachment"] = serde_json::json!(job
            .attachments
            .iter()
            .map(|a| serde_json::json!({ "name": a.name, "content": a.content }))
            .collect::<Vec<_>>());
    }
    match http_json(
        "https://api.brevo.com/v3/smtp/email",
        &[("api-key", secrets.api_key.as_str())],
        &payload,
    )
    .await
    {
        Ok((status, raw)) => json_ok_id(status, &raw, &["messageId"]),
        Err(e) => fail(e),
    }
}

async fn send_resend(secrets: &ProviderSecrets, job: &SendJob) -> ProviderSendResult {
    let mut payload = serde_json::json!({
        "from": from_header(job),
        "to": job.to,
        "subject": job.subject,
        "html": job.html,
        "text": job.text,
    });
    if !job.tag.is_empty() {
        payload["tags"] = serde_json::json!([{ "name": "campaign", "value": job.tag }]);
    }
    if !job.attachments.is_empty() {
        payload["attachments"] = serde_json::json!(job
            .attachments
            .iter()
            .map(|a| serde_json::json!({ "filename": a.name, "content": a.content }))
            .collect::<Vec<_>>());
    }
    match http_json(
        "https://api.resend.com/emails",
        &[("Authorization", &format!("Bearer {}", secrets.api_key))],
        &payload,
    )
    .await
    {
        Ok((status, raw)) => json_ok_id(status, &raw, &["id"]),
        Err(e) => fail(e),
    }
}

async fn send_sendgrid(secrets: &ProviderSecrets, job: &SendJob) -> ProviderSendResult {
    let mut payload = serde_json::json!({
        "personalizations": [{ "to": job.to.iter().map(|e| serde_json::json!({ "email": e })).collect::<Vec<_>>() }],
        "from": { "email": job.from_email, "name": job.from_name },
        "subject": job.subject,
        "content": [
            { "type": "text/plain", "value": job.text },
            { "type": "text/html", "value": job.html }
        ]
    });
    if !job.tag.is_empty() {
        payload["categories"] = serde_json::json!([job.tag]);
    }
    if !job.attachments.is_empty() {
        payload["attachments"] = serde_json::json!(job
            .attachments
            .iter()
            .map(|a| serde_json::json!({
                "content": a.content,
                "filename": a.name,
                "disposition": "attachment"
            }))
            .collect::<Vec<_>>());
    }
    match http_json(
        "https://api.sendgrid.com/v3/mail/send",
        &[("Authorization", &format!("Bearer {}", secrets.api_key))],
        &payload,
    )
    .await
    {
        Ok((status, raw)) => json_ok_id(status, &raw, &["id"]),
        Err(e) => fail(e),
    }
}

async fn send_postmark(secrets: &ProviderSecrets, job: &SendJob) -> ProviderSendResult {
    let mut payload = serde_json::json!({
        "From": from_header(job),
        "To": job.to.join(","),
        "Subject": job.subject,
        "HtmlBody": job.html,
        "TextBody": job.text,
        "MessageStream": "outbound"
    });
    if !job.tag.is_empty() {
        payload["Tag"] = serde_json::json!(job.tag);
    }
    if !job.attachments.is_empty() {
        payload["Attachments"] = serde_json::json!(job
            .attachments
            .iter()
            .map(|a| serde_json::json!({
                "Name": a.name,
                "Content": a.content,
                "ContentType": "application/octet-stream"
            }))
            .collect::<Vec<_>>());
    }
    match http_json(
        "https://api.postmarkapp.com/email",
        &[
            ("X-Postmark-Server-Token", secrets.api_key.as_str()),
            ("Accept", "application/json"),
        ],
        &payload,
    )
    .await
    {
        Ok((status, raw)) => json_ok_id(status, &raw, &["MessageID", "MessageId"]),
        Err(e) => fail(e),
    }
}

async fn send_mailersend(secrets: &ProviderSecrets, job: &SendJob) -> ProviderSendResult {
    let mut payload = serde_json::json!({
        "from": { "email": job.from_email, "name": job.from_name },
        "to": job.to.iter().map(|e| serde_json::json!({ "email": e })).collect::<Vec<_>>(),
        "subject": job.subject,
        "html": job.html,
        "text": job.text
    });
    if !job.tag.is_empty() {
        payload["tags"] = serde_json::json!([job.tag]);
    }
    if !job.attachments.is_empty() {
        payload["attachments"] = serde_json::json!(job
            .attachments
            .iter()
            .map(|a| serde_json::json!({
                "content": a.content,
                "filename": a.name,
                "disposition": "attachment"
            }))
            .collect::<Vec<_>>());
    }
    match http_json(
        "https://api.mailersend.com/v1/email",
        &[("Authorization", &format!("Bearer {}", secrets.api_key))],
        &payload,
    )
    .await
    {
        Ok((status, raw)) => json_ok_id(status, &raw, &["message_id", "id"]),
        Err(e) => fail(e),
    }
}

async fn send_smtp2go(secrets: &ProviderSecrets, job: &SendJob) -> ProviderSendResult {
    let mut payload = serde_json::json!({
        "sender": from_header(job),
        "to": job.to,
        "subject": job.subject,
        "html_body": job.html,
        "text_body": job.text
    });
    if !job.attachments.is_empty() {
        payload["attachments"] = serde_json::json!(job
            .attachments
            .iter()
            .map(|a| serde_json::json!({
                "filename": a.name,
                "fileblob": a.content,
                "mimetype": "application/octet-stream"
            }))
            .collect::<Vec<_>>());
    }
    match http_json(
        "https://api.smtp2go.com/v3/email/send",
        &[("X-Smtp2go-Api-Key", secrets.api_key.as_str())],
        &payload,
    )
    .await
    {
        Ok((status, raw)) => json_ok_id(status, &raw, &["id", "email_id"]),
        Err(e) => fail(e),
    }
}

async fn send_sparkpost(secrets: &ProviderSecrets, job: &SendJob) -> ProviderSendResult {
    let mut content = serde_json::json!({
        "from": from_header(job),
        "subject": job.subject,
        "html": job.html,
        "text": job.text
    });
    if !job.attachments.is_empty() {
        content["attachments"] = serde_json::json!(job
            .attachments
            .iter()
            .map(|a| serde_json::json!({
                "name": a.name,
                "type": "application/octet-stream",
                "data": a.content
            }))
            .collect::<Vec<_>>());
    }
    let payload = serde_json::json!({
        "content": content,
        "recipients": job.to.iter().map(|e| serde_json::json!({ "address": { "email": e } })).collect::<Vec<_>>(),
        "options": { "sandbox": false }
    });
    match http_json(
        "https://api.sparkpost.com/api/v1/transmissions",
        &[("Authorization", secrets.api_key.as_str())],
        &payload,
    )
    .await
    {
        Ok((status, raw)) => json_ok_id(status, &raw, &["id"]),
        Err(e) => fail(e),
    }
}

async fn send_mailgun(secrets: &ProviderSecrets, job: &SendJob) -> ProviderSendResult {
    let domain = if !secrets.extra_domain.trim().is_empty() {
        secrets.extra_domain.trim().to_string()
    } else {
        job.provider_domain.trim().to_string()
    };
    if domain.is_empty() {
        return fail("Mailgun requires mail.providerDomain or MAILGUN_DOMAIN".into());
    }
    let url = format!("https://api.mailgun.net/v3/{domain}/messages");
    let auth = format!("Basic {}", b64(&format!("api:{}", secrets.api_key)));
    let (body, content_type) = mailgun_multipart(job);
    match http_bytes(
        &url,
        &[("Authorization", auth.as_str())],
        &content_type,
        body,
    )
    .await
    {
        Ok((status, raw)) => json_ok_id(status, &raw, &["id"]),
        Err(e) => fail(e),
    }
}

fn mailgun_multipart(job: &SendJob) -> (Vec<u8>, String) {
    let boundary = "----CustomMailBoundary7a3f";
    let mut body = Vec::new();
    push_field(&mut body, boundary, "from", &from_header(job));
    for to in &job.to {
        push_field(&mut body, boundary, "to", to);
    }
    push_field(&mut body, boundary, "subject", &job.subject);
    push_field(&mut body, boundary, "html", &job.html);
    push_field(&mut body, boundary, "text", &job.text);
    for att in &job.attachments {
        let bytes = decode_b64(&att.content).unwrap_or_default();
        push_file(&mut body, boundary, "attachment", &att.name, &bytes);
    }
    body.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
    (body, format!("multipart/form-data; boundary={boundary}"))
}

fn push_field(body: &mut Vec<u8>, boundary: &str, name: &str, value: &str) {
    body.extend_from_slice(
        format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"{name}\"\r\n\r\n{value}\r\n"
        )
        .as_bytes(),
    );
}

fn push_file(body: &mut Vec<u8>, boundary: &str, name: &str, filename: &str, bytes: &[u8]) {
    body.extend_from_slice(
        format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"{name}\"; filename=\"{filename}\"\r\nContent-Type: application/octet-stream\r\n\r\n"
        )
        .as_bytes(),
    );
    body.extend_from_slice(bytes);
    body.extend_from_slice(b"\r\n");
}

fn b64(s: &str) -> String {
    encode_b64(s.as_bytes())
}

fn encode_b64(bytes: &[u8]) -> String {
    const TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    let mut i = 0;
    while i < bytes.len() {
        let b0 = bytes[i];
        let b1 = if i + 1 < bytes.len() { bytes[i + 1] } else { 0 };
        let b2 = if i + 2 < bytes.len() { bytes[i + 2] } else { 0 };
        out.push(TABLE[(b0 >> 2) as usize] as char);
        out.push(TABLE[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        if i + 1 < bytes.len() {
            out.push(TABLE[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            out.push('=');
        }
        if i + 2 < bytes.len() {
            out.push(TABLE[(b2 & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }
        i += 3;
    }
    out
}

fn decode_b64(input: &str) -> Option<Vec<u8>> {
    fn val(c: u8) -> Option<u8> {
        match c {
            b'A'..=b'Z' => Some(c - b'A'),
            b'a'..=b'z' => Some(c - b'a' + 26),
            b'0'..=b'9' => Some(c - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            b'=' => Some(0),
            _ => None,
        }
    }
    let clean: Vec<u8> = input.bytes().filter(|c| !c.is_ascii_whitespace()).collect();
    if clean.len() % 4 != 0 {
        return None;
    }
    let mut out = Vec::with_capacity(clean.len() / 4 * 3);
    for chunk in clean.chunks(4) {
        let a = val(chunk[0])?;
        let b = val(chunk[1])?;
        let c = val(chunk[2])?;
        let d = val(chunk[3])?;
        out.push((a << 2) | (b >> 4));
        if chunk[2] != b'=' {
            out.push((b << 4) | (c >> 2));
        }
        if chunk[3] != b'=' {
            out.push((c << 6) | d);
        }
    }
    Some(out)
}

fn fail(message: String) -> ProviderSendResult {
    ProviderSendResult {
        ok: false,
        status: 502,
        message,
        message_id: None,
    }
}

async fn http_json(
    url: &str,
    headers: &[(&str, &str)],
    payload: &serde_json::Value,
) -> Result<(u16, String), String> {
    use worker::{Fetch, Headers, Method, Request, RequestInit};
    let h = Headers::new();
    h.set("Content-Type", "application/json")
        .map_err(|e| e.to_string())?;
    for (k, v) in headers {
        h.set(k, v).map_err(|e| e.to_string())?;
    }
    let mut init = RequestInit::new();
    init.with_method(Method::Post)
        .with_headers(h)
        .with_body(Some(wasm_bindgen::JsValue::from_str(
            &serde_json::to_string(payload).map_err(|e| e.to_string())?,
        )));
    let req = Request::new_with_init(url, &init).map_err(|e| e.to_string())?;
    let mut resp = Fetch::Request(req)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = resp.status_code();
    let text = resp.text().await.map_err(|e| e.to_string())?;
    Ok((status, text))
}

async fn http_bytes(
    url: &str,
    headers: &[(&str, &str)],
    content_type: &str,
    body: Vec<u8>,
) -> Result<(u16, String), String> {
    use worker::{Fetch, Headers, Method, Request, RequestInit};
    let h = Headers::new();
    h.set("Content-Type", content_type)
        .map_err(|e| e.to_string())?;
    for (k, v) in headers {
        h.set(k, v).map_err(|e| e.to_string())?;
    }
    let mut init = RequestInit::new();
    let js_body = js_sys::Uint8Array::from(body.as_slice());
    init.with_method(Method::Post)
        .with_headers(h)
        .with_body(Some(js_body.into()));
    let req = Request::new_with_init(url, &init).map_err(|e| e.to_string())?;
    let mut resp = Fetch::Request(req)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = resp.status_code();
    let text = resp.text().await.map_err(|e| e.to_string())?;
    Ok((status, text))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_provider_is_brevo() {
        assert_eq!(ProviderId::parse("nope"), ProviderId::Brevo);
        assert_eq!(ProviderId::parse("SendGrid"), ProviderId::SendGrid);
        assert_eq!(ProviderId::parse("mailer-send"), ProviderId::MailerSend);
    }

    #[test]
    fn from_header_formats_name() {
        let job = SendJob {
            from_email: "a@b.test".into(),
            from_name: "Desk".into(),
            to: vec![],
            subject: String::new(),
            html: String::new(),
            text: String::new(),
            tag: String::new(),
            attachments: vec![],
            provider_domain: String::new(),
        };
        assert_eq!(from_header(&job), "Desk <a@b.test>");
    }

    #[test]
    fn b64_roundtrip() {
        let src = b"hello-mail";
        let enc = encode_b64(src);
        assert_eq!(decode_b64(&enc).unwrap(), src);
    }

    #[test]
    fn mailgun_multipart_includes_fields() {
        let job = SendJob {
            from_email: "a@b.test".into(),
            from_name: "Desk".into(),
            to: vec!["c@d.test".into()],
            subject: "Hi".into(),
            html: "<p>x</p>".into(),
            text: "x".into(),
            tag: String::new(),
            attachments: vec![],
            provider_domain: "mg.example.com".into(),
        };
        let (body, ct) = mailgun_multipart(&job);
        let s = String::from_utf8(body).unwrap();
        assert!(ct.contains("multipart/form-data"));
        assert!(s.contains("name=\"from\""));
        assert!(s.contains("Desk <a@b.test>"));
        assert!(s.contains("c@d.test"));
    }
}
