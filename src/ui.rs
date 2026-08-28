use crate::brand::brand_mark_html;
use crate::config::MailConfig;
use crate::markdown::escape_html;
use crate::plugins::{
    resolve_layout_id, resolve_provider_id, resolve_theme, resolve_theme_id, LogoMode,
};

const TEMPLATE: &str = include_str!("../templates/app.html");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

fn hide(on: bool) -> &'static str {
    if on {
        ""
    } else {
        " hidden"
    }
}

fn product_chrome_html(cfg: &MailConfig) -> String {
    let org = escape_html(&cfg.org.name);
    let org_url = escape_html(&cfg.org.url);
    let product = escape_html(&cfg.app.title);
    let version = escape_html(APP_VERSION);
    let releases = escape_html(
        if cfg.org.releases.trim().is_empty() {
            "https://github.com/InnoNestX/Custom-Mail/releases"
        } else {
            cfg.org.releases.as_str()
        },
    );
    let tagline = cfg.org.tagline.trim();
    let tagline_html = if tagline.is_empty() {
        String::new()
    } else {
        format!(
            r#"<span class="chrome-sep" aria-hidden="true">·</span><span class="chrome-tagline">{}</span>"#,
            escape_html(tagline)
        )
    };
    format!(
        r#"<footer class="product-chrome" aria-label="Product attribution">
  <a class="chrome-org" href="{org_url}" target="_blank" rel="noopener noreferrer">{org}</a>
  <span class="chrome-sep" aria-hidden="true">·</span>
  <span class="chrome-product">{product}</span>
  <a class="chrome-version" href="{releases}" target="_blank" rel="noopener noreferrer" title="{product} version">v{version}</a>
  {tagline_html}
</footer>"#,
        org_url = org_url,
        org = org,
        product = product,
        releases = releases,
        version = version,
        tagline_html = tagline_html,
    )
}

fn org_eyebrow_html(cfg: &MailConfig) -> String {
    let name = cfg.org.name.trim();
    if name.is_empty() {
        return String::new();
    }
    let url = cfg.org.url.trim();
    if url.is_empty() {
        format!(
            r#"<p class="org-eyebrow"><span>{}</span></p>"#,
            escape_html(name)
        )
    } else {
        format!(
            r#"<p class="org-eyebrow"><a href="{}" target="_blank" rel="noopener noreferrer">{}</a></p>"#,
            escape_html(url),
            escape_html(name)
        )
    }
}

fn version_pill_html(cfg: &MailConfig) -> String {
    let version = escape_html(APP_VERSION);
    let releases = escape_html(
        if cfg.org.releases.trim().is_empty() {
            "https://github.com/InnoNestX/Custom-Mail/releases"
        } else {
            cfg.org.releases.as_str()
        },
    );
    format!(
        r#"<a class="version-pill" href="{releases}" target="_blank" rel="noopener noreferrer" title="Version {version}">v{version}</a>"#,
        releases = releases,
        version = version,
    )
}

