use image::*;
use std::path::Path;
use webp::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    for a in &args {
        if a.contains(".png") {
            println!("Converting {} to WebP", a);
            convert_image_to_webp(&a, &a.replace(".png", ".webp"));
        }
    }

    if args.len() == 0 {
        println!("Please provide a path to a .png file.");
    }
}

/// ## Convert an image to WebP format
/// Takes two path parameters, containing the full path and filename of the input image, and the output WebP file.
/// 
/// Example: 
/// 
/// input:  "images/Sticker.png"
/// 
/// output: "images/Sticker.webp"
fn convert_image_to_webp(input: &str, output: &str) {
    // Using `image` crate, open the included .jpg file
    let img = image::open(input).unwrap();
    let (w, h) = img.dimensions();
    // Optionally, resize the existing photo and convert back into DynamicImage
    let size_factor = 1.0;
    let img: DynamicImage = image::DynamicImage::ImageRgba8(imageops::resize(
        &img,
        (w as f64 * size_factor) as u32,
        (h as f64 * size_factor) as u32,
        imageops::FilterType::Triangle,
    ));

    // Create the WebP encoder for the above image
    let encoder: Encoder = Encoder::from_image(&img).unwrap();
    // Encode the image at a specified quality 0-100
    let webp: WebPMemory = encoder.encode(100f32);
    // Define and write the WebP-encoded file to a given path
    let output_path = Path::new(output);

    std::fs::write(&output_path, &*webp).unwrap();
}