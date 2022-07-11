use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::{decode, encode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;

//section 14 - 250
#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"Grayscale called.".into());

    //section 14 - 251
    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image decoded.".into());

    //section 14 - 252
    //https://docs.rs/image/0.19.0/image/fn.load_from_memory.html
    let mut image = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded.".into());

    //section 14 - 253
    image = image.grayscale();
    log(&"Grayscale effect applied.".into());

    //section 14 - 254
    let mut buffer = vec![];
    image.write_to(&mut buffer, Png).unwrap();
    log(&"New image written.".into());

    //section 14 - 255
    let encoded_image = encode(&buffer);
    let data_url = format!(
        "data:image/png;base64,{}",
        encoded_image
    );

    data_url
}

//section 14 - 245
//webpack: https://webpack.js.org