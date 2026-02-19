/// Font management module
/// Centralized management of all available ASCII fonts

pub const AVAILABLE_FONTS: &[&str] = &[
    "3d",
    "banner",
    "big",
    "block",
    "Bloody",
    "Broadway",
    "Bulbhead",
    "Caligraphy",
    "Chunky",
    "colossal",
    "Colossal",
    "Crawford2",
    "Cursive",
    "Cyberlarge",
    "Cybermedium",
    "Cybersmall",
    "doom",
    "Double",
    "Efti Chess",
    "Efti Font",
    "Efti Italic",
    "Efti Piti",
    "Efti Robot",
    "Efti Wall",
    "Efti Water",
    "Electronic",
    "Elite",
    "epic",
    "Graceful",
    "graffiti",
    "Graffiti",
    "Greek",
    "Hollywood",
    "isometric1",
    "Isometric1",
    "isometric2",
    "Isometric2",
    "isometric3",
    "Isometric3",
    "Isometric4",
    "Ivrit",
    "Jacky",
    "Jazmine",
    "Jerusalem",
    "Katakana",
    "Kban",
    "Keyboard",
    "larry3d",
    "LCD",
    "lean",
    "Lean",
    "Letters",
    "Linux",
    "Lockergnome",
    "Madrid",
    "Marquee",
    "Maxfour",
    "Mini",
    "Mirror",
    "Moscow",
    "Mshebrew210",
    "Nancyj",
    "Nancyj-Fancy",
    "Nancyj-Improved",
    "Nancyj-Underlined",
    "O8",
    "ogre",
    "Ogre",
    "OS2",
    "Pawp",
    "Peaks",
    "Peaks Slant",
    "Pebbles",
    "Pepper",
    "Poison",
    "Puffy",
    "Rectangles",
    "Runic",
    "Runyc",
    "script",
    "Serifcap",
    "shadow",
    "Shadow",
    "Shimrod",
    "Short",
    "slant",
    "Slant",
    "Slant Relief",
    "small",
    "Small",
    "Small Isometric1",
    "Small Keyboard",
    "Small Poison",
    "Small Script",
    "Small Shadow",
    "Small Slant",
    "Small Tengwar",
    "Speed",
    "Stacey",
    "Stampate",
    "Stampatello",
    "standard",
    "Standard",
    "starwars",
    "Star Wars",
    "Stellar",
    "Straight",
    "Sub-Zero",
    "Tanja",
    "Tengwar",
    "Thick",
    "Thin",
    "THIS",
    "Three Point",
    "Ticks",
    "Ticks Slant",
    "Tinker-Toy",
    "Tombstone",
    "Trek",
    "Tubular",
    "Two Point",
    "Univers",
    "Wavy",
    "Weird",
];

/// Check if font exists
pub fn is_valid_font(name: &str) -> bool {
    AVAILABLE_FONTS.contains(&name)
}

/// Get all fonts list
pub fn get_all_fonts() -> Vec<String> {
    AVAILABLE_FONTS.iter().map(|&s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_fonts() {
        let fonts = get_all_fonts();
        assert!(!fonts.is_empty());
        assert!(fonts.contains(&"standard".to_string()));
        assert!(fonts.contains(&"banner".to_string()));
        assert_eq!(fonts.len(), AVAILABLE_FONTS.len());
    }

    #[test]
    fn test_is_valid_font() {
        assert!(is_valid_font("standard"));
        assert!(is_valid_font("banner"));
        assert!(is_valid_font("isometric1"));
        assert!(!is_valid_font("nonexistent"));
        assert!(!is_valid_font(""));
    }

    #[test]
    fn test_available_fonts_count() {
        assert!(AVAILABLE_FONTS.len() >= 19);
    }
}
