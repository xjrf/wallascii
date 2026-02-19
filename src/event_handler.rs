use crate::app::{App, Focus, ResolutionFocus};
use crate::image_gen::ImageGenerator;
use crossterm::event::KeyCode;

pub fn handle_key_event(app: &mut App, key_code: KeyCode) -> bool {
    match key_code {
        KeyCode::Esc => return true, // Signal to quit
        KeyCode::Tab => handle_tab(app),
        KeyCode::Enter => handle_enter(app),
        KeyCode::Up => handle_up(app),
        KeyCode::Down => handle_down(app),
        KeyCode::Left => handle_left(app),
        KeyCode::Right => handle_right(app),
        KeyCode::Char(c) => {
            // Handle shortcuts only when NOT in text/resolution input mode
            if app.focus != Focus::Text && app.focus != Focus::Resolution {
                match c {
                    'q' => return true, // Quit
                    'l' => handle_layout_toggle(app),
                    'g' => handle_gradient_toggle(app),
                    'p' => handle_external_preview(app),
                    _ => {}
                }
            } else {
                // In text/resolution mode, handle as character input
                handle_char_input(app, c);
            }
        }
        KeyCode::Backspace => handle_backspace(app),
        _ => {}
    }
    false // Continue running
}

fn handle_tab(app: &mut App) {
    app.focus = match app.focus {
        Focus::Text => Focus::Font,
        Focus::Font => Focus::Color,
        Focus::Color => Focus::FontSize,
        Focus::FontSize => Focus::Resolution,
        Focus::Resolution => Focus::Text,
    };
}

fn handle_left(app: &mut App) {
    if app.focus == Focus::Resolution {
        app.resolution_focus = ResolutionFocus::Width;
    } else {
        // Left arrow also switches focus backwards
        app.focus = match app.focus {
            Focus::Text => Focus::Resolution,
            Focus::Font => Focus::Text,
            Focus::Color => Focus::Font,
            Focus::FontSize => Focus::Color,
            Focus::Resolution => Focus::FontSize,
        };
    }
}

fn handle_right(app: &mut App) {
    if app.focus == Focus::Resolution {
        app.resolution_focus = ResolutionFocus::Height;
    } else {
        // Right arrow also switches focus forwards
        app.focus = match app.focus {
            Focus::Text => Focus::Font,
            Focus::Font => Focus::Color,
            Focus::Color => Focus::FontSize,
            Focus::FontSize => Focus::Resolution,
            Focus::Resolution => Focus::Text,
        };
    }
}

fn handle_enter(app: &mut App) {
    if let Err(e) = app.generate_wallpaper() {
        app.status_message = format!("✗ Generation failed: {}", e);
    }
}

fn handle_layout_toggle(app: &mut App) {
    app.toggle_layout();
    app.update_preview();
}

fn handle_gradient_toggle(app: &mut App) {
    app.use_gradient = !app.use_gradient;
    app.update_preview();
}

fn handle_external_preview(app: &mut App) {
    // Ensure resolution is up to date
    app.update_resolution_from_input();

    let img_gen = ImageGenerator::new(app.output_width, app.output_height);
    let color_scheme = app.available_colors[app.selected_color];
    let preview_path = "/tmp/ascii_preview_external.png";

    if let Ok(_) = img_gen.generate(
        &app.preview,
        color_scheme,
        preview_path,
        &app.layout,
        app.font_size,
        app.use_gradient,
    ) {
        if let Ok(_) = std::process::Command::new("xdg-open")
            .arg(preview_path)
            .spawn()
        {
            app.status_message = format!(
                "✓ PNG preview opened ({}x{})",
                app.output_width, app.output_height
            );
        } else {
            app.status_message = "✗ Cannot open image viewer".to_string();
        }
    } else {
        app.status_message = "✗ Preview generation failed".to_string();
    }
}

fn handle_up(app: &mut App) {
    match app.focus {
        Focus::Font => {
            if app.selected_font > 0 {
                app.selected_font -= 1;
                app.update_preview();
            }
        }
        Focus::Color => {
            if app.selected_color > 0 {
                app.selected_color -= 1;
                app.update_preview();
            }
        }
        Focus::FontSize => {
            if app.font_size < 48.0 {
                app.font_size += 2.0;
                app.update_preview();
            }
        }
        _ => {}
    }
}

fn handle_down(app: &mut App) {
    match app.focus {
        Focus::Font => {
            if app.selected_font < app.available_fonts.len() - 1 {
                app.selected_font += 1;
                app.update_preview();
            }
        }
        Focus::Color => {
            if app.selected_color < app.available_colors.len() - 1 {
                app.selected_color += 1;
                app.update_preview();
            }
        }
        Focus::FontSize => {
            if app.font_size > 8.0 {
                app.font_size -= 2.0;
                app.update_preview();
            }
        }
        _ => {}
    }
}

fn handle_char_input(app: &mut App, c: char) {
    match app.focus {
        Focus::Text => {
            app.input_text.push(c);
            app.update_preview();
            app.status_message.clear();
        }
        Focus::Resolution => {
            if c.is_ascii_digit() {
                if app.resolution_focus == ResolutionFocus::Width {
                    app.width_input.push(c);
                    app.update_resolution_from_input();
                } else {
                    app.height_input.push(c);
                    app.update_resolution_from_input();
                }
            }
        }
        _ => {}
    }
}

fn handle_backspace(app: &mut App) {
    match app.focus {
        Focus::Text => {
            app.input_text.pop();
            app.update_preview();
            app.status_message.clear();
        }
        Focus::Resolution => {
            if app.resolution_focus == ResolutionFocus::Width {
                app.width_input.pop();
                app.update_resolution_from_input();
            } else {
                app.height_input.pop();
                app.update_resolution_from_input();
            }
        }
        _ => {}
    }
}
