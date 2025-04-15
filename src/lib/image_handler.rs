use image::{DynamicImage, ImageBuffer, ImageFormat, ImageReader, Rgba};

pub fn read_image(image_path: String) -> DynamicImage {
    let img = ImageReader::open(image_path.clone())
        .unwrap()
        .decode()
        .unwrap();

    return img;
}
