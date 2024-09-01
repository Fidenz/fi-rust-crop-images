use fi_common::{error::Error, logger};
use image::{imageops::crop, open};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::constants::{FAILED_TO_LOAD_SOURCE_IMAGE, FAILED_TO_SAVE_IMAGE};

pub fn crop_img(
    source_path: &str,
    location_x: u32,
    location_y: u32,
    crop_width: u32,
    crop_height: u32,
    out_path: &str,
) -> Result<(), Error> {
    let mut img = match open(source_path) {
        Ok(val) => val,
        Err(error) => {
            logger::error(error.to_string().as_str());
            return Err(Error::new(FAILED_TO_LOAD_SOURCE_IMAGE));
        }
    };
    let cropped_img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        crop(&mut img, location_x, location_y, crop_width, crop_height).to_image();

    match cropped_img.save(out_path) {
        Ok(_) => {}
        Err(error) => {
            logger::error(error.to_string().as_str());
            return Err(Error::new(FAILED_TO_SAVE_IMAGE));
        }
    }
    return Ok(());
}
