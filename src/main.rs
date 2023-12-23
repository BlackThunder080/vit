use std::io::Write;

use clap::Parser;
use image::GenericImageView;
use crossterm::style::Stylize;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
    
    #[arg(short, long, default_value_t = 1)]
    char_size: u8,
}

fn main() {
    let args = Args::parse();
    let mut stdout = std::io::stdout();

    crossterm::terminal::enable_raw_mode().unwrap();
    crossterm::queue!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
    
    let (width, height) = crossterm::terminal::size().unwrap();
    
    let img = image::io::Reader::open(args.file).unwrap().decode().unwrap();
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
