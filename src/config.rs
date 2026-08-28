//! Deploy-time product configuration. Empty optional fields disable that chrome.

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
    #[serde(default)]
    pub plugins: PluginsConfig,
    #[serde(default)]
    pub features: FeaturesConfig,
    #[serde(default)]
    pub layout: LayoutFlags,
    pub app: AppConfig,
    pub mail: MailSettings,
    #[serde(default)]
    pub site: SiteConfig,
    #[serde(default)]
    pub brand: BrandOverrides,
    #[serde(default)]
    pub i18n: I18nConfig,
    #[serde(default)]
    pub syntax: SyntaxConfig,
    #[serde(default, rename = "addressBook")]
    pub address_book: Vec<AddressBookEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PluginsConfig {
    /// Email API plugin: brevo, resend, sendgrid, mailgun, postmark, mailersend, smtp2go, sparkpost.
    #[serde(default = "default_provider")]
    pub provider: String,
    /// Visual palette plugin: forest, midnight, ocean, paper, rose, slate, aurora, sunset, nord, …
    #[serde(default = "default_theme")]
    pub theme: String,
    /// Outbound HTML layout plugin: card, minimal, banner, digest, compact, …
    #[serde(default = "default_layout")]
    pub layout: String,
    /// Brand mark plugin: auto, image, monogram, none.
    #[serde(default = "default_logo")]
    pub logo: String,
}

fn default_provider() -> String {
    "brevo".into()
}
fn default_theme() -> String {
    "forest".into()
}
fn default_layout() -> String {
    "card".into()
}
fn default_logo() -> String {
    "auto".into()
}

