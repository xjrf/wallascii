use retrofont::{Font, RenderOptions};

pub struct AsciiGenerator;

impl AsciiGenerator {
    pub fn new() -> Self {
        AsciiGenerator
    }

    pub fn generate(&self, text: &str, font_name: &str) -> String {
        // Try multiple possible paths
        let cargo_manifest_dir = env!("CARGO_MANIFEST_DIR");
        let possible_paths = vec![
            format!("{}/fonts/{}.flf", cargo_manifest_dir, font_name),
            format!("fonts/{}.flf", font_name),
            format!("./fonts/{}.flf", font_name),
        ];

        let mut font_data = None;
        for path in &possible_paths {
            if let Ok(data) = std::fs::read(path) {
                font_data = Some(data);
                break;
            }
        }

        let font_data = match font_data {
            Some(data) => data,
            None => {
                return format!("Font not found: {}", font_name);
            }
        };

        match Font::load(&font_data) {
            Ok(fonts) if !fonts.is_empty() => {
                let font = &fonts[0];
                let options = RenderOptions::default();

                // Render each character separately
                let mut char_renders: Vec<Vec<String>> = Vec::new();

                for ch in text.chars() {
                    let mut target = StringTarget::new();
                    if font.render_glyph(&mut target, ch, &options).is_ok() {
                        // Ensure the last line is added
                        target.finalize();
                        char_renders.push(target.lines);
                    } else {
                        // If character can't be rendered, try to use a space
                        char_renders.push(vec![" ".to_string()]);
                    }
                }

                if char_renders.is_empty() {
                    return String::from("Cannot render text");
                }

                // Horizontally merge all characters
                self.merge_horizontal(char_renders)
            }
            Ok(_) => format!("Font file is empty: {}", font_name),
            Err(e) => format!("Failed to load font {}: {:?}", font_name, e),
        }
    }

    fn merge_horizontal(&self, char_renders: Vec<Vec<String>>) -> String {
        if char_renders.is_empty() {
            return String::new();
        }

        // Find maximum height
        let max_height = char_renders
            .iter()
            .map(|lines| lines.len())
            .max()
            .unwrap_or(0);

        let mut result_lines: Vec<String> = vec![String::new(); max_height];

        // Horizontally concatenate each character
        for char_lines in char_renders {
            let char_height = char_lines.len();

            for (i, result_line) in result_lines.iter_mut().enumerate() {
                if i < char_height {
                    result_line.push_str(&char_lines[i]);
                } else {
                    // Fill with spaces to maintain alignment
                    if !char_lines.is_empty() {
                        let width = char_lines[0].chars().count();
                        result_line.push_str(&" ".repeat(width));
                    }
                }
            }
        }

        result_lines.join("\n")
    }
}

struct StringTarget {
    lines: Vec<String>,
    current_line: String,
}

impl StringTarget {
    fn new() -> Self {
        StringTarget {
            lines: Vec::new(),
            current_line: String::new(),
        }
    }

    fn finalize(&mut self) {
        // Add the last line if it's not empty
        if !self.current_line.is_empty() {
            self.lines.push(std::mem::take(&mut self.current_line));
        }
    }
}

impl retrofont::FontTarget for StringTarget {
    type Error = std::io::Error;

    fn draw(&mut self, cell: retrofont::Cell) -> Result<(), Self::Error> {
        self.current_line.push(cell.ch);
        Ok(())
    }

    fn next_line(&mut self) -> Result<(), Self::Error> {
        self.lines.push(std::mem::take(&mut self.current_line));
        Ok(())
    }
}
