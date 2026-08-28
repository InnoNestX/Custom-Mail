use crate::config::{BrandOverrides, PluginsConfig};

/// Built-in palettes. Operators pick one with `plugins.theme`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeId {
    Forest,
    Midnight,
    Ocean,
    Paper,
    Rose,
    Slate,
}

impl ThemeId {
    pub fn parse(raw: &str) -> Self {
        match raw.trim().to_ascii_lowercase().as_str() {
            "midnight" | "dark" => Self::Midnight,
            "ocean" | "blue" => Self::Ocean,
            "paper" | "light" => Self::Paper,
            "rose" | "pink" => Self::Rose,
            "slate" | "gray" | "grey" | "neutral" => Self::Slate,
            _ => Self::Forest,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Forest => "forest",
            Self::Midnight => "midnight",
            Self::Ocean => "ocean",
            Self::Paper => "paper",
            Self::Rose => "rose",
            Self::Slate => "slate",
        }
    }
}

#[derive(Clone, Debug)]
pub struct ThemePalette {
    #[allow(dead_code)]
    pub id: ThemeId,
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
    fn forest() -> Self {
        Self {
            id: ThemeId::Forest,
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

    fn midnight() -> Self {
        Self {
            id: ThemeId::Midnight,
            accent: "#7c9cff".into(),
            accent_deep: "#1a2140".into(),
            accent_soft: "#e8edff".into(),
            ink: "#e8ecf8".into(),
            muted: "#9aa3c2".into(),
            paper: "#121624".into(),
            line: "#2a3358".into(),
            hero_from: "#1a2140".into(),
            hero_to: "#0d1224".into(),
            header_text: "#ffffff".into(),
        }
    }

    fn ocean() -> Self {
        Self {
            id: ThemeId::Ocean,
            accent: "#0e7490".into(),
            accent_deep: "#155e75".into(),
            accent_soft: "#e0f2fe".into(),
            ink: "#0f172a".into(),
            muted: "#475569".into(),
            paper: "#f0f9ff".into(),
            line: "#bae6fd".into(),
            hero_from: "#0e7490".into(),
            hero_to: "#164e63".into(),
            header_text: "#ffffff".into(),
        }
    }

    fn paper() -> Self {
        Self {
            id: ThemeId::Paper,
            accent: "#44403c".into(),
            accent_deep: "#1c1917".into(),
            accent_soft: "#f5f5f4".into(),
            ink: "#1c1917".into(),
            muted: "#78716c".into(),
            paper: "#fafaf9".into(),
            line: "#e7e5e4".into(),
            hero_from: "#292524".into(),
            hero_to: "#1c1917".into(),
            header_text: "#fafaf9".into(),
        }
    }

    fn rose() -> Self {
        Self {
            id: ThemeId::Rose,
            accent: "#be123c".into(),
            accent_deep: "#9f1239".into(),
            accent_soft: "#ffe4e6".into(),
            ink: "#1c1917".into(),
            muted: "#9f1239".into(),
            paper: "#fff1f2".into(),
            line: "#fecdd3".into(),
            hero_from: "#be123c".into(),
            hero_to: "#9f1239".into(),
            header_text: "#ffffff".into(),
        }
    }

    fn slate() -> Self {
        Self {
            id: ThemeId::Slate,
            accent: "#334155".into(),
            accent_deep: "#1e293b".into(),
            accent_soft: "#f1f5f9".into(),
            ink: "#0f172a".into(),
            muted: "#64748b".into(),
            paper: "#f8fafc".into(),
            line: "#e2e8f0".into(),
            hero_from: "#334155".into(),
            hero_to: "#1e293b".into(),
            header_text: "#ffffff".into(),
        }
    }

    fn base(id: ThemeId) -> Self {
        match id {
            ThemeId::Forest => Self::forest(),
            ThemeId::Midnight => Self::midnight(),
            ThemeId::Ocean => Self::ocean(),
            ThemeId::Paper => Self::paper(),
            ThemeId::Rose => Self::rose(),
            ThemeId::Slate => Self::slate(),
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
    ThemePalette::base(ThemeId::parse(&plugins.theme)).apply_overrides(brand)
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
        assert_eq!(pal.id, ThemeId::Forest);
        assert_eq!(pal.accent, "#15624f");
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
        assert_eq!(pal.id, ThemeId::Ocean);
        assert_eq!(pal.accent, "#ff00aa");
        assert_eq!(pal.hero_from, "#0e7490");
    }
}
