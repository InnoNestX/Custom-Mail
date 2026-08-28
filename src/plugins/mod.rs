//! Deploy-time plugins: visual theme, HTML layout, and outbound mail provider.
//!
//! Every plugin is compiled into the Worker. Operators pick one of each in
//! `config/mail.json` (`plugins.theme`, `plugins.layout`, `plugins.provider`).
//! Optional sections (logo, header, footer contact, site link, attachments)
//! are omitted when they are not configured.

mod layout;
mod provider;
mod theme;

pub use layout::{wrap_email_html, LayoutId};
pub use provider::{send_via_provider, JobAttachment, ProviderId, ProviderSecrets, SendJob};
pub use theme::{resolve_theme, ThemeId, ThemePalette};
