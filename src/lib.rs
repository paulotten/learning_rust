extern crate image;

use image::{ ImageBuffer, RgbImage};

// not even close to functional lol
pub fn test() {
    let test_file = "./src/sample1.cr2";
    let image = rawloader::decode_file(test_file).unwrap();

    let mut output_buffer: RgbImage = ImageBuffer::new(image.width as u32, image.height as u32);


    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut marker = 0;

    if let rawloader::RawImageData::Integer(data) = image.data {
    for pix in data {
      // output_buffer.put_pixel(x, y, pix)
      if marker > 0 && marker % image.width == 0 {
        y_pos += 1;
        x_pos = 0;
        marker = 0;
      }
      // output_buffer.put_pixel(x_pos, y_pos, image::Rgb([]))

      let r = pix & 0xf;
      let g = (pix >> 4) & 0xf;
      let b = (pix >> 8) & 0xf;
      let a = (pix as u32 >> 16) & 0xf;

      let values = rgb::RGBA::new(r as u8, g as u8, b as u8, a as u8).rgb();
      let pixels = image::Rgb([values.r, values.g, values.b]);

      output_buffer.put_pixel(x_pos, y_pos, pixels);

      marker += 1;

    }

    output_buffer.save("./src/test_raw.jpeg").expect("Could not save");
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
