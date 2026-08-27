use sha2::{Digest, Sha256};
use worker::kv::KvStore;

pub const MAX_LOGIN_ATTEMPTS: u32 = 3;
pub const LOGIN_LOCKOUT_SEC: u64 = 15 * 60;
const FAILURE_TTL_SEC: u64 = 60 * 60;
const FAIL_PREFIX: &str = "login:fail:";

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
struct LoginGuardRecord {
    failures: u32,
    #[serde(rename = "lockedUntil", skip_serializing_if = "Option::is_none")]
    locked_until: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LoginGuardStatus {
    pub allowed: bool,
    pub retry_after_sec: Option<u64>,
    pub locked_until: Option<String>,
    pub attempts_remaining: Option<u32>,
}

pub fn get_client_ip(cf_ip: Option<&str>, forwarded: Option<&str>) -> String {
    if let Some(ip) = cf_ip.map(str::trim).filter(|s| !s.is_empty()) {
        return ip.to_string();
    }
    if let Some(fwd) = forwarded {
        if let Some(first) = fwd.split(',').next().map(str::trim).filter(|s| !s.is_empty()) {
            return first.to_string();
        }
    }
    "unknown".to_string()
}

fn sha256_prefix(value: &str, len: usize) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    let hex = hex::encode(hasher.finalize());
    hex.chars().take(len).collect()
}

async fn guard_key(kv_request_ip: &str) -> String {
    let hash = sha256_prefix(&format!("login-guard:{kv_request_ip}"), 24);
    format!("{FAIL_PREFIX}{hash}")
}

fn locked_retry_sec(locked_until: &str) -> u64 {
    let ms = js_sys::Date::parse(locked_until) - js_sys::Date::now();
    if ms.is_nan() {
        return 1;
    }
    ((ms / 1000.0).ceil() as i64).max(1) as u64
}

fn status_from_record(record: &LoginGuardRecord) -> LoginGuardStatus {
    if let Some(ref locked_until) = record.locked_until {
        let retry = locked_retry_sec(locked_until);
        if retry > 0 && js_sys::Date::parse(locked_until) > js_sys::Date::now() {
            return LoginGuardStatus {
                allowed: false,
                retry_after_sec: Some(retry),
                locked_until: Some(locked_until.clone()),
                attempts_remaining: Some(0),
            };
        }
    }
    let remaining = MAX_LOGIN_ATTEMPTS.saturating_sub(record.failures);
    LoginGuardStatus {
        allowed: true,
        retry_after_sec: None,
        locked_until: None,
        attempts_remaining: Some(remaining),
    }
}

fn parse_record(raw: Option<String>) -> LoginGuardRecord {
    let Some(raw) = raw else {
        return LoginGuardRecord::default();
    };
    serde_json::from_str(&raw).unwrap_or_default()
}

pub async fn check_login_allowed(kv: &KvStore, ip: &str) -> Result<LoginGuardStatus, String> {
    let key = guard_key(ip).await;
    let record = parse_record(kv.get(&key).text().await.map_err(|e| format!("{e:?}"))?);
    Ok(status_from_record(&record))
}

pub async fn record_login_failure(kv: &KvStore, ip: &str) -> Result<LoginGuardStatus, String> {
    let key = guard_key(ip).await;
    let mut record = parse_record(kv.get(&key).text().await.map_err(|e| format!("{e:?}"))?);

    if let Some(ref locked) = record.locked_until {
        if locked_retry_sec(locked) > 0 && js_sys::Date::parse(locked) > js_sys::Date::now() {
            return Ok(status_from_record(&record));
        }
    }

    let failures = if record.locked_until.is_some() {
        1
    } else {
        record.failures + 1
    };
    record.failures = failures;
    record.locked_until = None;

    if failures >= MAX_LOGIN_ATTEMPTS {
        let locked_until = String::from(
            js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(
                js_sys::Date::now() + (LOGIN_LOCKOUT_SEC as f64) * 1000.0,
            ))
            .to_iso_string(),
        );
        record.locked_until = Some(locked_until.clone());
        record.failures = MAX_LOGIN_ATTEMPTS;
        kv.put(&key, serde_json::to_string(&record).unwrap())
            .map_err(|e| format!("{e:?}"))?
            .expiration_ttl(LOGIN_LOCKOUT_SEC + 300)
            .execute()
            .await
            .map_err(|e| format!("{e:?}"))?;
        return Ok(LoginGuardStatus {
            allowed: false,
            retry_after_sec: Some(LOGIN_LOCKOUT_SEC),
            locked_until: Some(locked_until),
            attempts_remaining: Some(0),
        });
    }

    kv.put(&key, serde_json::to_string(&record).unwrap())
        .map_err(|e| format!("{e:?}"))?
        .expiration_ttl(FAILURE_TTL_SEC)
        .execute()
        .await
        .map_err(|e| format!("{e:?}"))?;

    Ok(LoginGuardStatus {
        allowed: true,
        retry_after_sec: None,
        locked_until: None,
        attempts_remaining: Some(MAX_LOGIN_ATTEMPTS.saturating_sub(failures)),
    })
}

pub async fn clear_login_failures(kv: &KvStore, ip: &str) -> Result<(), String> {
    let key = guard_key(ip).await;
    kv.delete(&key).await.map_err(|e| format!("{e:?}"))
}

pub fn format_lockout_message(retry_after_sec: u64) -> String {
    let minutes = ((retry_after_sec as f64) / 60.0).ceil() as u64;
    if minutes <= 1 {
        "Too many failed attempts. Try again in about 1 minute.".into()
    } else {
        format!("Too many failed attempts. Try again in about {minutes} minutes.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_ip_prefers_cf() {
        assert_eq!(
            get_client_ip(Some("1.2.3.4"), Some("9.9.9.9, 8.8.8.8")),
            "1.2.3.4"
        );
        assert_eq!(get_client_ip(None, Some("9.9.9.9, 8.8.8.8")), "9.9.9.9");
    }

    #[test]
    fn lockout_copy() {
        assert!(format_lockout_message(30).contains("1 minute"));
        assert!(format_lockout_message(120).contains("2 minutes"));
    }
}
