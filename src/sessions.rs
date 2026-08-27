use serde::{Deserialize, Serialize};
use worker::kv::KvStore;

pub const SESSION_COOKIE: &str = "xxm_session";
pub const SESSION_TTL_SEC: u64 = 60 * 60 * 24;
const SESSION_PREFIX: &str = "session:";
const REGISTRY_KEY: &str = "session:registry";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRecord {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
}

fn session_key(token: &str) -> String {
    format!("{SESSION_PREFIX}{token}")
}

pub fn random_hex(bytes: usize) -> Result<String, String> {
    let mut buf = vec![0u8; bytes];
    getrandom::getrandom(&mut buf).map_err(|e| format!("{e:?}"))?;
    Ok(hex::encode(buf))
}

fn parse_registry(raw: Option<String>) -> Vec<String> {
    let Some(raw) = raw else { return vec![] };
    serde_json::from_str::<Vec<String>>(&raw).unwrap_or_default()
}

pub fn read_session_token(cookie_header: Option<&str>) -> Option<String> {
    let cookie = cookie_header?;
    for part in cookie.split(';') {
        let trimmed = part.trim();
        let prefix = format!("{SESSION_COOKIE}=");
        if let Some(value) = trimmed.strip_prefix(&prefix) {
            let value = value.trim();
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }
    None
}

pub fn session_cookie_header(token: &str) -> String {
    format!(
        "{SESSION_COOKIE}={token}; Path=/; HttpOnly; Secure; SameSite=Strict; Max-Age={SESSION_TTL_SEC}"
    )
}

pub fn clear_session_cookie_header() -> String {
    format!("{SESSION_COOKIE}=; Path=/; HttpOnly; Secure; SameSite=Strict; Max-Age=0")
}

pub async fn revoke_all_sessions(kv: &KvStore) -> Result<(), String> {
    let tokens = parse_registry(kv.get(REGISTRY_KEY).text().await.map_err(|e| format!("{e:?}"))?);
    for token in tokens {
        kv.delete(&session_key(&token))
            .await
            .map_err(|e| format!("{e:?}"))?;
    }
    kv.delete(REGISTRY_KEY).await.map_err(|e| format!("{e:?}"))?;
    Ok(())
}

pub async fn revoke_session(kv: &KvStore, token: &str) -> Result<(), String> {
    if token.is_empty() {
        return Ok(());
    }
    kv.delete(&session_key(token))
        .await
        .map_err(|e| format!("{e:?}"))?;
    let tokens: Vec<String> = parse_registry(kv.get(REGISTRY_KEY).text().await.map_err(|e| format!("{e:?}"))?)
        .into_iter()
        .filter(|t| t != token)
        .collect();
    if tokens.is_empty() {
        kv.delete(REGISTRY_KEY).await.map_err(|e| format!("{e:?}"))?;
    } else {
        kv.put(REGISTRY_KEY, serde_json::to_string(&tokens).unwrap())
            .map_err(|e| format!("{e:?}"))?
            .execute()
            .await
            .map_err(|e| format!("{e:?}"))?;
    }
    Ok(())
}

pub async fn create_session(kv: &KvStore) -> Result<(String, String), String> {
    revoke_all_sessions(kv).await?;
    let token = random_hex(32)?;
    let now = js_sys_now_ms();
    let expires_at = iso_from_ms(now + (SESSION_TTL_SEC as f64) * 1000.0);
    let record = SessionRecord {
        created_at: iso_from_ms(now),
        expires_at: expires_at.clone(),
    };
    kv.put(
        &session_key(&token),
        serde_json::to_string(&record).unwrap(),
    )
    .map_err(|e| format!("{e:?}"))?
    .expiration_ttl(SESSION_TTL_SEC)
    .execute()
    .await
    .map_err(|e| format!("{e:?}"))?;
    kv.put(REGISTRY_KEY, serde_json::to_string(&vec![&token]).unwrap())
        .map_err(|e| format!("{e:?}"))?
        .expiration_ttl(SESSION_TTL_SEC)
        .execute()
        .await
        .map_err(|e| format!("{e:?}"))?;
    Ok((token, expires_at))
}

pub async fn validate_session(kv: &KvStore, token: &str) -> Result<bool, String> {
    if token.is_empty() {
        return Ok(false);
    }
    let Some(raw) = kv
        .get(&session_key(token))
        .text()
        .await
        .map_err(|e| format!("{e:?}"))?
    else {
        return Ok(false);
    };
    let Ok(record) = serde_json::from_str::<SessionRecord>(&raw) else {
        let _ = revoke_session(kv, token).await;
        return Ok(false);
    };
    let expires = js_sys::Date::parse(&record.expires_at);
    if expires.is_nan() || expires <= js_sys_now_ms() {
        let _ = revoke_session(kv, token).await;
        return Ok(false);
    }
    Ok(true)
}

fn js_sys_now_ms() -> f64 {
    js_sys::Date::now()
}

fn iso_from_ms(ms: f64) -> String {
    String::from(
        js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(ms)).to_iso_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cookie_roundtrip_parse() {
        let header = session_cookie_header("abc123");
        assert!(header.contains("xxm_session=abc123"));
        let token = read_session_token(Some("foo=1; xxm_session=abc123; bar=2"));
        assert_eq!(token.as_deref(), Some("abc123"));
    }
}