fn login_points_html(points: &[String]) -> String {
    if points.is_empty() {
        return String::new();
    }
    let items: String = points
        .iter()
        .map(|p| format!("<li>{}</li>", escape_html(p)))
        .collect();
    format!(r#"<ul class="login-points">{items}</ul>"#)
}

fn syntax_chips_html(cfg: &MailConfig) -> String {
    if !cfg.syntax.enabled || cfg.syntax.chips.is_empty() {
        return String::new();
    }
    let click = escape_html(&cfg.i18n.click_to_copy);
    let mut rows = String::new();
    rows.push_str(&format!(
        r#"<div class="md-syntax-head">{}</div>"#,
        escape_html(&cfg.syntax.heading)
    ));
    rows.push_str(r#"<div class="md-syntax-row">"#);
    for (i, chip) in cfg.syntax.chips.iter().enumerate() {
        if i > 0 && i % 4 == 0 {
            rows.push_str(r#"</div><div class="md-syntax-row">"#);
        }
        rows.push_str(&format!(
            r#"<code class="md-copy" data-copy="{insert}" title="{click}">{label}</code>"#,
            insert = escape_html(&chip.insert),
            click = click,
            label = escape_html(&chip.label),
        ));
    }
    rows.push_str("</div>");
    rows
}

fn plugin_chips_html(cfg: &MailConfig) -> String {
    let slots = [
        ("provider", resolve_provider_id(&cfg.plugins.provider)),
        ("theme", resolve_theme_id(&cfg.plugins.theme)),
        ("layout", resolve_layout_id(&cfg.plugins.layout)),
        (
            "logo",
            LogoMode::parse(&cfg.plugins.logo).as_str().to_string(),
        ),
    ];
    let chips: String = slots
        .iter()
        .map(|(key, value)| {
            format!(
                r#"<span class="plugin-chip"><span class="plugin-chip-k">{}</span>{}</span>"#,
                escape_html(key),
                escape_html(value)
            )
        })
        .collect();
    format!(r#"<div class="plugin-chips" aria-label="Active plugins">{chips}</div>"#)
}

fn subtitle_block(text: &str, class: &str) -> String {
    if text.trim().is_empty() {
        String::new()
    } else {
        format!(r#"<p class="{class}">{}</p>"#, escape_html(text.trim()))
    }
}

pub fn render_app_html(cfg: &MailConfig, from_name: &str, from_email: &str) -> String {
    let pal = resolve_theme(&cfg.plugins, &cfg.brand);
    let book = serde_json::to_string(&cfg.address_book).unwrap_or_else(|_| "[]".into());
    let book = book.replace('<', "\\u003c");
    let bootstrap = serde_json::json!({
        "locale": cfg.app.locale,
        "features": cfg.features,
        "i18n": cfg.i18n,
        "plugins": {
            "provider": resolve_provider_id(&cfg.plugins.provider),
            "theme": resolve_theme_id(&cfg.plugins.theme),
            "layout": resolve_layout_id(&cfg.plugins.layout),
            "logo": LogoMode::parse(&cfg.plugins.logo).as_str(),
        }
    });
    let bootstrap = serde_json::to_string(&bootstrap)
        .unwrap_or_else(|_| "{}".into())
        .replace('<', "\\u003c");

    let headline = format!(
        "{}{}",
        escape_html(&cfg.app.login_headline_before),
        if cfg.app.login_headline_em.trim().is_empty() {
            String::new()
        } else {
            format!(" <em>{}</em>", escape_html(&cfg.app.login_headline_em))
        }
    );

    let login_lead = if cfg.app.login_lead.trim().is_empty() {
        String::new()
    } else {
        format!(
            r#"<p class="login-lead">{}</p>"#,
            escape_html(&cfg.app.login_lead)
        )
    };

    TEMPLATE
        .replace("___HTML_LANG___", &escape_html(&cfg.app.locale))
        .replace("___THEME_COLOR___", &escape_html(&pal.paper))
        .replace("___PAGE_TITLE___", &escape_html(&cfg.app.title))
        .replace(
            "___FAVICON_HREF___",
            &escape_html(&cfg.configured_favicon_href()),
        )
        .replace(
            "___APPLE_TOUCH_HREF___",
            &escape_html(&cfg.configured_favicon_href()),
        )
        .replace("___THEME_VARS___", &pal.css_vars())
        .replace("___BRAND_MARK_HDR___", &brand_mark_html(cfg, 44, "hdr"))
        .replace("___BRAND_MARK_HERO___", &brand_mark_html(cfg, 56, "hero"))
        .replace("___BRAND_MARK_APP___", &brand_mark_html(cfg, 44, "app"))
        .replace("___ORG_EYEBROW___", &org_eyebrow_html(cfg))
        .replace("___PRODUCT_CHROME___", &product_chrome_html(cfg))
        .replace("___VERSION_PILL___", &version_pill_html(cfg))
        .replace("___APP_TITLE___", &escape_html(&cfg.app.title))
        .replace(
            "___APP_SUBTITLE_P___",
            &subtitle_block(&cfg.app.subtitle, ""),
        )
        .replace("___PLUGIN_CHIPS___", &plugin_chips_html(cfg))
        .replace(
            "___LOGIN_TAGLINE_P___",
            &subtitle_block(&cfg.app.login_tagline, "login-tagline"),
        )
        .replace("___LOGIN_HEADLINE___", &headline)
        .replace("___LOGIN_LEAD___", &login_lead)
        .replace(
            "___LOGIN_POINTS___",
            &login_points_html(&cfg.app.login_points),
        )
        .replace(
            "___LOGIN_FORM_TITLE___",
            &escape_html(&cfg.app.login_form_title),
        )
        .replace(
            "___LOGIN_FORM_SUB___",
            &escape_html(&cfg.app.login_form_sub),
        )
        .replace("___I18N_PASSWORD___", &escape_html(&cfg.i18n.password))
        .replace(
            "___I18N_PASSWORD_PLACEHOLDER___",
            &escape_html(&cfg.i18n.password_placeholder),
        )
        .replace("___I18N_SIGN_IN___", &escape_html(&cfg.i18n.sign_in))
        .replace(
            "___I18N_TAB_COMPOSE___",
            &escape_html(&cfg.i18n.tab_compose),
        )
        .replace(
            "___I18N_TAB_HISTORY___",
            &escape_html(&cfg.i18n.tab_history),
        )
        .replace("___I18N_LOGOUT___", &escape_html(&cfg.i18n.logout))
        .replace("___I18N_FROM___", &escape_html(&cfg.i18n.from))
        .replace(
            "___I18N_FROM_NAME_PLACEHOLDER___",
            &escape_html(&cfg.i18n.from_name_placeholder),
        )
        .replace("___I18N_LOCKED___", &escape_html(&cfg.i18n.locked))
        .replace("___I18N_TO___", &escape_html(&cfg.i18n.to))
        .replace(
            "___I18N_TO_PLACEHOLDER___",
            &escape_html(&cfg.i18n.to_placeholder),
        )
        .replace("___I18N_TO_HINT___", &escape_html(&cfg.i18n.to_hint))
        .replace("___I18N_SYNTAX___", &escape_html(&cfg.i18n.syntax))
        .replace("___SYNTAX_CHIPS___", &syntax_chips_html(cfg))
        .replace("___I18N_ATTACH___", &escape_html(&cfg.i18n.attach))
        .replace(
            "___I18N_ATTACH_DROP_TITLE___",
            &escape_html(&cfg.i18n.attach_drop_title),
        )
        .replace(
            "___I18N_ATTACH_DROP_SUB___",
            &escape_html(&cfg.i18n.attach_drop_sub),
        )
        .replace("___I18N_ATTACH_ADD___", &escape_html(&cfg.i18n.attach_add))
        .replace("___I18N_SUBJECT___", &escape_html(&cfg.i18n.subject))
        .replace(
            "___I18N_SUBJECT_PLACEHOLDER___",
            &escape_html(&cfg.i18n.subject_placeholder),
        )
        .replace("___I18N_BODY___", &escape_html(&cfg.i18n.body))
        .replace(
            "___I18N_BODY_PLACEHOLDER___",
            &escape_html(&cfg.i18n.body_placeholder),
        )
        .replace("___I18N_CLEAR___", &escape_html(&cfg.i18n.clear))
        .replace("___I18N_PREVIEW___", &escape_html(&cfg.i18n.preview))
        .replace("___I18N_SEND___", &escape_html(&cfg.i18n.send))
        .replace(
            "___I18N_HISTORY_TITLE___",
            &escape_html(&cfg.i18n.history_title),
        )
        .replace(
            "___I18N_HISTORY_SUB___",
            &escape_html(&cfg.i18n.history_sub),
        )
        .replace(
            "___I18N_HISTORY_REFRESH___",
            &escape_html(&cfg.i18n.history_refresh),
        )
        .replace(
            "___I18N_HISTORY_BACK___",
            &escape_html(&cfg.i18n.history_back),
        )
        .replace(
            "___I18N_SELECT_RECORD___",
            &escape_html(&cfg.i18n.select_record),
        )
        .replace(
            "___I18N_SELECT_RECORD_HINT___",
            &escape_html(&cfg.i18n.select_record_hint),
        )
        .replace("___I18N_SEND_OK___", &escape_html(&cfg.i18n.send_ok))
        .replace(
            "___I18N_RESULT_CLOSE___",
            &escape_html(&cfg.i18n.result_close),
        )
        .replace(
            "___I18N_PREVIEW_TITLE___",
            &escape_html(&cfg.i18n.preview_title),
        )
        .replace(
            "___I18N_PREVIEW_BACK___",
            &escape_html(&cfg.i18n.preview_back),
        )
        .replace(
            "___I18N_PREVIEW_SEND___",
            &escape_html(&cfg.i18n.preview_send),
        )
        .replace("___FEATURE_HISTORY___", hide(cfg.features.history))
        .replace("___FEATURE_ATTACH___", hide(cfg.features.attachments))
        .replace(
            "___FEATURE_SYNTAX___",
            hide(cfg.syntax.enabled && cfg.features.syntax_help),
        )
        .replace(
            "___FEATURE_BOOK___",
            hide(cfg.features.address_book && !cfg.address_book.is_empty()),
        )
        .replace("___FROM_NAME___", &escape_html(from_name))
        .replace("___FROM_EMAIL___", &escape_html(from_email))
        .replace("___ADDRESS_BOOK___", &book)
        .replace("___BOOTSTRAP___", &bootstrap)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::load_config;

    #[test]
    fn plugin_chips_include_active_slots() {
        let cfg = load_config();
        let html = plugin_chips_html(&cfg);
        assert!(html.contains("plugin-chip-k"), "{html}");
        assert!(html.contains("provider"), "{html}");
        assert!(html.contains("theme"), "{html}");
        assert!(html.contains("layout"), "{html}");
        assert!(html.contains("logo"), "{html}");
    }

    #[test]
    fn render_includes_plugin_chips() {
        let cfg = load_config();
        let html = render_app_html(&cfg, "Desk", "a@b.test");
        assert!(html.contains("plugin-chips"));
        assert!(!html.contains("___PLUGIN_CHIPS___"));
    }

    #[test]
    fn render_includes_org_chrome_and_version() {
        let cfg = load_config();
        let html = render_app_html(&cfg, "Desk", "a@b.test");
        assert!(html.contains("InnoNestX"), "{html}");
        assert!(html.contains("product-chrome"), "{html}");
        assert!(html.contains(&format!("v{APP_VERSION}")), "{html}");
        assert!(html.contains("version-pill"), "{html}");
        assert!(html.contains("org-eyebrow"), "{html}");
        assert!(!html.contains("___PRODUCT_CHROME___"));
        assert!(!html.contains("___ORG_EYEBROW___"));
        assert!(!html.contains("___VERSION_PILL___"));
    }
}
