use serde::{Deserialize, Serialize};
use worker::kv::KvStore;

use crate::sessions::random_hex;

const INDEX_KEY: &str = "log:index";
const MAX_ENTRIES: usize = 200;
const MAX_BODY_CHARS: usize = 12000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendLogEntry {
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "fromName")]
    pub from_name: String,
    #[serde(rename = "fromEmail")]
    pub from_email: String,
    pub to: Vec<String>,
    pub subject: String,
    pub body: String,
    #[serde(rename = "attachmentNames")]
    pub attachment_names: Vec<String>,
    #[serde(rename = "attachmentSizes")]
    pub attachment_sizes: Vec<u64>,
    pub ok: bool,
    #[serde(rename = "messageId", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendLogSummary {
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "fromName")]
    pub from_name: String,
    pub to: Vec<String>,
    pub subject: String,
    #[serde(rename = "bodyPreview")]
    pub body_preview: String,
    #[serde(rename = "attachmentNames")]
    pub attachment_names: Vec<String>,
    pub ok: bool,
    #[serde(rename = "messageId", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

fn preview_body(body: &str) -> String {
    let t = body.trim();
    if t.chars().count() <= 160 {
        t.to_string()
    } else {
        format!("{}…", t.chars().take(160).collect::<String>())
    }
}

async fn read_index(kv: &KvStore) -> Result<Vec<String>, String> {
    let Some(raw) = kv.get(INDEX_KEY).text().await.map_err(|e| format!("{e:?}"))? else {
        return Ok(vec![]);
    };
    Ok(serde_json::from_str(&raw).unwrap_or_default())
}

pub struct NewLog<'a> {
    pub from_name: &'a str,
    pub from_email: &'a str,
    pub to: &'a [String],
    pub subject: &'a str,
    pub body: &'a str,
    pub attachment_names: &'a [String],
    pub attachment_sizes: &'a [u64],
    pub ok: bool,
    pub message_id: Option<&'a str>,
    pub error: Option<&'a str>,
}

pub async fn append_send_log(kv: &KvStore, entry: NewLog<'_>) -> Result<SendLogEntry, String> {
    let body: String = entry.body.chars().take(MAX_BODY_CHARS).collect();
    let record = SendLogEntry {
        id: random_hex(12)?,
        created_at: String::from(js_sys::Date::new_0().to_iso_string()),
        from_name: entry.from_name.to_string(),
        from_email: entry.from_email.to_string(),
        to: entry.to.to_vec(),
        subject: entry.subject.to_string(),
        body,
        attachment_names: entry.attachment_names.to_vec(),
        attachment_sizes: entry.attachment_sizes.to_vec(),
        ok: entry.ok,
        message_id: entry.message_id.map(str::to_string),
        error: entry.error.map(str::to_string),
    };

    let mut ids = read_index(kv).await?;
    ids.insert(0, record.id.clone());
    let trimmed: Vec<String> = ids.iter().take(MAX_ENTRIES).cloned().collect();
    let prune: Vec<String> = ids.into_iter().skip(MAX_ENTRIES).collect();

    kv.put(
        &format!("log:{}", record.id),
        serde_json::to_string(&record).unwrap(),
    )
    .map_err(|e| format!("{e:?}"))?
    .execute()
    .await
    .map_err(|e| format!("{e:?}"))?;

    kv.put(INDEX_KEY, serde_json::to_string(&trimmed).unwrap())
        .map_err(|e| format!("{e:?}"))?
        .execute()
        .await
        .map_err(|e| format!("{e:?}"))?;

    for old in prune {
        let _ = kv.delete(&format!("log:{old}")).await;
    }

    Ok(record)
}

pub async fn list_send_logs(kv: &KvStore, limit: usize) -> Result<Vec<SendLogSummary>, String> {
    let ids = read_index(kv).await?;
    let take = ids.into_iter().take(limit.min(100));
    let mut out = Vec::new();
    for id in take {
        let Some(raw) = kv
            .get(&format!("log:{id}"))
            .text()
            .await
            .map_err(|e| format!("{e:?}"))?
        else {
            continue;
        };
        if let Ok(e) = serde_json::from_str::<SendLogEntry>(&raw) {
            out.push(SendLogSummary {
                id: e.id,
                created_at: e.created_at,
                from_name: e.from_name,
                to: e.to,
                subject: e.subject,
                body_preview: preview_body(&e.body),
                attachment_names: e.attachment_names,
                ok: e.ok,
                message_id: e.message_id,
                error: e.error,
            });
        }
    }
    Ok(out)
}

pub async fn get_send_log(kv: &KvStore, id: &str) -> Result<Option<SendLogEntry>, String> {
    if id.is_empty() {
        return Ok(None);
    }
    let Some(raw) = kv
        .get(&format!("log:{id}"))
        .text()
        .await
        .map_err(|e| format!("{e:?}"))?
    else {
        return Ok(None);
    };
    Ok(serde_json::from_str(&raw).ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_truncates() {
        let short = preview_body("hello");
        assert_eq!(short, "hello");
        let long = "a".repeat(200);
        let p = preview_body(&long);
        assert!(p.ends_with('…'));
        assert!(p.chars().count() <= 161);
    }
}
