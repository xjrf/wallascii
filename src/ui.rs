use crate::app::{App, Focus, ResolutionFocus};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn render(f: &mut Frame, app: &mut App) {
    // Main layout: top, middle, bottom split
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(55), // Top: preview and shortcuts
            Constraint::Percentage(35), // Middle: configuration options
            Constraint::Length(3),      // Bottom: resolution and output
        ])
        .split(f.area());

    // Top section: left-right split (preview | shortcuts)
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70), // Left: preview
            Constraint::Percentage(30), // Right: shortcuts
        ])
        .split(main_chunks[0]);

    // Get current theme colors
    let color_scheme = app.available_colors[app.selected_color];
    let theme_fg = color_scheme.fg;
    let theme_accent = color_scheme.accent;

    render_preview(f, app, top_chunks[0], theme_fg);
    render_shortcuts(f, top_chunks[1], theme_accent);

    // Middle: Configuration options (4 columns)
    let config_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25), // Input text
            Constraint::Percentage(25), // Font
            Constraint::Percentage(25), // Theme
            Constraint::Percentage(25), // Font size
        ])
        .split(main_chunks[1]);

    render_input(f, app, config_chunks[0], theme_accent);
    render_font_list(f, app, config_chunks[1], theme_accent);
    render_color_list(f, app, config_chunks[2], theme_accent);
    render_font_size(f, app, config_chunks[3], theme_accent);

    // Bottom: left-right split (resolution | output)
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // Left: resolution
            Constraint::Percentage(70), // Right: output filename
        ])
        .split(main_chunks[2]);

    render_resolution(f, app, bottom_chunks[0], theme_accent);
    render_output(f, app, bottom_chunks[1], theme_fg, theme_accent);
}

fn render_preview(f: &mut Frame, app: &App, area: ratatui::layout::Rect, theme_fg: Color) {
    let gradient_indicator = if app.use_gradient {
        "Gradient"
    } else {
        "Solid"
    };
    let color_scheme = app.available_colors[app.selected_color];

    let preview = Paragraph::new(app.preview.clone())
        .block(
            Block::default()
                .title(format!(
                    " Preview - {} | {} | {} | {}px ",
                    color_scheme.name, app.layout, gradient_indicator, app.font_size
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme_fg)),
        )
        .wrap(Wrap { trim: false });
    f.render_widget(preview, area);
}

