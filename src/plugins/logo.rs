/// How the brand mark is produced. Operators pick one with `plugins.logo`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogoMode {
    Auto,
    None,
    Monogram,
    Image,
}

impl LogoMode {
    pub fn parse(raw: &str) -> Self {
        match raw.trim().to_ascii_lowercase().as_str() {
            "none" | "off" | "false" | "hidden" => Self::None,
            "monogram" | "letter" | "initial" | "badge" => Self::Monogram,
            "image" | "file" | "img" => Self::Image,
            _ => Self::Auto,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::None => "none",
            Self::Monogram => "monogram",
            Self::Image => "image",
        }
    }

    pub fn uses_image(self) -> bool {
        matches!(self, Self::Auto | Self::Image)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_logo_modes() {
        assert_eq!(LogoMode::parse(""), LogoMode::Auto);
        assert_eq!(LogoMode::parse("IMAGE"), LogoMode::Image);
        assert_eq!(LogoMode::parse("none"), LogoMode::None);
        assert_eq!(LogoMode::parse("letter"), LogoMode::Monogram);
    }
}
