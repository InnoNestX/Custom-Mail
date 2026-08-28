use crate::brand::email_logo_html;
use crate::config::MailConfig;
use crate::markdown::escape_html;
use crate::plugins::catalog::{self, LayoutSpec};
use crate::plugins::theme::ThemePalette;

fn header_html(cfg: &MailConfig, pal: &ThemePalette, spec: &LayoutSpec) -> String {
    if !cfg.layout.show_header || spec.header_style.eq_ignore_ascii_case("none") {
        return String::new();
    }
    let name = escape_html(cfg.brand_name());
    let logo = email_logo_html(cfg, pal);
    if name.is_empty() && logo.is_empty() {
        return String::new();
    }
    let title = if name.is_empty() {
        String::new()
    } else {
        format!(r#"<div style="font-size:15px;font-weight:700;letter-spacing:.04em;">{name}</div>"#)
    };
    let gradient = spec.header_style.eq_ignore_ascii_case("gradient");
    if gradient {
        format!(
            r#"<tr><td style="padding:22px 28px;background:{from};background:linear-gradient(135deg,{from},{to});color:{fg};">
              <table role="presentation" cellpadding="0" cellspacing="0"><tr>
                {logo_cell}
                <td style="vertical-align:middle;color:{fg};">{title}</td>
              </tr></table>
            </td></tr>"#,
            from = pal.hero_from,
            to = pal.hero_to,
            fg = pal.header_text,
            logo_cell = if logo.is_empty() {
                String::new()
            } else {
                format!(r#"<td style="padding-right:12px;vertical-align:middle;">{logo}</td>"#)
            },
        )
    } else if logo.is_empty() && title.is_empty() {
        String::new()
    } else {
        format!(
            r#"<tr><td style="padding:20px 28px 0;">
              <table role="presentation" cellpadding="0" cellspacing="0"><tr>
                {logo_cell}
                <td style="vertical-align:middle;color:{ink};">{title}</td>
              </tr></table>
            </td></tr>"#,
            ink = pal.ink,
            logo_cell = if logo.is_empty() {
                String::new()
            } else {
                format!(r#"<td style="padding-right:12px;vertical-align:middle;">{logo}</td>"#)
            },
        )
    }
}

fn subject_block(cfg: &MailConfig, pal: &ThemePalette, subject: &str) -> String {
    if !cfg.layout.show_subject {
        return String::new();
    }
    format!(
        r#"<h1 style="margin:0 0 8px;font-size:22px;line-height:1.25;color:{ink};">{subject}</h1>"#,
        ink = pal.ink,
        subject = escape_html(subject),
    )
}

fn from_block(cfg: &MailConfig, pal: &ThemePalette, from_name: &str) -> String {
    if !cfg.layout.show_from {
        return String::new();
    }
    let name = from_name.trim();
    if name.is_empty() {
        return String::new();
    }
    format!(
        r#"<p style="margin:0 0 18px;font-size:13px;color:{muted};">{name}</p>"#,
        muted = pal.muted,
        name = escape_html(name),
    )
}

fn footer_html(cfg: &MailConfig, pal: &ThemePalette) -> String {
    let mut parts: Vec<String> = Vec::new();
    if cfg.layout.show_footer_contact {
        if let Some(email) = cfg.contact_email() {
            parts.push(format!(
                r#"<a href="mailto:{email}" style="color:{accent};text-decoration:none;">{email}</a>"#,
                email = escape_html(email),
                accent = pal.accent,
            ));
        }
    }
    if cfg.layout.show_footer_site {
        if let Some((href, label)) = cfg.site_link() {
            parts.push(format!(
                r#"<a href="{href}" style="color:{accent};text-decoration:none;">{label}</a>"#,
                href = escape_html(href),
                label = escape_html(label),
                accent = pal.accent,
            ));
        }
    }
    if parts.is_empty() {
        return String::new();
    }
    format!(
        r#"<tr><td style="padding:0 28px 24px;">
          <p style="margin:18px 0 0;padding-top:14px;border-top:1px solid {line};font-size:12px;color:{muted};">{inner}</p>
        </td></tr>"#,
        line = pal.line,
        muted = pal.muted,
        inner = parts.join(" · "),
    )
}

/// Wrap rendered body HTML in the selected layout. Empty optional sections are omitted.
pub fn wrap_email_html(
    cfg: &MailConfig,
    pal: &ThemePalette,
    subject: &str,
    inner: &str,
    from_name: &str,
    interactive: bool,
) -> String {
    let spec = catalog::resolve_layout(&cfg.plugins.layout);
    let header = header_html(cfg, pal, &spec);
    let footer = footer_html(cfg, pal);
    let subject_html = subject_block(cfg, pal, subject);
    let from_html = from_block(cfg, pal, from_name);
    let copy_script = if interactive {
        r#"<script>(function(){document.querySelectorAll("a.xxm-copy-btn").forEach(function(a){a.addEventListener("click",function(e){e.preventDefault();var t=a.getAttribute("data-copy")||"";if(navigator.clipboard&&navigator.clipboard.writeText){navigator.clipboard.writeText(t).then(function(){var p=a.textContent;a.textContent="Copied";setTimeout(function(){a.textContent=p;},1200);});}});});})();</script>"#
    } else {
        ""
    };
    let lang = if cfg.app.locale.trim().is_empty() {
        "en"
    } else {
        cfg.app.locale.trim()
    };
    let pad = spec.body_padding.as_str();
    let card_shadow = spec.card_shadow.as_str();
    let outer_bg = pal.paper.as_str();
    format!(
        r#"<!DOCTYPE html>
<html lang="{lang}">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>{subject}</title>
</head>
<body style="margin:0;padding:0;background:{outer};font-family:ui-sans-serif,system-ui,-apple-system,Segoe UI,sans-serif;color:{ink};">
  <table role="presentation" width="100%" cellpadding="0" cellspacing="0" style="background:{outer};padding:28px 12px;">
    <tr><td align="center">
      <table role="presentation" width="100%" cellpadding="0" cellspacing="0" style="max-width:640px;background:#ffffff;border-radius:18px;overflow:hidden;border:1px solid {line};box-shadow:{shadow};">
        {header}
        <tr><td style="padding:{pad};">
          {subject_html}
          {from_html}
          <div class="mail-body">{inner}</div>
        </td></tr>
        {footer}
      </table>
    </td></tr>
  </table>
  {copy_script}
</body>
</html>"#,
        lang = escape_html(lang),
        subject = escape_html(subject),
        outer = outer_bg,
        ink = pal.ink,
        line = pal.line,
        shadow = card_shadow,
        pad = pad,
        header = header,
        subject_html = subject_html,
        from_html = from_html,
        inner = inner,
        footer = footer,
        copy_script = copy_script,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::load_config;
    use crate::plugins::theme::resolve_theme;

    #[test]
    fn omits_footer_when_flags_off() {
        let mut c = load_config();
        c.layout.show_footer_contact = false;
        c.layout.show_footer_site = false;
        c.site.url.clear();
        c.mail.contact_email.clear();
        let pal = resolve_theme(&c.plugins, &c.brand);
        let html = wrap_email_html(&c, &pal, "Hello", "<p>Hi</p>", "Desk", false);
        assert!(!html.contains("mailto:"));
        assert!(html.contains("<p>Hi</p>"));
    }

    #[test]
    fn includes_contact_when_configured() {
        let mut c = load_config();
        c.layout.show_footer_contact = true;
        c.layout.show_footer_site = true;
        c.mail.contact_email = "hi@desk.test".into();
        c.site.url = "https://desk.test".into();
        c.site.label = "desk.test".into();
        let pal = resolve_theme(&c.plugins, &c.brand);
        let html = wrap_email_html(&c, &pal, "Hello", "<p>Hi</p>", "Desk", false);
        assert!(html.contains("mailto:hi@desk.test"));
        assert!(html.contains("https://desk.test"));
    }

    #[test]
    fn omits_header_when_disabled() {
        let mut c = load_config();
        c.layout.show_header = false;
        let pal = resolve_theme(&c.plugins, &c.brand);
        let html = wrap_email_html(&c, &pal, "Hello", "<p>Hi</p>", "Desk", false);
        assert!(!html.contains("linear-gradient"));
        assert!(html.contains("Hello"));
    }

    #[test]
    fn compact_layout_uses_json_padding() {
        let mut c = load_config();
        c.plugins.layout = "compact".into();
        let pal = resolve_theme(&c.plugins, &c.brand);
        let html = wrap_email_html(&c, &pal, "Hello", "<p>Hi</p>", "Desk", false);
        assert!(html.contains("12px 16px 12px"), "{html}");
        assert!(!html.contains("linear-gradient"));
    }
}
