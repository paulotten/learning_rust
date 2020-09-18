extern crate image;

use image::{ImageBuffer, RgbImage};

pub fn test() {
    let test_file = "./src/sample1.cr2";
    let image = rawloader::decode_file(test_file).unwrap();

    let mut output_buffer: RgbImage =
        ImageBuffer::new((image.width / 2) as u32, (image.height / 2) as u32);

    if let rawloader::RawImageData::Integer(data) = image.data {
        let mut y: usize = 0;

        while y < (image.height / 2) {
            let mut x: usize = 0;

            while x < (image.width / 2) {
                // see https://en.wikipedia.org/wiki/Bayer_filter
                // I'm converting 4 Bayer pixels into one RGB pixel
                let r = data[y * 2 * image.width + x * 2] >> 6;
                let g1 = data[(y * 2) * image.width + x * 2 + 1] >> 6;
                let g2 = data[(y * 2 + 1) * image.width + x * 2] >> 6;
                let b = data[(y * 2 + 1) * image.width + x * 2 + 1] >> 6;

                let pixel = image::Rgb([r as u8, ((g1 + g2) / 2) as u8, b as u8]);

                output_buffer.put_pixel(x as u32, y as u32, pixel);

                x += 1;
            }
            y += 1;
        }
        output_buffer
            .save("./src/test_raw.jpeg")
            .expect("Could not save");
    } else {
        eprintln!("Don't know how to process non-integer raw files");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_test() {
        test();
    }
}
