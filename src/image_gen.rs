use crate::colors::ColorScheme;
use ab_glyph::{Font, FontRef, PxScale};
use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;

pub struct ImageGenerator {
    width: u32,
    height: u32,
}

impl ImageGenerator {
    pub fn new(width: u32, height: u32) -> Self {
        ImageGenerator { width, height }
    }

    pub fn generate(
        &self,
        ascii_text: &str,
        color_scheme: ColorScheme,
        output_path: &str,
        layout: &str,
        font_size: f32,
        use_gradient: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let bg_rgb = color_to_rgb(color_scheme.bg);
        let fg_rgb = color_to_rgb(color_scheme.fg);
        let accent_rgb = color_to_rgb(color_scheme.accent);

        let mut img: RgbImage = ImageBuffer::from_pixel(self.width, self.height, bg_rgb);

        // Use system monospace font to render ASCII art
        let font_data = std::fs::read("/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf")
            .or_else(|_| {
                std::fs::read("/usr/share/fonts/truetype/liberation/LiberationMono-Regular.ttf")
            })
            .or_else(|_| std::fs::read("/System/Library/Fonts/Monaco.ttf"))
            .map_err(|_| "Could not find a suitable monospace font on the system")?;
        let font = FontRef::try_from_slice(&font_data)?;

        // Font size
        let scale = PxScale::from(font_size);

        let lines: Vec<&str> = ascii_text.lines().collect();

        // Calculate actual text dimensions
        let line_height = (font_size * 1.2) as i32;

        // Measure the width of the longest line
        let mut max_width = 0i32;
        for line in &lines {
            let mut width = 0.0f32;
            for ch in line.chars() {
                let glyph_id = font.glyph_id(ch);
                width += font.h_advance_unscaled(glyph_id) * font_size
                    / font.units_per_em().unwrap_or(1000.0);
            }
            max_width = max_width.max(width as i32);
        }

        let content_width = max_width;
        let content_height = (lines.len() as i32) * line_height;

        // Calculate offset based on layout mode
        let (x_offset, y_offset) = match layout {
            "vertical" => {
                // Vertical layout: left-aligned, vertically centered
                let x = 50;
                let y = ((self.height as i32 - content_height) / 2).max(20);
                (x, y)
            }
            "horizontal" | _ => {
                // Horizontal layout: fully centered (default)
                let x = ((self.width as i32 - content_width) / 2).max(20);
                let y = ((self.height as i32 - content_height) / 2).max(20);
                (x, y)
            }
        };

        // Render each line
        for (line_idx, line) in lines.iter().enumerate() {
            let y = y_offset + (line_idx as i32 * line_height);

            // Choose color based on gradient setting
            let color = if use_gradient {
                // Gradient effect: choose color based on line position
                let progress = line_idx as f32 / lines.len().max(1) as f32;
                if progress < 0.3 {
                    accent_rgb
                } else {
                    fg_rgb
                }
            } else {
                // Solid color: use foreground color uniformly
                fg_rgb
            };

            draw_text_mut(&mut img, color, x_offset, y, scale, &font, line);
        }

        img.save(output_path)?;
        Ok(())
    }
}

fn color_to_rgb(color: ratatui::style::Color) -> Rgb<u8> {
    use ratatui::style::Color;
    match color {
        Color::Black => Rgb([0, 0, 0]),
        Color::Red => Rgb([255, 0, 0]),
        Color::Green => Rgb([0, 255, 0]),
        Color::Yellow => Rgb([255, 255, 0]),
        Color::Blue => Rgb([0, 0, 255]),
        Color::Magenta => Rgb([255, 0, 255]),
        Color::Cyan => Rgb([0, 255, 255]),
        Color::White => Rgb([255, 255, 255]),
        Color::Gray => Rgb([128, 128, 128]),
        Color::DarkGray => Rgb([64, 64, 64]),
        Color::LightRed => Rgb([255, 128, 128]),
        Color::LightGreen => Rgb([128, 255, 128]),
        Color::LightYellow => Rgb([255, 255, 128]),
        Color::LightBlue => Rgb([128, 128, 255]),
        Color::LightMagenta => Rgb([255, 128, 255]),
        Color::LightCyan => Rgb([128, 255, 255]),
        Color::Rgb(r, g, b) => Rgb([r, g, b]),
        _ => Rgb([255, 255, 255]),
    }
}
