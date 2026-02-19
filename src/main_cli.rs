use clap::{Parser, Subcommand};

mod ascii_gen;
mod colors;
mod fonts;
mod image_gen;

use ascii_gen::AsciiGenerator;
use colors::ColorScheme;
use image_gen::ImageGenerator;

#[derive(Parser)]
#[command(name = "ascii-cli")]
#[command(about = "ASCII art wallpaper generator", long_about = None)]
struct Cli {
    /// Text to generate
    text: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,

    /// Font to use (default: standard)
    #[arg(short, long, default_value = "standard")]
    font: String,

    /// Color scheme (default: nord)
    #[arg(short, long, default_value = "nord")]
    color: String,

    /// Font size (default: 16)
    #[arg(short, long, default_value_t = 16.0)]
    size: f32,

    /// Enable gradient effect
    #[arg(short, long)]
    gradient: bool,

    /// Output filename (default: wallpaper.png)
    #[arg(short, long, default_value = "wallpaper.png")]
    output: String,

    /// Layout: horizontal/vertical (default: horizontal)
    #[arg(short, long, default_value = "horizontal")]
    layout: String,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available fonts
    ListFonts,
    /// List all color schemes
    ListColors,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Handle subcommands
    if let Some(command) = cli.command {
        match command {
            Commands::ListFonts => {
                list_fonts();
                return Ok(());
            }
            Commands::ListColors => {
                list_colors();
                return Ok(());
            }
        }
    }

    // Check if text is provided
    let text = match cli.text {
        Some(t) => t,
        None => {
            eprintln!("Error: Please provide text to generate");
            eprintln!("Use --help for more information");
            std::process::exit(1);
        }
    };

    // Validate font
    if !fonts::is_valid_font(&cli.font) {
        eprintln!("⚠ Unknown font '{}', using default 'standard'", cli.font);
    }

    // Get color scheme
    let color_scheme = ColorScheme::from_name(&cli.color).unwrap_or_else(|| {
        println!(
            "⚠ Unknown color scheme '{}', using default 'nord'",
            cli.color
        );
        ColorScheme::nord()
    });

    // Validate layout
    let layout = match cli.layout.as_str() {
        "horizontal" | "vertical" => cli.layout.as_str(),
        _ => {
            println!(
                "⚠ Unknown layout '{}', using default 'horizontal'",
                cli.layout
            );
            "horizontal"
        }
    };

    // Validate font size
    let font_size = if cli.size < 8.0 || cli.size > 48.0 {
        println!("⚠ Font size out of range (8-48), using default 16");
        16.0
    } else {
        cli.size
    };

    println!("Color scheme: {}", color_scheme.name);
    println!("Font: {}", cli.font);
    println!("Size: {}px", font_size);
    println!("Layout: {}", layout);
    if cli.gradient {
        println!("Gradient: enabled");
    }
    println!();

    println!("Generating ASCII art...");
    let generator = AsciiGenerator::new();
    let ascii_gen = generator.generate(&text, &cli.font);

    println!("\nPreview:");
    println!("{}", ascii_gen);

    println!("\nGenerating PNG wallpaper...");
    let img_gen = ImageGenerator::new(1920, 1080);
    img_gen.generate(
        &ascii_gen,
        color_scheme,
        &cli.output,
        layout,
        font_size,
        cli.gradient,
    )?;

    println!("✓ Wallpaper saved to: {}", cli.output);

    Ok(())
}

fn list_colors() {
    println!("Available color schemes:");
    println!();
    for scheme in ColorScheme::get_all() {
        println!("  • {}", scheme.name);
    }
}

fn list_fonts() {
    println!("Available fonts:");
    println!();
    for font in fonts::AVAILABLE_FONTS {
        println!("  • {}", font);
    }
}
