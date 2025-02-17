extern crate image;

use wasm_bindgen::prelude::*;
use std::io;
use image::io::{Reader};
use image::imageops::FilterType;
use image::ImageFormat;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);

  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_u32(a: u32);
}

#[wasm_bindgen] 
pub fn resize_image(mut data: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
  // decode data into DynamicImage
  let full = Reader::new(io::Cursor::new(&data))
    .with_guessed_format()
    .unwrap()
    .decode()
    .unwrap();

  // debug stuff
  // log_u32(width);
  // log_u32(height);

  // resize DynamicImage
  let resized = full.resize(width, height, FilterType::Lanczos3);

  // // convert to webp for compression
  // let encoder: Encoder = Encoder::from_image(&image).unwrap();

  // // Encode image into WebPMemory.
  // let encoded_webp: WebPMemory = encoder.encode(quality.unwrap_or(65f32));

  // write to data
  resized.write_to(&mut io::Cursor::new(&mut data), ImageFormat::Png).expect("REASON");

  return data;
}
