use image::{DynamicImage, GenericImageView};

const ASCII_CHARS: [char; 74] = [
    '█', '▓', '▒', '░', '$', '@', 'B', '%', '8', '&', 'W', 'M', '#', '*', 'o', 'a', 'h', 'k', 'b',
    'd', 'p', 'q', 'w', 'm', 'Z', 'O', '0', 'Q', 'L', 'C', 'J', 'U', 'Y', 'X', 'z', 'c', 'v', 'u',
    'n', 'x', 'r', 'j', 'f', 't', '/', '\\', '|', '(', ')', '1', '{', '}', '[', ']', '?', '-', '_',
    '+', '~', '<', '>', 'i', '!', 'l', 'I', ';', ':', ',', '"', '^', '`', '\'', '.', ' ',
];

fn resize_image(image: &DynamicImage, width: u32) -> DynamicImage {
    let (rheight, rwidth) = (image.height(), image.width());
    let height: u32 = (width as f32 * (rheight as f32 / rwidth as f32)).round() as u32;
    println!(
        "Original dimension: ({}, {})\nTarget dimensions : ({}, {})",
        rwidth, rheight, width, height
    );
    image.resize(width, height, image::imageops::FilterType::Nearest)
}

fn pixel_to_ascii(image: &DynamicImage) -> String {
    let mut out = String::with_capacity((image.height() * image.width()) as usize);

    for i in 0..image.height() {
        for j in 0..image.width() {
            let pixel_data = image.get_pixel(j, i);
            let brightness: f64 =
                ((pixel_data[0] as u64 + pixel_data[1] as u64 + pixel_data[2] as u64) / 3) as f64;
            let character_position = ((brightness / 255.0) * 73 as f64).round() as usize;
            out.push(ASCII_CHARS[character_position]);
        }
        out.push('\n');
    }

    out
}

pub fn image_to_ascii(image: &DynamicImage, target_width: Option<u32>) -> String {
    match target_width {
        None => pixel_to_ascii(&image.grayscale()),
        Some(t) => pixel_to_ascii(&resize_image(image, t).grayscale()),
    }
}
