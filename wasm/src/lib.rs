use image::{io::Reader as ImageReader, GenericImageView, imageops, Rgba, RgbaImage, ImageBuffer, codecs::{png::PngEncoder, jpeg::JpegEncoder}, ImageFormat};
use std::io::Cursor;

mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {name}!"));
}

#[wasm_bindgen]
pub fn files(padding: u32, buf: Vec<u8>) -> Vec<u8> {
    let r = ImageReader::new(Cursor::new(&buf));

    console_log!("{:?}", &buf);
    let mut bg: ImageBuffer<Rgba<u8>, Vec<u8>>;

    // 
    let r = r.with_guessed_format().unwrap();
    let format = r.format().unwrap();

    match r.decode() {
        Ok(f) => {
            let (w, h) = f.dimensions();
            let nw = if w > h { w } else { h };
            
            let padding = padding * nw / 800;
            let wp = nw + padding * 2;
            let x = (nw - w) / 2 + padding;
            let y = (nw - h) / 2 + padding;

            bg = RgbaImage::from_pixel(wp, wp, Rgba([255, 255, 255, 255]));
            imageops::replace(&mut bg, &f, x as i64, y as i64);

            let mut buffer: Vec<u8> = Vec::new();
            let _ = match format {
                // TODO: all other encoders
                ImageFormat::Png => { bg.write_with_encoder(PngEncoder::new(&mut buffer)) },
                _ => { bg.write_with_encoder(JpegEncoder::new(&mut buffer)) }
            };

            console_log!("{:?}", &buffer);
            buffer
        },
        Err(e) => {
            console_log!("{e}");
            Vec::new()
        }
    }
}
