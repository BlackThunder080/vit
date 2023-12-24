use std::io::Write;

use clap::Parser;
use image::GenericImageView;
use crossterm::style::Stylize;
use clipboard::ClipboardProvider;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to image: can be a file or url.
    /// If no path is provided the clipboard will be used
    file: Option<String>,

    /// Number of characters per pixel
    #[arg(short, default_value_t = 2)]
    chars: u8,

    /// Open in browser as well
    #[arg(short)]
    browser: bool,
}

fn main() {
    if let Err(error) = run() {
        let _ = write!(std::io::stderr(), "{}: {}", "error".red(), error);
    }
}

fn run() -> Result<(), String> {
    let args = Args::parse();
    let mut stdout = std::io::stdout();

    let file = match args.file {
        Some(file) => file,
        None => clipboard::ClipboardContext::new().map_err(|_| "")?.get_contents().map_err(|_| "failed to get clipboard contents")?,
    };

    let img = if let Some(img) = from_file(&file) {
        img
    } else if let Some(img) = from_url(&file) {
        img
    } else {
        return Err(format!(r#"failed to open image "{}""#, &file.dark_grey()));
    };

    if args.browser { let _ = webbrowser::open(&file); }

    crossterm::terminal::enable_raw_mode().map_err(|_| "")?;
    crossterm::queue!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).map_err(|_| "")?;
    
    let (width, height) = crossterm::terminal::size().map_err(|_| "")?;
    let resized = img.resize(
        width as u32,
        height as u32,
        image::imageops::FilterType::CatmullRom
    );
    
    for (x, y, rgba) in resized.pixels() {
        crossterm::queue!(
            stdout,
            crossterm::cursor::MoveTo(x as u16 * args.chars as u16, y as u16),
            crossterm::style::PrintStyledContent(" ".repeat(args.chars as usize).on(pixel_to_colour(rgba))),
        ).map_err(|_| "")?;
    }
    
    stdout.flush().map_err(|_| "")?;
    
    crossterm::terminal::disable_raw_mode().map_err(|_| "")?;
    std::io::stdin().read_line(&mut String::new()).map_err(|_| "")?;
    
    Ok(())
}

fn pixel_to_colour(pixel: impl image::Pixel<Subpixel = u8>) -> crossterm::style::Color {
    let rgb = pixel.to_rgb();
    crossterm::style::Color::Rgb { r: rgb.0[0], g: rgb.0[1], b: rgb.0[2] }
}

fn from_file(path: &str) -> Option<image::DynamicImage> {
    Some(image::io::Reader::open(path).ok()?.decode().ok()?)
}

fn from_url(path: &str) -> Option<image::DynamicImage> {
    Some(image::load_from_memory(&reqwest::blocking::get(path).ok()?.bytes().ok()?).ok()?)
}
