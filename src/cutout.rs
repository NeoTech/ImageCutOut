use clap::Parser;
use image::{GenericImageView, ImageReader};
use std::path::Path;
use vtracer::{Config, convert_image_to_svg};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, required = true)]
    image: String,
    #[arg(short, long, default_value_t = 1)]
    count: usize,
    // #[arg(long, default_value_t = false)]
    // png2bmp: bool,
    #[arg(long, default_value_t = false)]
    png2svg: bool,
    #[arg(short, long, required = true)]
    output: String
}

fn main() {
    let args = Args::parse();
    println!("Processing {}...", args.image);
    let image = ImageReader::open(args.image.clone()).unwrap().decode().unwrap();
/*
    if args.png2bmp {
        // Runs png to BMP and changes file ending to bmp.
        png_to_bmp(image.clone()).save(format!("converted_{}.bmp", &args.image[..args.image.len() - 4])).unwrap();
    } else
*/
    if args.png2svg {
        let config = Config::default();
        convert_image_to_svg(Path::new(&args.image), Path::new(&args.output), config).unwrap();
    } else {
        // Runs binary mask creation and padding.
        pad_image(create_binarymask(image.clone()), args.count).save(&args.output).unwrap();
    }   
}

fn create_binarymask(image: image::DynamicImage) -> image::DynamicImage {
    let mut new_image = image::ImageBuffer::new(image.width(), image.height());
    let (width, height) = image.dimensions();
    println!("Image dimensions: {}x{}", width, height);
    let first_pixel = image.get_pixel(0, 0);
    let first_alpha = first_pixel[3];
    for x in 0..width {
        for y in 0..height {
            let pixel = image.get_pixel(x, y);
            if first_alpha == 0 {
                if pixel[3] == 0 {
                    new_image.put_pixel(x, y, image::Rgba([0, 0, 0, 0]));
                } else {
                    new_image.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
                }
            } else {
                if pixel[0] == first_pixel[0] && pixel[1] == first_pixel[1] && pixel[2] == first_pixel[2] {
                    new_image.put_pixel(x, y, image::Rgba([0, 0, 0, 0]));
                } else {
                    new_image.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
                }
            }           
        }
    }
    image::DynamicImage::ImageRgba8(new_image)
}
/*
fn png_to_bmp(image: image::DynamicImage) -> image::DynamicImage {
    // Reads a PNG file and converts transparent pixels to white and saves it as a bmp file.
    let (width, height) = image.dimensions();
    let mut new_image = image::ImageBuffer::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let pixel = image.get_pixel(x, y);
            if pixel[3] == 0 {
                new_image.put_pixel(x, y, image::Rgba([255, 255, 255, 255]));
            } else {
                new_image.put_pixel(x, y, pixel);
            }
        }
    }
    image::DynamicImage::ImageRgba8(new_image)
}
*/

fn pad_image(image: image::DynamicImage, margin: usize) -> image::DynamicImage {
    let (width, height) = image.dimensions();
    let mut new_image = image::ImageBuffer::new(width + margin as u32 * 2, height + margin as u32 * 2);
    for x in 0..width {
        for y in 0..height {
            let pixel = image.get_pixel(x, y);
            new_image.put_pixel(x + margin as u32, y + margin as u32, pixel);
        }
    }
    image::DynamicImage::ImageRgba8(new_image)
}