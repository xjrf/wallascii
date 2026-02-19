use ratatui::style::Color;

#[derive(Clone, Copy)]
pub struct ColorScheme {
    pub name: &'static str,
    pub bg: Color,
    pub fg: Color,
    pub accent: Color,
}

impl ColorScheme {
    pub fn get_all() -> Vec<ColorScheme> {
        vec![
            Self::nord(),
            Self::gruvbox_dark(),
            Self::gruvbox_light(),
            Self::everforest_dark(),
            Self::everforest_light(),
            Self::dracula(),
            Self::tokyo_night(),
            Self::catppuccin_mocha(),
            Self::catppuccin_latte(),
            Self::catppuccin_frappe(),
            Self::catppuccin_macchiato(),
            Self::solarized_dark(),
            Self::solarized_light(),
            Self::monokai(),
            Self::one_dark(),
            Self::one_light(),
            Self::material_dark(),
            Self::material_light(),
            Self::ayu_dark(),
            Self::ayu_light(),
            Self::ayu_mirage(),
            Self::rose_pine(),
            Self::rose_pine_moon(),
            Self::rose_pine_dawn(),
            Self::github_dark(),
            Self::github_light(),
            Self::night_owl(),
            Self::light_owl(),
            Self::palenight(),
            Self::oceanic_next(),
            Self::cobalt2(),
            Self::synthwave84(),
            Self::cyberpunk(),
            Self::horizon(),
            Self::moonlight(),
        ]
    }

    #[allow(dead_code)]
    pub fn from_name(name: &str) -> Option<ColorScheme> {
        Self::get_all().into_iter().find(|s| s.name == name)
    }

    pub fn nord() -> Self {
        ColorScheme {
            name: "nord",
            bg: Color::Rgb(46, 52, 64),        // #2E3440
            fg: Color::Rgb(216, 222, 233),     // #D8DEE9
            accent: Color::Rgb(136, 192, 208), // #88C0D0
        }
    }

    pub fn gruvbox_dark() -> Self {
        ColorScheme {
            name: "gruvbox-dark",
            bg: Color::Rgb(40, 40, 40),       // #282828
            fg: Color::Rgb(235, 219, 178),    // #EBDBB2
            accent: Color::Rgb(184, 187, 38), // #B8BB26
        }
    }

    pub fn gruvbox_light() -> Self {
        ColorScheme {
            name: "gruvbox-light",
            bg: Color::Rgb(251, 241, 199),  // #FBF1C7
            fg: Color::Rgb(60, 56, 54),     // #3C3836
            accent: Color::Rgb(175, 58, 3), // #AF3A03
        }
    }

    pub fn everforest_dark() -> Self {
        ColorScheme {
            name: "everforest-dark",
            bg: Color::Rgb(45, 50, 48),        // #2D3234
            fg: Color::Rgb(211, 198, 170),     // #D3C6AA
            accent: Color::Rgb(167, 192, 128), // #A7C080
        }
    }

    pub fn everforest_light() -> Self {
        ColorScheme {
            name: "everforest-light",
            bg: Color::Rgb(253, 244, 237),    // #FDF4ED
            fg: Color::Rgb(92, 95, 119),      // #5C5F77
            accent: Color::Rgb(140, 143, 82), // #8C8F52
        }
    }

    pub fn dracula() -> Self {
        ColorScheme {
            name: "dracula",
            bg: Color::Rgb(40, 42, 54),        // #282A36
            fg: Color::Rgb(248, 248, 242),     // #F8F8F2
            accent: Color::Rgb(189, 147, 249), // #BD93F9
        }
    }

    pub fn tokyo_night() -> Self {
        ColorScheme {
            name: "tokyo-night",
            bg: Color::Rgb(26, 27, 38),        // #1A1B26
            fg: Color::Rgb(192, 202, 245),     // #C0CAF5
            accent: Color::Rgb(125, 207, 255), // #7DCFFF
        }
    }

    pub fn catppuccin_mocha() -> Self {
        ColorScheme {
            name: "catppuccin-mocha",
            bg: Color::Rgb(30, 30, 46),        // #1E1E2E
            fg: Color::Rgb(205, 214, 244),     // #CDD6F4
            accent: Color::Rgb(137, 180, 250), // #89B4FA
        }
    }

    pub fn solarized_dark() -> Self {
        ColorScheme {
            name: "solarized-dark",
            bg: Color::Rgb(0, 43, 54),        // #002B36
            fg: Color::Rgb(131, 148, 150),    // #839496
            accent: Color::Rgb(42, 161, 152), // #2AA198
        }
    }

