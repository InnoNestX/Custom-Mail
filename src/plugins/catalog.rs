//! Bundled plugin catalogs compiled from `plugins/**/*.json`.

use serde::Deserialize;
use serde_json::json;
use std::sync::OnceLock;

include!(concat!(env!("OUT_DIR"), "/plugin_catalog.rs"));

#[derive(Clone, Debug, Deserialize)]
pub struct ThemeSpec {
    pub id: String,
    #[serde(default)]
    #[allow(dead_code)]
    pub label: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub accent: String,
    #[serde(rename = "accentDeep")]
    pub accent_deep: String,
    #[serde(rename = "accentSoft")]
    pub accent_soft: String,
    pub ink: String,
    pub muted: String,
    pub paper: String,
    pub line: String,
    #[serde(rename = "heroFrom")]
    pub hero_from: String,
    #[serde(rename = "heroTo")]
    pub hero_to: String,
    #[serde(rename = "headerText")]
    pub header_text: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LayoutSpec {
    pub id: String,
    #[serde(default)]
    #[allow(dead_code)]
    pub label: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default = "default_header_style", rename = "headerStyle")]
    pub header_style: String,
    #[serde(default = "default_body_padding", rename = "bodyPadding")]
    pub body_padding: String,
    #[serde(default = "default_card_shadow", rename = "cardShadow")]
    pub card_shadow: String,
}

fn default_header_style() -> String {
    "plain".into()
}
fn default_body_padding() -> String {
    "20px 28px 8px".into()
}
fn default_card_shadow() -> String {
    "0 18px 40px rgba(16,35,29,.12)".into()
}

#[derive(Clone, Debug, Deserialize)]
pub struct ProviderSpec {
    pub id: String,
    #[serde(default)]
    #[allow(dead_code)]
    pub label: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub secrets: Vec<String>,
    #[serde(default, rename = "needsDomain")]
    pub needs_domain: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FeatureSpec {
    pub id: String,
    #[serde(default)]
    #[allow(dead_code)]
    pub label: String,
    #[serde(default, rename = "configKey")]
    #[allow(dead_code)]
    pub config_key: String,
}

fn parse_all<T: for<'de> Deserialize<'de>>(rows: &[(&str, &str)]) -> Vec<T> {
    rows.iter()
        .map(|(stem, raw)| {
            serde_json::from_str::<T>(raw).unwrap_or_else(|e| panic!("plugin {stem}: {e}"))
        })
        .collect()
}

fn themes() -> &'static [ThemeSpec] {
    static CELL: OnceLock<Vec<ThemeSpec>> = OnceLock::new();
    CELL.get_or_init(|| parse_all(BUNDLED_THEMES))
}

fn layouts() -> &'static [LayoutSpec] {
    static CELL: OnceLock<Vec<LayoutSpec>> = OnceLock::new();
    CELL.get_or_init(|| parse_all(BUNDLED_LAYOUTS))
}

fn providers() -> &'static [ProviderSpec] {
    static CELL: OnceLock<Vec<ProviderSpec>> = OnceLock::new();
    CELL.get_or_init(|| parse_all(BUNDLED_PROVIDERS))
}

fn features() -> &'static [FeatureSpec] {
    static CELL: OnceLock<Vec<FeatureSpec>> = OnceLock::new();
    CELL.get_or_init(|| parse_all(BUNDLED_FEATURES))
}

fn matches_id(id: &str, aliases: &[String], key: &str) -> bool {
    if id.eq_ignore_ascii_case(key) {
        return true;
    }
    aliases.iter().any(|a| a.eq_ignore_ascii_case(key))
}

pub fn resolve_theme_id(raw: &str) -> String {
    let key = raw.trim().to_ascii_lowercase();
    if !key.is_empty() {
        for t in themes() {
            if matches_id(&t.id, &t.aliases, &key) {
                return t.id.clone();
            }
        }
    }
    if themes().iter().any(|t| t.id == "forest") {
        "forest".into()
    } else {
        themes()
            .first()
            .map(|t| t.id.clone())
            .unwrap_or_else(|| "forest".into())
    }
}

pub fn resolve_layout_id(raw: &str) -> String {
    let key = raw.trim().to_ascii_lowercase();
    if !key.is_empty() {
        for t in layouts() {
            if matches_id(&t.id, &t.aliases, &key) {
                return t.id.clone();
            }
        }
    }
    if layouts().iter().any(|t| t.id == "card") {
        "card".into()
    } else {
        layouts()
            .first()
            .map(|t| t.id.clone())
            .unwrap_or_else(|| "card".into())
    }
}

