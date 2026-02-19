use crate::ascii_gen::AsciiGenerator;
use crate::colors::ColorScheme;
use crate::fonts;
use crate::image_gen::ImageGenerator;

pub struct App {
    pub input_text: String,
    pub selected_font: usize,
    pub available_fonts: Vec<String>,
    pub selected_color: usize,
    pub available_colors: Vec<ColorScheme>,
    pub preview: String,
    pub layout: String,
    pub focus: Focus,
    pub status_message: String,
    pub font_size: f32,
    pub use_gradient: bool,
    pub output_width: u32,
    pub output_height: u32,
    pub width_input: String,
    pub height_input: String,
    pub resolution_focus: ResolutionFocus,
}

#[derive(PartialEq)]
pub enum Focus {
    Text,
    Font,
    Color,
    FontSize,
    Resolution,
}

#[derive(PartialEq)]
pub enum ResolutionFocus {
    Width,
    Height,
}

impl App {
    pub fn new() -> App {
        let available_fonts = fonts::get_all_fonts();
        let available_colors = ColorScheme::get_all();

        let mut app = App {
            input_text: String::from("Hello"),
            selected_font: 0,
            available_fonts,
            selected_color: 0,
            available_colors,
            preview: String::new(),
            layout: "horizontal".to_string(),
            focus: Focus::Text,
            status_message: String::new(),
            font_size: 16.0,
            use_gradient: false,
            output_width: 1920,
            output_height: 1080,
            width_input: String::from("1920"),
            height_input: String::from("1080"),
            resolution_focus: ResolutionFocus::Width,
        };

        app.update_preview();
        app
    }

    pub fn generate_filename(&self) -> String {
        let text = if self.input_text.len() > 20 {
            &self.input_text[..20]
        } else {
            &self.input_text
        };
        let text = text.replace(" ", "-").replace("/", "-").replace("\\", "-");
        let font = &self.available_fonts[self.selected_font];
        let theme = self.available_colors[self.selected_color].name;
        let size = self.font_size as u32;
        format!("{}-{}-{}-{}.png", text, font, theme, size)
    }

    pub fn validate_input(&self) -> Result<(), String> {
        if self.input_text.trim().is_empty() {
            return Err("Text cannot be empty".to_string());
        }
        if self.input_text.len() > 100 {
            return Err("Text too long (max 100 characters)".to_string());
        }
        Ok(())
    }

    pub fn update_preview(&mut self) {
        let generator = AsciiGenerator::new();
        self.preview =
            generator.generate(&self.input_text, &self.available_fonts[self.selected_font]);
    }

    pub fn generate_wallpaper(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Validate input
        if let Err(msg) = self.validate_input() {
            self.status_message = format!("✗ {}", msg);
            return Ok(());
        }

        // Ensure resolution is up to date
        self.update_resolution_from_input();

        let img_gen = ImageGenerator::new(self.output_width, self.output_height);
        let color_scheme = self.available_colors[self.selected_color];
        let filename = self.generate_filename();
        img_gen.generate(
            &self.preview,
            color_scheme,
            &filename,
            &self.layout,
            self.font_size,
            self.use_gradient,
        )?;
        self.status_message = format!(
            "✓ Saved to {} ({}x{})",
            filename, self.output_width, self.output_height
        );
        Ok(())
    }

    pub fn toggle_layout(&mut self) {
        self.layout = if self.layout == "horizontal" {
            "vertical".to_string()
        } else {
            "horizontal".to_string()
        };
    }

    pub fn update_resolution_from_input(&mut self) {
        // Only update if input is valid, don't reset on invalid input
        if let Ok(width) = self.width_input.parse::<u32>() {
            if width >= 640 && width <= 7680 {
                self.output_width = width;
            }
        }

        if let Ok(height) = self.height_input.parse::<u32>() {
            if height >= 480 && height <= 4320 {
                self.output_height = height;
            }
        }
    }
}
