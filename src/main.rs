use std::io::Write;

use clap::Parser;
use image::GenericImageView;
use crossterm::style::Stylize;
use clipboard::ClipboardProvider;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    file: Option<String>,
    
    #[arg(short, long, default_value_t = 2)]
    char_size: u8,
}

fn main() {
    let args = Args::parse();
    let mut stdout = std::io::stdout();

    let file = match args.file {
        Some(file) => file,
        None => clipboard::ClipboardContext::new().unwrap().get_contents().unwrap(),
    };

    crossterm::terminal::enable_raw_mode().unwrap();
    crossterm::queue!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
    
    let (width, height) = crossterm::terminal::size().unwrap();

    let img = if let Some(img) = from_file(&file) {
        img
    } else if let Some(img) = from_url(&file) {
        img
    } else {
        todo!()
    };
    
    let resized = img.resize(
        width as u32,
        height as u32,
        image::imageops::FilterType::CatmullRom
    );
    
    for (x, y, rgba) in resized.pixels() {
        crossterm::queue!(
            stdout,
            crossterm::cursor::MoveTo(x as u16 * args.char_size as u16, y as u16),
            crossterm::style::PrintStyledContent(" ".repeat(args.char_size as usize).on(pixel_to_colour(rgba))),
        ).unwrap();
    }
    
    stdout.flush().unwrap();
    
    crossterm::terminal::disable_raw_mode().unwrap();
    std::io::stdin().read_line(&mut String::new()).unwrap();
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
