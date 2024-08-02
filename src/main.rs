use ab_glyph::FontRef;
use image::{ImageBuffer, Rgba};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage:\n\t{} <input_file_path>", args[0]);
        std::process::exit(1);
    }

    let p = PathBuf::from(&args[1]);
    let filename = p.file_name().unwrap().to_string_lossy();
    let input = std::fs::read_to_string(&p).expect("Unable to read your file...");
    let re =
        Regex::new(r"#([a-fA-F0-9]{6})").expect("Compiling a regex should basically never fail");

    let square_size = 50_i32;
    let padding = 10_i32;
    let text_height = 20_i32;
    let border = 50_i32;
    let image_size = 600_i32 + 2 * border;

    let font_data: &'static [u8] = include_bytes!("../FiraMono-Medium.ttf") as &[u8];
    let font = FontRef::try_from_slice(font_data).unwrap();

    let scale = ab_glyph::PxScale { x: 16.0, y: 16.0 };

    let mut img = ImageBuffer::from_pixel(
        image_size as u32,
        image_size as u32,
        Rgba([255, 255, 255, 255]),
    );

    // Explicitly sort them so it's nice
    let mut colours = re
        .captures_iter(&input)
        .map(|cap| cap[0].to_string())
        .collect::<HashSet<String>>()
        .into_iter()
        .collect::<Vec<String>>();
    colours.sort();

    colours.iter().enumerate().for_each(|(i, color)| {
        let index = i as i32;
        let x = border + (index % 10) * (square_size + padding);
        let y = border + (index / 10) * (square_size + padding + text_height);

        let rect = Rect::at(x, y).of_size(square_size as u32, square_size as u32);
        let color_rgba = hex_to_rgba(color);
        draw_filled_rect_mut(&mut img, rect, Rgba(color_rgba));

        draw_text_mut(
            &mut img,
            Rgba([0, 0, 0, 255]),
            x,
            y + square_size,
            scale,
            &font,
            &color[1..],
        );
    });

    // Name the output as {their-input}.png
    let mut output = PathBuf::from(&filename.to_string());
    output.set_extension("png");
    img.save(output).expect("Unable to save palette.png");
}

fn hex_to_rgba(hex: &str) -> [u8; 4] {
    let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
    let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
    let b = u8::from_str_radix(&hex[5..7], 16).unwrap();
    [r, g, b, 255]
}
