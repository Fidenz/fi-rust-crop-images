use fi_images::img::crop_img;
use imageproc::drawing::Canvas;
use wasm_bindgen_test::wasm_bindgen_test;

#[test]
#[wasm_bindgen_test]
pub fn test_basic_crop() {
    let cropped_img_width = 100u32;
    let cropped_img_height = 100u32;
    let output_img_path = "./.generated/test_cropped_image.png";

    _ = crop_img(
        "./tests/test_image.png",
        0,
        0,
        cropped_img_width,
        cropped_img_height,
        output_img_path,
    );

    let output_img = match image::open(output_img_path) {
        Ok(val) => val,
        Err(error) => {
            eprintln!("Error loading output image: {}", error);
            assert!(false);
            return;
        }
    };
    let (width, height) = output_img.dimensions();
    assert_eq!(width, cropped_img_width);
    assert_eq!(height, cropped_img_height);
}