pub fn resolve_provider_id(raw: &str) -> String {
    let key = raw.trim().to_ascii_lowercase();
    if !key.is_empty() {
        for t in providers() {
            if matches_id(&t.id, &t.aliases, &key) {
                return t.id.clone();
            }
        }
    }
    if providers().iter().any(|t| t.id == "brevo") {
        "brevo".into()
    } else {
        providers()
            .first()
            .map(|t| t.id.clone())
            .unwrap_or_else(|| "brevo".into())
    }
}

pub fn theme_by_id(id: &str) -> Option<&'static ThemeSpec> {
    let canon = resolve_theme_id(id);
    themes().iter().find(|t| t.id == canon)
}

pub fn layout_by_id(id: &str) -> Option<&'static LayoutSpec> {
    let canon = resolve_layout_id(id);
    layouts().iter().find(|t| t.id == canon)
}

pub fn provider_by_id(id: &str) -> Option<&'static ProviderSpec> {
    let canon = resolve_provider_id(id);
    providers().iter().find(|t| t.id == canon)
}

pub fn resolve_layout(raw: &str) -> LayoutSpec {
    layout_by_id(raw).cloned().unwrap_or_else(|| LayoutSpec {
        id: "card".into(),
        label: "Card".into(),
        aliases: vec![],
        header_style: default_header_style(),
        body_padding: default_body_padding(),
        card_shadow: default_card_shadow(),
    })
}

pub fn theme_ids() -> Vec<String> {
    themes().iter().map(|t| t.id.clone()).collect()
}

pub fn layout_ids() -> Vec<String> {
    layouts().iter().map(|t| t.id.clone()).collect()
}

pub fn provider_ids() -> Vec<String> {
    providers().iter().map(|t| t.id.clone()).collect()
}

pub fn feature_ids() -> Vec<String> {
    features().iter().map(|t| t.id.clone()).collect()
}

pub fn bundled_logo_paths() -> &'static [&'static str] {
    BUNDLED_LOGOS
}

pub fn first_bundled_logo() -> Option<String> {
    BUNDLED_LOGOS.first().map(|s| (*s).to_string())
}

pub fn config_overlays() -> &'static [&'static str] {
    CONFIG_OVERLAYS
}

pub fn provider_secrets(id: &str) -> Vec<String> {
    provider_by_id(id)
        .map(|p| p.secrets.clone())
        .filter(|s| !s.is_empty())
        .unwrap_or_default()
}

pub fn provider_needs_domain(id: &str) -> bool {
    provider_by_id(id).map(|p| p.needs_domain).unwrap_or(false)
}

pub fn available_catalog() -> serde_json::Value {
    json!({
        "providers": provider_ids(),
        "themes": theme_ids(),
        "layouts": layout_ids(),
        "logos": ["auto", "none", "monogram", "image"],
        "logoFiles": bundled_logo_paths(),
        "features": feature_ids(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalogs_are_nonempty() {
        assert!(provider_ids().contains(&"brevo".into()));
        assert!(provider_ids().contains(&"resend".into()));
        assert!(theme_ids().contains(&"forest".into()));
        assert!(theme_ids().contains(&"aurora".into()));
        assert!(theme_ids().contains(&"nord".into()));
        assert!(layout_ids().contains(&"banner".into()));
        assert!(layout_ids().contains(&"compact".into()));
        assert!(feature_ids().contains(&"markdown".into()));
    }

    #[test]
    fn aliases_resolve() {
        assert_eq!(resolve_theme_id("dark"), "midnight");
        assert_eq!(resolve_theme_id("polar"), "nord");
        assert_eq!(resolve_layout_id("hero"), "banner");
        assert_eq!(resolve_provider_id("sendinblue"), "brevo");
        assert_eq!(resolve_provider_id("mailer-send"), "mailersend");
    }

    #[test]
    fn unknown_theme_falls_back_to_forest() {
        assert_eq!(resolve_theme_id("not-a-theme"), "forest");
        assert_eq!(resolve_layout_id("nope"), "card");
        assert_eq!(resolve_provider_id("nope"), "brevo");
    }

    #[test]
    fn mailgun_needs_domain() {
        assert!(provider_needs_domain("mailgun"));
        assert!(!provider_needs_domain("brevo"));
        assert!(provider_secrets("postmark")
            .iter()
            .any(|s| s == "POSTMARK_SERVER_TOKEN"));
    }
}
