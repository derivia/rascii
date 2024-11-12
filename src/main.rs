use clap::Parser;
use image::{imageops::FilterType, DynamicImage, GenericImageView, Rgba};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(help = "Image to convert into ASCII art")]
    image_path: String,

    #[arg(long, default_value = "100", help = "Width of the output")]
    width: u32,

    #[arg(
        long,
        default_value = "0.5",
        help = "Aspect ratio correction factor for output"
    )]
    aspect_ratio: f32,

    #[arg(long, default_value = "1.0", help = "Contrast adjustment (0.5 to 2.0)")]
    contrast: f32,

    #[arg(long, default_value = "false", help = "Invert colors")]
    invert: bool,

    #[arg(long, default_value = "false", help = "Use dense character set")]
    dense: bool,
}

fn rgb_to_grayscale(pixel: Rgba<u8>) -> u8 {
    let [r, g, b, a] = pixel.0;
    let alpha = a as f32 / 255.0;
    let r = (r as f32 * alpha + 255.0 * (1.0 - alpha)) as u8;
    let g = (g as f32 * alpha + 255.0 * (1.0 - alpha)) as u8;
    let b = (b as f32 * alpha + 255.0 * (1.0 - alpha)) as u8;

    // perceptual luminance formula (see: https://en.wikipedia.org/wiki/Rec._709)
    (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32) as u8
}

fn get_ascii_chars(dense: bool) -> Vec<char> {
    if dense {
        "@&%QWN0gB$D8mHXKAUbGOpV4d9h6PkyqwSE2]ayjxY5Zeo[nult13If}C{iF|(7J)vTLs?z/*cr!+><;=^,':-. "
            .chars()
            .collect()
    } else {
        "@&%Q$wusv*+=^,':-. ".chars().collect()
    }
}

fn adjust_contrast(value: u8, contrast: f32) -> u8 {
    let normalized = value as f32 / 255.0;
    let adjusted = (((normalized - 0.5) * contrast) + 0.5).clamp(0.0, 1.0);

    (adjusted * 255.0) as u8
}

fn resize_image(img: &DynamicImage, args: &Cli) -> (DynamicImage, u32, u32) {
    let (orig_width, orig_height) = img.dimensions();
    let new_width = args.width;
    let new_height =
        (new_width as f32 * args.aspect_ratio * (orig_height as f32 / orig_width as f32)) as u32;

    (
        img.resize_exact(new_width, new_height, FilterType::CatmullRom),
        new_width,
        new_height,
    )
}

fn img_to_ascii(img: &DynamicImage, args: &Cli) -> Vec<String> {
    let (img, width, height) = resize_image(img, args);

    let ascii_chars = get_ascii_chars(args.dense);
    let mut output = Vec::with_capacity(height as usize);

    for y in 0..height {
        let mut line = String::with_capacity(width as usize);
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let mut gray = rgb_to_grayscale(pixel);

            gray = adjust_contrast(gray, args.contrast);

            if args.invert {
                gray = 255 - gray;
            }

            let idx = ((gray as f32 / 255.0) * (ascii_chars.len() - 1) as f32).round() as usize;
            line.push(ascii_chars[ascii_chars.len() - 1 - idx]);
        }
        output.push(line);
    }

    output
}

fn main() -> Result<(), image::ImageError> {
    let args = Cli::parse();

    let img = image::open(&args.image_path)?;
    let ascii_art = img_to_ascii(&img, &args);

    for line in ascii_art {
        println!("{}", line);
    }

    Ok(())
}
