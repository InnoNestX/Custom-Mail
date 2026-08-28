use crate::config::{BrandOverrides, PluginsConfig};
use crate::plugins::catalog::{self, ThemeSpec};

#[derive(Clone, Debug)]
pub struct ThemePalette {
    #[allow(dead_code)]
    pub id: String,
    pub accent: String,
    pub accent_deep: String,
    pub accent_soft: String,
    pub ink: String,
    pub muted: String,
    pub paper: String,
    pub line: String,
    pub hero_from: String,
    pub hero_to: String,
    pub header_text: String,
}

impl ThemePalette {
    fn from_spec(spec: &ThemeSpec) -> Self {
        Self {
            id: spec.id.clone(),
            accent: spec.accent.clone(),
            accent_deep: spec.accent_deep.clone(),
            accent_soft: spec.accent_soft.clone(),
            ink: spec.ink.clone(),
            muted: spec.muted.clone(),
            paper: spec.paper.clone(),
            line: spec.line.clone(),
            hero_from: spec.hero_from.clone(),
            hero_to: spec.hero_to.clone(),
            header_text: spec.header_text.clone(),
        }
    }

    fn forest_fallback() -> Self {
        Self {
            id: "forest".into(),
            accent: "#15624f".into(),
            accent_deep: "#0d3d32".into(),
            accent_soft: "#e7f3ef".into(),
            ink: "#10231d".into(),
            muted: "#5b6f68".into(),
            paper: "#f4f1ea".into(),
            line: "#d5e4de".into(),
            hero_from: "#15624f".into(),
            hero_to: "#0d3d32".into(),
            header_text: "#ffffff".into(),
        }
    }

    fn apply_overrides(mut self, brand: &BrandOverrides) -> Self {
        let set = |slot: &mut String, raw: &str| {
            let t = raw.trim();
            if !t.is_empty() {
                *slot = t.to_string();
            }
        };
        // Legacy tile fields drive the header gradient when hero* is unset.
        set(&mut self.accent, &brand.accent);
        set(&mut self.accent_deep, &brand.accent_deep);
        set(&mut self.accent_soft, &brand.accent_soft);
        set(&mut self.ink, &brand.ink);
        set(&mut self.muted, &brand.muted);
        set(&mut self.paper, &brand.paper);
        set(&mut self.line, &brand.line);
        set(&mut self.hero_from, &brand.hero_from);
        set(&mut self.hero_to, &brand.hero_to);
        set(&mut self.header_text, &brand.header_text);
        if brand.hero_from.trim().is_empty() {
            set(&mut self.hero_from, &brand.tile);
        }
        if brand.hero_to.trim().is_empty() {
            set(&mut self.hero_to, &brand.tile_edge);
        }
        if brand.accent_deep.trim().is_empty() {
            set(&mut self.accent_deep, &brand.tile_edge);
        }
        if brand.paper.trim().is_empty() {
            set(&mut self.paper, &brand.cream);
        }
        if brand.accent.trim().is_empty() && !brand.tile.trim().is_empty() {
            self.accent = brand.tile.trim().to_string();
        }
        let _ = brand.site_blue.trim();
        self
    }

    pub fn css_vars(&self) -> String {
        format!(
            "--accent:{a};--accent-2:{a};--accent-deep:{d};--accent-soft:{s};--ink:{ink};--ink-soft:{m};--muted:{m};--paper:{p};--line:{l};--hero-from:{hf};--hero-to:{ht};--header-text:{htx};--theme-color:{p};",
            a = self.accent,
            d = self.accent_deep,
            s = self.accent_soft,
            ink = self.ink,
            m = self.muted,
            p = self.paper,
            l = self.line,
            hf = self.hero_from,
            ht = self.hero_to,
            htx = self.header_text,
        )
    }
}

pub fn resolve_theme(plugins: &PluginsConfig, brand: &BrandOverrides) -> ThemePalette {
    let pal = catalog::theme_by_id(&plugins.theme)
        .map(ThemePalette::from_spec)
        .unwrap_or_else(ThemePalette::forest_fallback);
    pal.apply_overrides(brand)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::PluginsConfig;

    #[test]
    fn unknown_theme_falls_back_to_forest() {
        let plugins = PluginsConfig {
            theme: "not-a-theme".into(),
            ..PluginsConfig::default()
        };
        let pal = resolve_theme(&plugins, &BrandOverrides::default());
        assert_eq!(pal.id, "forest");
        assert_eq!(pal.accent, "#15624f");
    }

    #[test]
    fn json_theme_aurora_loads() {
        let plugins = PluginsConfig {
            theme: "aurora".into(),
            ..PluginsConfig::default()
        };
        let pal = resolve_theme(&plugins, &BrandOverrides::default());
        assert_eq!(pal.id, "aurora");
        assert_eq!(pal.accent, "#7c3aed");
    }

    #[test]
    fn brand_overrides_replace_palette_colors() {
        let plugins = PluginsConfig {
            theme: "ocean".into(),
            ..PluginsConfig::default()
        };
        let brand = BrandOverrides {
            accent: "#ff00aa".into(),
            ..BrandOverrides::default()
        };
        let pal = resolve_theme(&plugins, &brand);
        assert_eq!(pal.id, "ocean");
        assert_eq!(pal.accent, "#ff00aa");
        assert_eq!(pal.hero_from, "#0e7490");
    }
}
