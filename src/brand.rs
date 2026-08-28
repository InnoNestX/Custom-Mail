use crate::config::MailConfig;
use crate::markdown::escape_html;
use crate::plugins::{resolve_theme, LogoMode, ThemePalette};

fn palette(cfg: &MailConfig) -> ThemePalette {
    resolve_theme(&cfg.plugins, &cfg.brand)
}

fn monogram_letter(cfg: &MailConfig) -> String {
    cfg.brand_name()
        .chars()
        .find(|c| !c.is_whitespace())
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "M".into())
}

/// Generated mark used when the operator did not configure a logo file.
fn monogram_svg(cfg: &MailConfig, pal: &ThemePalette, size: u32, grad_key: &str) -> String {
    let letter = escape_html(&monogram_letter(cfg));
    let grad_id = format!("cm-g-{grad_key}");
    format!(
        r##"<svg class="logo-mark" width="{size}" height="{size}" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
  <defs>
    <linearGradient id="{grad_id}" x1="6" y1="4" x2="42" y2="44" gradientUnits="userSpaceOnUse">
      <stop stop-color="{from}"/>
      <stop offset="1" stop-color="{to}"/>
    </linearGradient>
  </defs>
  <rect width="48" height="48" rx="14" fill="url(#{grad_id})"/>
  <text x="24" y="32" text-anchor="middle" font-size="22" font-weight="700" font-family="ui-sans-serif,system-ui,sans-serif" fill="{fg}">{letter}</text>
</svg>"##,
        size = size,
        grad_id = grad_id,
        from = pal.hero_from,
        to = pal.hero_to,
        fg = pal.header_text,
        letter = letter,
    )
}

fn image_tag(cfg: &MailConfig, src: &str, size: u32) -> String {
    format!(
        r#"<img class="logo-mark" src="{src}" width="{size}" height="{size}" alt="{alt}" />"#,
        src = escape_html(src),
        size = size,
        alt = escape_html(cfg.brand_name()),
    )
}

/// Console brand mark: configured image, monogram, or omitted.
pub fn brand_mark_html(cfg: &MailConfig, size: u32, grad_key: &str) -> String {
    if !cfg.layout.show_logo {
        return String::new();
    }
    let pal = palette(cfg);
    match LogoMode::parse(&cfg.plugins.logo) {
        LogoMode::None => String::new(),
        LogoMode::Monogram => monogram_svg(cfg, &pal, size, grad_key),
        LogoMode::Image => {
            if let Some(src) = cfg.console_logo_src() {
                image_tag(cfg, &src, size)
            } else {
                monogram_svg(cfg, &pal, size, grad_key)
            }
        }
        LogoMode::Auto => {
            if let Some(src) = cfg.console_logo_src() {
                image_tag(cfg, &src, size)
            } else if cfg.brand_name().is_empty() {
                String::new()
            } else {
                monogram_svg(cfg, &pal, size, grad_key)
            }
        }
    }
}

/// Logo block for outbound HTML. Images use an absolute URL; monogram is inline SVG.
pub fn email_logo_html(cfg: &MailConfig, pal: &ThemePalette) -> String {
    if !cfg.layout.show_logo {
        return String::new();
    }
    match LogoMode::parse(&cfg.plugins.logo) {
        LogoMode::None => String::new(),
        LogoMode::Monogram => monogram_svg(cfg, pal, 40, "mail"),
        LogoMode::Image => {
            if let Some(url) = cfg.configured_logo_url() {
                format!(
                    r#"<img src="{src}" alt="{alt}" width="40" height="40" style="display:block;border:0;border-radius:10px;max-width:40px;height:auto;" />"#,
                    src = escape_html(&url),
                    alt = escape_html(cfg.brand_name()),
                )
            } else {
                monogram_svg(cfg, pal, 40, "mail")
            }
        }
        LogoMode::Auto => {
            if let Some(url) = cfg.configured_logo_url() {
                format!(
                    r#"<img src="{src}" alt="{alt}" width="40" height="40" style="display:block;border:0;border-radius:10px;max-width:40px;height:auto;" />"#,
                    src = escape_html(&url),
                    alt = escape_html(cfg.brand_name()),
                )
            } else if cfg.brand_name().is_empty() {
                String::new()
            } else {
                monogram_svg(cfg, pal, 40, "mail")
            }
        }
    }
}

pub fn favicon_svg(cfg: &MailConfig, size: u32) -> String {
    let pal = palette(cfg);
    let letter = escape_html(&monogram_letter(cfg));
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48" width="{size}" height="{size}">
  <defs>
    <linearGradient id="cm-grad-fav" x1="6" y1="4" x2="42" y2="44" gradientUnits="userSpaceOnUse">
      <stop stop-color="{from}"/>
      <stop offset="1" stop-color="{to}"/>
    </linearGradient>
  </defs>
  <rect width="48" height="48" rx="14" fill="url(#cm-grad-fav)"/>
  <text x="24" y="32" text-anchor="middle" font-size="22" font-weight="700" font-family="ui-sans-serif,system-ui,sans-serif" fill="{fg}">{letter}</text>
</svg>"##,
        size = size,
        from = pal.hero_from,
        to = pal.hero_to,
        fg = pal.header_text,
        letter = letter,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::load_config;

    #[test]
    fn configured_logo_renders_img() {
        let mut cfg = load_config();
        cfg.plugins.logo = "image".into();
        if cfg.console_logo_src().is_some() {
            let html = brand_mark_html(&cfg, 40, "t");
            assert!(html.contains("<img"), "{html}");
            assert!(!html.contains("<svg"), "{html}");
        }
    }

    #[test]
    fn missing_logo_renders_monogram() {
        let mut cfg = load_config();
        cfg.site.logo_path.clear();
        cfg.site.logo_url.clear();
        cfg.plugins.logo = "monogram".into();
        let html = brand_mark_html(&cfg, 40, "t");
        assert!(html.contains("<svg"), "{html}");
        assert!(html.contains("<text"), "{html}");
    }

    #[test]
    fn logo_none_omits_mark() {
        let mut cfg = load_config();
        cfg.plugins.logo = "none".into();
        let html = brand_mark_html(&cfg, 40, "t");
        assert!(html.is_empty(), "{html}");
    }
}