    pub fn solarized_light() -> Self {
        ColorScheme {
            name: "solarized-light",
            bg: Color::Rgb(253, 246, 227),    // #FDF6E3
            fg: Color::Rgb(101, 123, 131),    // #657B83
            accent: Color::Rgb(38, 139, 210), // #268BD2
        }
    }

    pub fn catppuccin_latte() -> Self {
        ColorScheme {
            name: "catppuccin-latte",
            bg: Color::Rgb(239, 241, 245),    // #EFF1F5
            fg: Color::Rgb(76, 79, 105),      // #4C4F69
            accent: Color::Rgb(30, 102, 245), // #1E66F5
        }
    }

    pub fn catppuccin_frappe() -> Self {
        ColorScheme {
            name: "catppuccin-frappe",
            bg: Color::Rgb(48, 52, 70),        // #303446
            fg: Color::Rgb(198, 208, 245),     // #C6D0F5
            accent: Color::Rgb(140, 170, 238), // #8CAAEE
        }
    }

    pub fn catppuccin_macchiato() -> Self {
        ColorScheme {
            name: "catppuccin-macchiato",
            bg: Color::Rgb(36, 39, 58),        // #24273A
            fg: Color::Rgb(202, 211, 245),     // #CAD3F5
            accent: Color::Rgb(138, 173, 244), // #8AADF4
        }
    }

    pub fn monokai() -> Self {
        ColorScheme {
            name: "monokai",
            bg: Color::Rgb(39, 40, 34),        // #272822
            fg: Color::Rgb(248, 248, 242),     // #F8F8F2
            accent: Color::Rgb(102, 217, 239), // #66D9EF
        }
    }

    pub fn one_dark() -> Self {
        ColorScheme {
            name: "one-dark",
            bg: Color::Rgb(40, 44, 52),       // #282C34
            fg: Color::Rgb(171, 178, 191),    // #ABB2BF
            accent: Color::Rgb(97, 175, 239), // #61AFEF
        }
    }

    pub fn one_light() -> Self {
        ColorScheme {
            name: "one-light",
            bg: Color::Rgb(250, 250, 250),    // #FAFAFA
            fg: Color::Rgb(56, 58, 66),       // #383A42
            accent: Color::Rgb(64, 120, 242), // #4078F2
        }
    }

    pub fn material_dark() -> Self {
        ColorScheme {
            name: "material-dark",
            bg: Color::Rgb(38, 50, 56),        // #263238
            fg: Color::Rgb(236, 239, 244),     // #ECEFF4
            accent: Color::Rgb(128, 203, 196), // #80CBC4
        }
    }

    pub fn material_light() -> Self {
        ColorScheme {
            name: "material-light",
            bg: Color::Rgb(250, 250, 250),   // #FAFAFA
            fg: Color::Rgb(66, 66, 66),      // #424242
            accent: Color::Rgb(0, 150, 136), // #009688
        }
    }

    pub fn ayu_dark() -> Self {
        ColorScheme {
            name: "ayu-dark",
            bg: Color::Rgb(10, 14, 20),       // #0A0E14
            fg: Color::Rgb(230, 237, 243),    // #E6EDF3
            accent: Color::Rgb(89, 184, 255), // #59B8FF
        }
    }

    pub fn ayu_light() -> Self {
        ColorScheme {
            name: "ayu-light",
            bg: Color::Rgb(250, 250, 250),    // #FAFAFA
            fg: Color::Rgb(95, 99, 104),      // #5F6368
            accent: Color::Rgb(85, 181, 219), // #55B5DB
        }
    }

    pub fn ayu_mirage() -> Self {
        ColorScheme {
            name: "ayu-mirage",
            bg: Color::Rgb(31, 35, 41),       // #1F2329
            fg: Color::Rgb(203, 204, 198),    // #CBCCC6
            accent: Color::Rgb(95, 191, 227), // #5FBFE3
        }
    }

    pub fn rose_pine() -> Self {
        ColorScheme {
            name: "rose-pine",
            bg: Color::Rgb(25, 23, 36),       // #191724
            fg: Color::Rgb(224, 222, 244),    // #E0DEF4
            accent: Color::Rgb(49, 116, 143), // #31748F
        }
    }

    pub fn rose_pine_moon() -> Self {
        ColorScheme {
            name: "rose-pine-moon",
            bg: Color::Rgb(35, 33, 54),       // #232136
            fg: Color::Rgb(224, 222, 244),    // #E0DEF4
            accent: Color::Rgb(60, 141, 188), // #3C8DBC
        }
    }