impl Default for PluginsConfig {
    fn default() -> Self {
        Self {
            provider: default_provider(),
            theme: default_theme(),
            layout: default_layout(),
            logo: default_logo(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeaturesConfig {
    #[serde(default = "default_true")]
    pub attachments: bool,
    #[serde(default = "default_true")]
    pub history: bool,
    #[serde(default = "default_true", rename = "addressBook")]
    pub address_book: bool,
    #[serde(default = "default_true")]
    pub markdown: bool,
    #[serde(default = "default_true", rename = "syntaxHelp")]
    pub syntax_help: bool,
}

fn default_true() -> bool {
    true
}

impl Default for FeaturesConfig {
    fn default() -> Self {
        Self {
            attachments: true,
            history: true,
            address_book: true,
            markdown: true,
            syntax_help: true,
        }
    }
}

/// Optional chrome in the composed HTML email. Missing/false omits that block.
#[derive(Debug, Clone, Deserialize)]
pub struct LayoutFlags {
    #[serde(default = "default_true", rename = "showHeader")]
    pub show_header: bool,
    #[serde(default = "default_true", rename = "showLogo")]
    pub show_logo: bool,
    #[serde(default = "default_true", rename = "showSubject")]
    pub show_subject: bool,
    #[serde(default = "default_true", rename = "showFrom")]
    pub show_from: bool,
    #[serde(default = "default_true", rename = "showFooterContact")]
    pub show_footer_contact: bool,
    #[serde(default = "default_true", rename = "showFooterSite")]
    pub show_footer_site: bool,
}

impl Default for LayoutFlags {
    fn default() -> Self {
        Self {
            show_header: true,
            show_logo: true,
            show_subject: true,
            show_from: true,
            show_footer_contact: true,
            show_footer_site: true,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub title: String,
    #[serde(default)]
    pub subtitle: String,
    #[serde(default = "default_locale")]
    pub locale: String,
    #[serde(default, rename = "loginTagline")]
    pub login_tagline: String,
    #[serde(default, rename = "loginHeadlineBefore")]
    pub login_headline_before: String,
    #[serde(default, rename = "loginHeadlineEm")]
    pub login_headline_em: String,
    #[serde(default, rename = "loginLead")]
    pub login_lead: String,
    #[serde(default, rename = "loginPoints")]
    pub login_points: Vec<String>,
    #[serde(default, rename = "loginFormTitle")]
    pub login_form_title: String,
    #[serde(default, rename = "loginFormSub")]
    pub login_form_sub: String,
}

fn default_locale() -> String {
    "en".into()
}

#[derive(Debug, Clone, Deserialize)]
pub struct MailSettings {
    #[serde(rename = "fromEmail")]
    pub from_email: String,
    #[serde(rename = "fromNameDefault")]
    pub from_name_default: String,
    #[serde(default, rename = "contactEmail")]
    pub contact_email: String,
    #[serde(default, alias = "brevoTag")]
    pub tag: String,
    /// Extra domain for providers that need it (Mailgun).
    #[serde(default, rename = "providerDomain")]
    pub provider_domain: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct SiteConfig {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub label: String,
    #[serde(default, rename = "brandName")]
    pub brand_name: String,
    #[serde(default, rename = "logoPath")]
    pub logo_path: String,
    #[serde(default, rename = "logoUrl")]
    pub logo_url: String,
    #[serde(default, rename = "faviconPath")]
    pub favicon_path: String,
}

/// Color overrides applied on top of the selected theme plugin.
/// Empty strings leave the theme default in place.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct BrandOverrides {
    #[serde(default)]
    pub tile: String,
    #[serde(default, rename = "tileEdge")]
    pub tile_edge: String,
    #[serde(default)]
    pub accent: String,
    #[serde(default, rename = "accentDeep")]
    pub accent_deep: String,
    #[serde(default, rename = "accentSoft")]
    pub accent_soft: String,
    #[serde(default)]
    pub cream: String,
    #[serde(default, rename = "siteBlue")]
    pub site_blue: String,
    #[serde(default)]
    pub ink: String,
    #[serde(default)]
    pub muted: String,
    #[serde(default)]
    pub paper: String,
    #[serde(default)]
    pub line: String,
    #[serde(default, rename = "heroFrom")]
    pub hero_from: String,
    #[serde(default, rename = "heroTo")]
    pub hero_to: String,
    #[serde(default, rename = "headerText")]
    pub header_text: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SyntaxChip {
    pub label: String,
    pub insert: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct SyntaxConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub heading: String,
    #[serde(default)]
    pub chips: Vec<SyntaxChip>,
}

/// Console copy. Empty keys fall back to English defaults.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nConfig {
    #[serde(default)]
    pub tab_compose: String,
    #[serde(default)]
    pub tab_history: String,
    #[serde(default)]
    pub logout: String,
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub from_name_placeholder: String,
    #[serde(default)]
    pub to: String,
    #[serde(default)]
    pub to_placeholder: String,
    #[serde(default)]
    pub to_hint: String,
    #[serde(default)]
    pub syntax: String,
    #[serde(default)]
    pub syntax_head: String,
    #[serde(default)]
    pub attach: String,
    #[serde(default)]
    pub attach_drop_title: String,
    #[serde(default)]
    pub attach_drop_sub: String,
    #[serde(default)]
    pub attach_add: String,
    #[serde(default)]
    pub subject: String,
    #[serde(default)]
    pub subject_placeholder: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub body_placeholder: String,
    #[serde(default)]
    pub clear: String,
    #[serde(default)]
    pub preview: String,
    #[serde(default)]
    pub send: String,
    #[serde(default)]
    pub history_title: String,
    #[serde(default)]
    pub history_sub: String,
    #[serde(default)]
    pub history_refresh: String,
    #[serde(default)]
    pub preview_title: String,
    #[serde(default)]
    pub preview_back: String,
    #[serde(default)]
    pub preview_send: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub password_placeholder: String,
    #[serde(default)]
    pub sign_in: String,
    #[serde(default)]
    pub locked: String,
    #[serde(default)]
    pub editing_prefix: String,
    #[serde(default)]
    pub history_loading: String,
    #[serde(default)]
    pub history_loading_hint: String,
    #[serde(default)]
    pub history_load_fail: String,
    #[serde(default)]
    pub retry_later: String,
    #[serde(default)]
    pub history_empty: String,
    #[serde(default)]
    pub history_empty_hint: String,
    #[serde(default)]
    pub status_ok: String,
    #[serde(default)]
    pub status_fail: String,
    #[serde(default)]
    pub n_attachments: String,
    #[serde(default)]
    pub none: String,
    #[serde(default)]
    pub loading_detail: String,
    #[serde(default)]
    pub time: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub attachments: String,
    #[serde(default)]
    pub body_label: String,
    #[serde(default)]
    pub detail: String,
    #[serde(default)]
    pub recipients_status: String,
    #[serde(default)]
    pub empty_body: String,
    #[serde(default)]
    pub generating_preview: String,
    #[serde(default)]
    pub preview_fail: String,
    #[serde(default)]
    pub sending: String,
    #[serde(default)]
    pub send_fail: String,
    #[serde(default)]
    pub send_ok: String,
    #[serde(default)]
    pub send_ok_body: String,
    #[serde(default)]
    pub copied: String,
    #[serde(default)]
    pub click_to_copy: String,
    #[serde(default)]
    pub result_close: String,
    #[serde(default)]
    pub history_back: String,
    #[serde(default)]
    pub select_record: String,
    #[serde(default)]
    pub select_record_hint: String,
    #[serde(default)]
    pub err_need_recipient: String,
    #[serde(default)]
    pub err_need_subject: String,
    #[serde(default)]
    pub err_need_body_or_attach: String,
    #[serde(default)]
    pub err_max_attach: String,
    #[serde(default)]
    pub err_file_too_big: String,
    #[serde(default)]
    pub err_total_too_big: String,
    #[serde(default)]
    pub err_empty_attach: String,
    #[serde(default)]
    pub err_read_file: String,
    #[serde(default)]
    pub body_empty_attach: String,
    #[serde(default)]
    pub missing_id: String,
    #[serde(default)]
    pub missing_record: String,
}

fn fallback(value: &mut String, default: &str) {
    if value.trim().is_empty() {
        *value = default.to_string();
    }
}

impl I18nConfig {
    fn apply_defaults(&mut self) {
        fallback(&mut self.tab_compose, "Compose");
        fallback(&mut self.tab_history, "History");
        fallback(&mut self.logout, "Sign out");
        fallback(&mut self.from, "From");
        fallback(&mut self.from_name_placeholder, "Display name");
        fallback(&mut self.to, "To");
        fallback(&mut self.to_placeholder, "Type an address and press Enter");
        fallback(
            &mut self.to_hint,
            "Pick from the address book or type any email",
        );
        fallback(&mut self.syntax, "Syntax");
        fallback(&mut self.syntax_head, "Click to copy");
        fallback(&mut self.attach, "Attachments");
        fallback(&mut self.attach_drop_title, "Drop or click to add");
        fallback(
            &mut self.attach_drop_sub,
            "Up to 8 files · 8MB each · 15MB total",
        );
        fallback(&mut self.attach_add, "Add files");
        fallback(&mut self.subject, "Subject");
        fallback(&mut self.subject_placeholder, "Subject");
        fallback(&mut self.body, "Body");
        fallback(&mut self.body_placeholder, "Write the message…");
        fallback(&mut self.clear, "Clear");
        fallback(&mut self.preview, "Preview");
        fallback(&mut self.send, "Send");
        fallback(&mut self.history_title, "Send history");
        fallback(&mut self.history_sub, "Latest 10 · click for detail");
        fallback(&mut self.history_refresh, "Refresh");
        fallback(&mut self.preview_title, "Message preview");
        fallback(&mut self.preview_back, "Edit");
        fallback(&mut self.preview_send, "Send now");
        fallback(&mut self.password, "Password");
        fallback(&mut self.password_placeholder, "Enter your password");
        fallback(&mut self.sign_in, "Sign in");
        fallback(&mut self.locked, "LOCKED");
        fallback(&mut self.editing_prefix, "Editing: ");
        fallback(&mut self.history_loading, "Loading");
        fallback(&mut self.history_loading_hint, "Reading send history…");
        fallback(&mut self.history_load_fail, "Failed to load");
        fallback(&mut self.retry_later, "Please try again later");
        fallback(&mut self.history_empty, "No send history yet");
        fallback(&mut self.history_empty_hint, "Successful sends appear here");
        fallback(&mut self.status_ok, "Sent");
        fallback(&mut self.status_fail, "Failed");
        fallback(&mut self.n_attachments, "{n} attachment(s)");
        fallback(&mut self.none, "None");
        fallback(&mut self.loading_detail, "Loading detail…");
        fallback(&mut self.time, "Time");
        fallback(&mut self.status, "Status");
        fallback(&mut self.attachments, "Attachments");
        fallback(&mut self.body_label, "Body");
        fallback(&mut self.detail, "Detail");
        fallback(&mut self.recipients_status, "Recipients and status");
        fallback(&mut self.empty_body, "(no body)");
        fallback(&mut self.generating_preview, "Generating preview…");
        fallback(&mut self.preview_fail, "Preview failed");
        fallback(&mut self.sending, "Sending…");
        fallback(&mut self.send_fail, "Send failed");
        fallback(&mut self.send_ok, "Sent");
        fallback(
            &mut self.send_ok_body,
            "The message was accepted for delivery.",
        );
        fallback(&mut self.copied, "Copied");
        fallback(&mut self.click_to_copy, "Click to copy");
        fallback(&mut self.result_close, "OK");
        fallback(&mut self.history_back, "Back to list");
        fallback(&mut self.select_record, "Select a record");
        fallback(
            &mut self.select_record_hint,
            "Click a row to view recipients, body, and attachments",
        );
        fallback(&mut self.err_need_recipient, "Add at least one recipient");
        fallback(&mut self.err_need_subject, "Subject is required");
        fallback(
            &mut self.err_need_body_or_attach,
            "Provide a body or an attachment",
        );
        fallback(&mut self.err_max_attach, "At most {n} attachments");
        fallback(&mut self.err_file_too_big, "{name} exceeds 8MB");
        fallback(&mut self.err_total_too_big, "Attachments exceed 15MB total");
        fallback(&mut self.err_empty_attach, "Attachment {name} is empty");
        fallback(&mut self.err_read_file, "Could not read {name}");
        fallback(&mut self.body_empty_attach, "(attachment only, no body)");
        fallback(&mut self.missing_id, "Missing record id");
        fallback(&mut self.missing_record, "Record not found");
    }

    pub fn fmt(&self, template: &str, key: &str, value: &str) -> String {
        template.replace(&format!("{{{key}}}"), value)
    }
}

fn default_syntax_chips() -> Vec<SyntaxChip> {
    vec![
        chip("# H1", "# Heading 1"),
        chip("## H2", "## Heading 2"),
        chip("### H3", "### Heading 3"),
        chip("**bold**", "**bold**"),
        chip("*italic*", "*italic*"),
        chip("~~strike~~", "~~strike~~"),
        chip("`code`", "`inline code`"),
        chip("```fence```", "```\ncode\n```"),
        chip("- list", "- list item"),
        chip("1. ordered", "1. ordered item"),
        chip("- [ ] task", "- [ ] task"),
        chip("> quote", "> quote"),
        chip("[link](url)", "[label](https://example.com)"),
        chip("![img](url)", "![alt](https://example.com/image.png)"),
        chip("table", "| Col A | Col B |\n| --- | --- |\n| 1 | 2 |"),
    ]
}

fn chip(label: &str, insert: &str) -> SyntaxChip {
    SyntaxChip {
        label: label.into(),
        insert: insert.into(),
    }
}

impl SyntaxConfig {
    fn apply_defaults(&mut self) {
        if self.heading.trim().is_empty() {
            self.heading = "CommonMark / GFM · click to copy".into();
        }
        if self.chips.is_empty() {
            self.chips = default_syntax_chips();
        }
    }
}

static RAW: &str = include_str!("../config/mail.json");

/// Deep-merge JSON objects. Overlay wins; `null` deletes a key; arrays/scalars replace.
pub fn merge_json(base: serde_json::Value, overlay: serde_json::Value) -> serde_json::Value {
    match (base, overlay) {
        (serde_json::Value::Object(mut acc), serde_json::Value::Object(over)) => {
            for (key, value) in over {
                if value.is_null() {
                    acc.remove(&key);
                } else if let Some(existing) = acc.remove(&key) {
                    acc.insert(key, merge_json(existing, value));
                } else {
                    acc.insert(key, value);
                }
            }
            serde_json::Value::Object(acc)
        }
        (_, overlay) => overlay,
    }
}

fn bundled_config_value() -> serde_json::Value {
    let mut value: serde_json::Value =
        serde_json::from_str(RAW).expect("config/mail.json must be valid JSON");
    for raw in crate::plugins::catalog::config_overlays() {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            continue;
        }
        let overlay: serde_json::Value = serde_json::from_str(trimmed)
            .unwrap_or_else(|e| panic!("config overlay must be valid JSON: {e}"));
        value = merge_json(value, overlay);
    }
    value
}

fn apply_slot(slot: &mut String, raw: &str) {
    let t = raw.trim();
    if !t.is_empty() {
        *slot = t.to_ascii_lowercase();
    }
}

fn finish_config(value: serde_json::Value) -> MailConfig {
    let mut cfg: MailConfig =
        serde_json::from_value(value).expect("merged mail config must match schema");
    cfg.mail.from_email = cfg.mail.from_email.trim().to_lowercase();
    cfg.mail.contact_email = cfg.mail.contact_email.trim().to_lowercase();
    cfg.site.url = cfg.site.url.trim_end_matches('/').to_string();
    if !cfg.site.logo_path.is_empty() && !cfg.site.logo_path.starts_with('/') {
        cfg.site.logo_path = format!("/{}", cfg.site.logo_path);
    }
    if !cfg.site.favicon_path.is_empty() && !cfg.site.favicon_path.starts_with('/') {
        cfg.site.favicon_path = format!("/{}", cfg.site.favicon_path);
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
    if !cfg.features.address_book {
        cfg.address_book.clear();
    }
    cfg.i18n.apply_defaults();
    cfg.syntax.apply_defaults();
    if !cfg.features.syntax_help {
        cfg.syntax.enabled = false;
    }
    cfg.plugins.provider = cfg.plugins.provider.trim().to_ascii_lowercase();
    cfg.plugins.theme = cfg.plugins.theme.trim().to_ascii_lowercase();
    cfg.plugins.layout = cfg.plugins.layout.trim().to_ascii_lowercase();
    cfg.plugins.logo = cfg.plugins.logo.trim().to_ascii_lowercase();
    if cfg.plugins.logo.is_empty() {
        cfg.plugins.logo = default_logo();
    }
    if cfg.app.locale.trim().is_empty() {
        cfg.app.locale = default_locale();
    }
    cfg
}

#[allow(dead_code)]
pub fn load_config() -> MailConfig {
    finish_config(bundled_config_value())
}

/// Load bundled config, then apply `MAIL_CONFIG_JSON` and slot env overrides.
pub fn load_config_with_env(get: impl Fn(&str) -> String) -> MailConfig {
    let mut value = bundled_config_value();
    let extra = get("MAIL_CONFIG_JSON");
    if !extra.trim().is_empty() {
        if let Ok(overlay) = serde_json::from_str::<serde_json::Value>(&extra) {
            value = merge_json(value, overlay);
        }
    }
    let mut cfg = finish_config(value);
    apply_slot(&mut cfg.plugins.provider, &get("MAIL_PROVIDER"));
    apply_slot(&mut cfg.plugins.theme, &get("MAIL_THEME"));
    apply_slot(&mut cfg.plugins.layout, &get("MAIL_LAYOUT"));
    apply_slot(&mut cfg.plugins.logo, &get("MAIL_LOGO"));
    cfg
}

pub fn mail_origin(cfg: &MailConfig) -> String {
    format!("https://{}", cfg.host)
}

fn logo_mode(cfg: &MailConfig) -> crate::plugins::LogoMode {
    crate::plugins::LogoMode::parse(&cfg.plugins.logo)
}

fn configured_image_href(cfg: &MailConfig) -> Option<String> {
    let abs = cfg.site.logo_url.trim();
    if !abs.is_empty() {
        return Some(abs.to_string());
    }
    let path = cfg.site.logo_path.trim();
    if !path.is_empty() {
        return Some(path.to_string());
    }
    crate::plugins::catalog::first_bundled_logo()
}

pub fn configured_logo_url(cfg: &MailConfig) -> Option<String> {
    if !cfg.layout.show_logo || !logo_mode(cfg).uses_image() {
        return None;
    }
    match configured_image_href(cfg)? {
        href if href.starts_with("http://") || href.starts_with("https://") => Some(href),
        path => Some(format!("{}{}", mail_origin(cfg), path)),
    }
}

/// Public path for the console logo `<img>` (relative, served from assets).
pub fn console_logo_src(cfg: &MailConfig) -> Option<String> {
    if !cfg.layout.show_logo || !logo_mode(cfg).uses_image() {
        return None;
    }
    configured_image_href(cfg)
}

pub fn configured_favicon_href(cfg: &MailConfig) -> String {
    let fav = cfg.site.favicon_path.trim();
    if !fav.is_empty() {
        return fav.to_string();
    }
    let logo = cfg.site.logo_path.trim();
    if !logo.is_empty() {
        return logo.to_string();
    }
    "/favicon.svg".into()
}

pub fn contact_email(cfg: &MailConfig) -> Option<&str> {
    if !cfg.layout.show_footer_contact {
        return None;
    }
    let s = cfg.mail.contact_email.trim();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

pub fn site_link(cfg: &MailConfig) -> Option<(&str, &str)> {
    if !cfg.layout.show_footer_site {
        return None;
    }
    let url = cfg.site.url.trim();
    if url.is_empty() {
        return None;
    }
    let label = if cfg.site.label.trim().is_empty() {
        url
    } else {
        cfg.site.label.trim()
    };
    Some((url, label))
}

pub fn brand_name(cfg: &MailConfig) -> &str {
    let n = cfg.site.brand_name.trim();
    if n.is_empty() {
        cfg.app.title.trim()
    } else {
        n
    }
}

impl MailConfig {
    pub fn configured_logo_url(&self) -> Option<String> {
        configured_logo_url(self)
    }

    pub fn console_logo_src(&self) -> Option<String> {
        console_logo_src(self)
    }

    pub fn configured_favicon_href(&self) -> String {
        configured_favicon_href(self)
    }

    pub fn contact_email(&self) -> Option<&str> {
        contact_email(self)
    }

    pub fn site_link(&self) -> Option<(&str, &str)> {
        site_link(self)
    }

    pub fn brand_name(&self) -> &str {
        brand_name(self)
    }
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
        assert!(!cfg.plugins.provider.is_empty());
        assert!(!cfg.i18n.send.is_empty());
    }

    #[test]
    fn i18n_defaults_fill_empty_keys() {
        let mut i = I18nConfig::default();
        i.apply_defaults();
        assert_eq!(i.send, "Send");
        assert!(i.err_need_recipient.contains("recipient"));
    }

    #[test]
    fn empty_logo_disables_image() {
        let mut cfg = load_config();
        cfg.site.logo_path.clear();
        cfg.site.logo_url.clear();
        cfg.plugins.logo = "none".into();
        assert!(configured_logo_url(&cfg).is_none());
        assert!(console_logo_src(&cfg).is_none());
    }

    #[test]
    fn footer_omitted_when_flags_off() {
        let mut cfg = load_config();
        cfg.layout.show_footer_contact = false;
        cfg.layout.show_footer_site = false;
        assert!(contact_email(&cfg).is_none());
        assert!(site_link(&cfg).is_none());
    }

    #[test]
    fn merge_json_overlay_wins_and_null_deletes() {
        let base = serde_json::json!({"plugins":{"theme":"forest","layout":"card"}});
        let over = serde_json::json!({"plugins":{"theme":"nord"}});
        let merged = merge_json(base, over);
        assert_eq!(merged["plugins"]["theme"], "nord");
        assert_eq!(merged["plugins"]["layout"], "card");
        let with_null = merge_json(
            serde_json::json!({"brand":{"accent":"#fff","ink":"#000"}}),
            serde_json::json!({"brand":{"accent":null}}),
        );
        assert!(with_null["brand"].get("accent").is_none());
        assert_eq!(with_null["brand"]["ink"], "#000");
    }

    #[test]
    fn env_overrides_plugin_slots() {
        let cfg = load_config_with_env(|k| match k {
            "MAIL_THEME" => "aurora".into(),
            "MAIL_LAYOUT" => "compact".into(),
            "MAIL_LOGO" => "none".into(),
            _ => String::new(),
        });
        assert_eq!(cfg.plugins.theme, "aurora");
        assert_eq!(cfg.plugins.layout, "compact");
        assert_eq!(cfg.plugins.logo, "none");
    }

    #[test]
    fn env_config_json_merges_before_slot_overrides() {
        let cfg = load_config_with_env(|k| match k {
            "MAIL_CONFIG_JSON" => r#"{"plugins":{"theme":"sunset","layout":"digest"}}"#.into(),
            "MAIL_THEME" => "nord".into(),
            _ => String::new(),
        });
        assert_eq!(cfg.plugins.theme, "nord");
        assert_eq!(cfg.plugins.layout, "digest");
    }

    #[test]
    fn monogram_mode_hides_image_src() {
        let mut cfg = load_config();
        cfg.plugins.logo = "monogram".into();
        assert!(console_logo_src(&cfg).is_none());
        assert!(configured_logo_url(&cfg).is_none());
    }
}
