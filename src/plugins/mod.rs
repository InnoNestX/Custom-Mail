//! Deploy-time plugins: visual theme, HTML layout, outbound mail provider, and logo.
//!
//! Catalog JSON under `plugins/` is compiled into the Worker (`build.rs`).
//! Operators pick one of each in `config/mail.json` (`plugins.theme`,
//! `plugins.layout`, `plugins.provider`, `plugins.logo`). Optional sections
//! (logo, header, footer contact, site link, attachments) are omitted when
//! they are not configured. Runtime env can override the slots without a rebuild.

pub mod catalog;
mod layout;
mod logo;
mod provider;
mod theme;

pub use catalog::available_catalog;
pub use layout::wrap_email_html;
pub use logo::LogoMode;
pub use provider::{send_via_provider, JobAttachment, ProviderId, ProviderSecrets, SendJob};
pub use theme::{resolve_theme, ThemePalette};

pub use catalog::{
    provider_needs_domain, provider_secrets, resolve_layout_id, resolve_provider_id,
    resolve_theme_id,
};