fn render_shortcuts(f: &mut Frame, area: ratatui::layout::Rect, theme_accent: Color) {
    let shortcuts_text = vec![
        "Shortcuts",
        "",
        "[Tab/←→] Switch focus",
        "[Up/Down] Navigate",
        "[Enter] Generate",
        "",
        "[L] Toggle layout",
        "[G] Toggle gradient",
        "[P] External preview",
        "",
        "[Q/Esc] Quit",
    ]
    .join("\n");

    let shortcuts = Paragraph::new(shortcuts_text)
        .block(
            Block::default()
                .title(" Shortcuts ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme_accent)),
        )
        .style(Style::default().fg(theme_accent));
    f.render_widget(shortcuts, area);
}

fn render_input(f: &mut Frame, app: &App, area: ratatui::layout::Rect, theme_accent: Color) {
    let input_style = if app.focus == Focus::Text {
        Style::default()
            .fg(theme_accent)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let display_text = if app.focus == Focus::Text {
        format!("{}█", app.input_text) // Add cursor
    } else {
        app.input_text.clone()
    };

    let input = Paragraph::new(display_text.as_str())
        .style(input_style)
        .block(
            Block::default()
                .title(" Text [1] ")
                .borders(Borders::ALL)
                .border_style(if app.focus == Focus::Text {
                    Style::default().fg(theme_accent)
                } else {
                    Style::default()
                }),
        );
    f.render_widget(input, area);
}

fn render_font_list(f: &mut Frame, app: &App, area: ratatui::layout::Rect, theme_accent: Color) {
    let font_items: Vec<ListItem> = app
        .available_fonts
        .iter()
        .enumerate()
        .map(|(i, font)| {
            let style = if i == app.selected_font {
                Style::default()
                    .fg(theme_accent)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(font.as_str()).style(style)
        })
        .collect();

    let mut fonts_list_state = ratatui::widgets::ListState::default();
    fonts_list_state.select(Some(app.selected_font));

    let fonts_list = List::new(font_items)
        .block(
            Block::default()
                .title(" Font [2] ↑↓ ")
                .borders(Borders::ALL)
                .border_style(if app.focus == Focus::Font {
                    Style::default().fg(theme_accent)
                } else {
                    Style::default()
                }),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_stateful_widget(fonts_list, area, &mut fonts_list_state);
}

fn render_color_list(f: &mut Frame, app: &App, area: ratatui::layout::Rect, theme_accent: Color) {
    let color_items: Vec<ListItem> = app
        .available_colors
        .iter()
        .enumerate()
        .map(|(i, scheme)| {
            let style = if i == app.selected_color {
                Style::default()
                    .fg(theme_accent)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(scheme.name).style(style)
        })
        .collect();

    let mut colors_list_state = ratatui::widgets::ListState::default();
    colors_list_state.select(Some(app.selected_color));

    let colors_list = List::new(color_items)
        .block(
            Block::default()
                .title(" Theme [3] ↑↓ ")
                .borders(Borders::ALL)
                .border_style(if app.focus == Focus::Color {
                    Style::default().fg(theme_accent)
                } else {
                    Style::default()
                }),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_stateful_widget(colors_list, area, &mut colors_list_state);
}

fn render_font_size(f: &mut Frame, app: &App, area: ratatui::layout::Rect, theme_accent: Color) {
    let fontsize_style = if app.focus == Focus::FontSize {
        Style::default()
            .fg(theme_accent)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let fontsize = Paragraph::new(format!("{:.0}px", app.font_size))
        .style(fontsize_style)
        .block(
            Block::default()
                .title(" Size [4] ↑↓ ")
                .borders(Borders::ALL)
                .border_style(if app.focus == Focus::FontSize {
                    Style::default().fg(theme_accent)
                } else {
                    Style::default()
                }),
        );
    f.render_widget(fontsize, area);
}

fn render_resolution(f: &mut Frame, app: &App, area: ratatui::layout::Rect, theme_accent: Color) {
    let resolution_style = if app.focus == Focus::Resolution {
        Style::default()
            .fg(theme_accent)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let display_text = if app.focus == Focus::Resolution {
        if app.resolution_focus == ResolutionFocus::Width {
            format!("{}█ x {}", app.width_input, app.height_input)
        } else {
            format!("{} x {}█", app.width_input, app.height_input)
        }
    } else {
        format!("{} x {}", app.width_input, app.height_input)
    };

    let resolution = Paragraph::new(display_text).style(resolution_style).block(
        Block::default()
            .title(" Resolution [5] ←→ ")
            .borders(Borders::ALL)
            .border_style(if app.focus == Focus::Resolution {
                Style::default().fg(theme_accent)
            } else {
                Style::default()
            }),
    );
    f.render_widget(resolution, area);
}

fn render_output(
    f: &mut Frame,
    app: &App,
    area: ratatui::layout::Rect,
    theme_fg: Color,
    theme_accent: Color,
) {
    let output_text = if !app.status_message.is_empty() {
        app.status_message.clone()
    } else {
        format!("Output: {}", app.generate_filename())
    };

    let output_style = if app.status_message.starts_with('✓') {
        Style::default().fg(theme_accent)
    } else if app.status_message.starts_with('✗') {
        Style::default().fg(Color::Red)
    } else {
        Style::default().fg(theme_fg)
    };

    let output = Paragraph::new(output_text).style(output_style).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme_fg)),
    );
    f.render_widget(output, area);
}
