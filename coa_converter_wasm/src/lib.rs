use coa_converter_lib as coa;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn coa_from_image_reduced_colors(
    image_data: Vec<u8>,
    is_landed_title: bool,
    color_count: u8,
) -> Option<String> {
    assert!(color_count >= 2);
    let img = image::load_from_memory(&image_data);
    match img {
        Ok(img) => Some(coa::from_image_custom_colors(
            img,
            is_landed_title,
            color_count,
        )),
        Err(_) => None,
    }
}

#[wasm_bindgen]
pub async fn coa_from_image_vanilla_colors(
    image_data: Vec<u8>,
    is_landed_title: bool,
) -> Option<String> {
    let img = image::load_from_memory(&image_data);
    match img {
        Ok(img) => Some(coa::from_image_vanilla_colors(img, is_landed_title)),
        Err(_) => None,
    }
}

#[wasm_bindgen]
pub async fn coa_from_image_all_colors(
    image_data: Vec<u8>,
    is_landed_title: bool,
) -> Option<String> {
    let img = image::load_from_memory(&image_data);
    match img {
        Ok(img) => Some(coa::from_image_all_colors(img, is_landed_title)),
        Err(_) => None,
    }
}
