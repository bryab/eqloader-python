use image::codecs::bmp::BmpDecoder;
use pyo3::{exceptions::PyValueError, prelude::*};
use std::io::Cursor;

pub fn texture_transparent_color(bmp_data: Vec<u8>) -> PyResult<(f32, f32, f32)> {
    let mut file = Cursor::new(bmp_data);
    let decoder =
        BmpDecoder::new(&mut file).map_err(|_| PyValueError::new_err("Invalid bitmap data!"))?;
    // NOTE: It is not necessary to get the BMP palette except for images with cutout transparency.
    // Possibly this operation should be optional if it is expensive, but it doesn't seem to be.
    match decoder.get_palette() {
        Some(palette) => Ok((
            palette[0][0] as f32 / 255.0,
            palette[0][1] as f32 / 255.0,
            palette[0][2] as f32 / 255.0,
        )),
        None => Err(PyValueError::new_err("Bitmap does not have a palette!")),
    }
}