    pub fn rose_pine_dawn() -> Self {
        ColorScheme {
            name: "rose-pine-dawn",
            bg: Color::Rgb(250, 244, 237),    // #FAF4ED
            fg: Color::Rgb(87, 82, 121),      // #575279
            accent: Color::Rgb(40, 105, 131), // #286983
        }
    }

    pub fn github_dark() -> Self {
        ColorScheme {
            name: "github-dark",
            bg: Color::Rgb(13, 17, 23),       // #0D1117
            fg: Color::Rgb(201, 209, 217),    // #C9D1D9
            accent: Color::Rgb(88, 166, 255), // #58A6FF
        }
    }

    pub fn github_light() -> Self {
        ColorScheme {
            name: "github-light",
            bg: Color::Rgb(255, 255, 255),   // #FFFFFF
            fg: Color::Rgb(36, 41, 47),      // #24292F
            accent: Color::Rgb(9, 105, 218), // #0969DA
        }
    }

    pub fn night_owl() -> Self {
        ColorScheme {
            name: "night-owl",
            bg: Color::Rgb(1, 22, 39),         // #011627
            fg: Color::Rgb(214, 222, 235),     // #D6DEEB
            accent: Color::Rgb(127, 219, 202), // #7FDBCA
        }
    }

    pub fn light_owl() -> Self {
        ColorScheme {
            name: "light-owl",
            bg: Color::Rgb(251, 251, 251),   // #FBFBFB
            fg: Color::Rgb(64, 64, 64),      // #403F53
            accent: Color::Rgb(0, 139, 148), // #008B94
        }
    }

    pub fn palenight() -> Self {
        ColorScheme {
            name: "palenight",
            bg: Color::Rgb(41, 45, 62),        // #292D3E
            fg: Color::Rgb(169, 175, 214),     // #A9B7D6
            accent: Color::Rgb(130, 170, 255), // #82AAFF
        }
    }

    pub fn oceanic_next() -> Self {
        ColorScheme {
            name: "oceanic-next",
            bg: Color::Rgb(27, 43, 52),        // #1B2B34
            fg: Color::Rgb(192, 197, 206),     // #C0C5CE
            accent: Color::Rgb(101, 198, 187), // #65C6BB
        }
    }

    pub fn cobalt2() -> Self {
        ColorScheme {
            name: "cobalt2",
            bg: Color::Rgb(19, 30, 41),      // #131E29
            fg: Color::Rgb(255, 255, 255),   // #FFFFFF
            accent: Color::Rgb(0, 199, 255), // #00C7FF
        }
    }

    pub fn synthwave84() -> Self {
        ColorScheme {
            name: "synthwave84",
            bg: Color::Rgb(38, 24, 61),      // #26183D
            fg: Color::Rgb(255, 255, 255),   // #FFFFFF
            accent: Color::Rgb(255, 0, 128), // #FF0080
        }
    }

    pub fn cyberpunk() -> Self {
        ColorScheme {
            name: "cyberpunk",
            bg: Color::Rgb(0, 16, 38),       // #001026
            fg: Color::Rgb(0, 255, 255),     // #00FFFF
            accent: Color::Rgb(255, 0, 255), // #FF00FF
        }
    }

    pub fn horizon() -> Self {
        ColorScheme {
            name: "horizon",
            bg: Color::Rgb(28, 32, 41),      // #1C2029
            fg: Color::Rgb(203, 204, 198),   // #CBCCC6
            accent: Color::Rgb(233, 89, 80), // #E95950
        }
    }

    pub fn moonlight() -> Self {
        ColorScheme {
            name: "moonlight",
            bg: Color::Rgb(34, 38, 60),        // #222436
            fg: Color::Rgb(195, 199, 245),     // #C3C7F5
            accent: Color::Rgb(130, 170, 255), // #82AAFF
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_scheme_count() {
        let schemes = ColorScheme::get_all();
        assert!(schemes.len() >= 10);
    }

    #[test]
    fn test_from_name() {
        assert!(ColorScheme::from_name("nord").is_some());
        assert!(ColorScheme::from_name("dracula").is_some());
        assert!(ColorScheme::from_name("nonexistent").is_none());
    }

    #[test]
    fn test_nord_colors() {
        let nord = ColorScheme::nord();
        assert_eq!(nord.name, "nord");
    }

    #[test]
    fn test_all_schemes_have_unique_names() {
        let schemes = ColorScheme::get_all();
        let names: Vec<&str> = schemes.iter().map(|s| s.name).collect();
        let mut unique_names = names.clone();
        unique_names.sort();
        unique_names.dedup();
        assert_eq!(names.len(), unique_names.len());
    }
}
