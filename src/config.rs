use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressBookEntry {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MailConfig {
    pub host: String,
    pub app: AppConfig,
    pub mail: MailSettings,
    pub site: SiteConfig,
    pub brand: BrandConfig,
    #[serde(default, rename = "addressBook")]
    pub address_book: Vec<AddressBookEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "loginTagline")]
    pub login_tagline: String,
    #[serde(rename = "loginHeadlineBefore")]
    pub login_headline_before: String,
    #[serde(rename = "loginHeadlineEm")]
    pub login_headline_em: String,
    #[serde(rename = "loginLead")]
    pub login_lead: String,
    #[serde(rename = "loginPoints")]
    pub login_points: Vec<String>,
    #[serde(rename = "loginFormTitle")]
    pub login_form_title: String,
    #[serde(rename = "loginFormSub")]
    pub login_form_sub: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MailSettings {
    #[serde(rename = "fromEmail")]
    pub from_email: String,
    #[serde(rename = "fromNameDefault")]
    pub from_name_default: String,
    #[serde(rename = "contactEmail")]
    pub contact_email: String,
    #[serde(rename = "brevoTag")]
    pub brevo_tag: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SiteConfig {
    pub url: String,
    pub label: String,
    #[serde(rename = "brandName")]
    pub brand_name: String,
    #[serde(rename = "logoPath")]
    pub logo_path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BrandConfig {
    pub tile: String,
    #[serde(rename = "tileEdge")]
    pub tile_edge: String,
    pub accent: String,
    pub cream: String,
    #[serde(rename = "siteBlue")]
    pub site_blue: String,
}

static RAW: &str = include_str!("../config/mail.json");

pub fn load_config() -> MailConfig {
    let mut cfg: MailConfig =
        serde_json::from_str(RAW).expect("config/mail.json must be valid JSON");
    cfg.mail.from_email = cfg.mail.from_email.trim().to_lowercase();
    cfg.mail.contact_email = cfg.mail.contact_email.trim().to_lowercase();
    cfg.site.url = cfg.site.url.trim_end_matches('/').to_string();
    if !cfg.site.logo_path.starts_with('/') {
        cfg.site.logo_path = format!("/{}", cfg.site.logo_path);
    }
    cfg.address_book = cfg
        .address_book
        .into_iter()
        .filter_map(|mut e| {
            e.address = e.address.trim().to_lowercase();
            if !e.address.contains('@') {
                return None;
            }
            if let Some(n) = e.note.take() {
                let t = n.trim().to_string();
                e.note = if t.is_empty() { None } else { Some(t) };
            }
            Some(e)
        })
        .collect();
    cfg
}

pub fn mail_origin(cfg: &MailConfig) -> String {
    format!("https://{}", cfg.host)
}

pub fn mail_logo_url(cfg: &MailConfig) -> String {
    format!("{}{}", mail_origin(cfg), cfg.site.logo_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_mail_json() {
        let cfg = load_config();
        assert!(cfg.host.contains('.'));
        assert!(cfg.mail.from_email.contains('@'));
        assert!(!cfg.app.title.is_empty());
    }
}
