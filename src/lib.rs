extern crate image;

use image::{ ImageBuffer, RgbImage};

// closer to functional
pub fn test() {
    let test_file = "./src/sample1.cr2";
    let image = rawloader::decode_file(test_file).unwrap();

    let mut output_buffer: RgbImage = ImageBuffer::new(image.width as u32, image.height as u32);

    // you had a marker and an x_pos variable
    // you just need x_pos and a bit of casting
    let mut x_pos: u32 = 0;
    let mut y_pos: u32 = 0;

    if let rawloader::RawImageData::Integer(data) = image.data {
    for pix /* u16 */ in data {
      if x_pos > 0 && x_pos % (image.width as u32) == 0 {
        y_pos += 1;
        x_pos = 0;
      }
      
      // so after after an hour of trying I realized I can't convert pix to rgb(a)
      // because it doesn't store rbg(a) data...
      // it stores Bayer data
      // see https://en.wikipedia.org/wiki/Bayer_filter

      // I can hack together a grey scale like in
      // https://github.com/pedrocr/rawloader#usage though...

      // take a look at https://github.com/wangds/libbayer
      // and see if you can use it to generate actually rgb data

      // also run this code through rustfmt

      // convert 12 bit color down to 8 bit
      let grey = (pix >> 4) as u8;

      let pixel = image::Rgb([grey, grey, grey]);

      output_buffer.put_pixel(x_pos, y_pos, pixel);

      x_pos += 1;
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
