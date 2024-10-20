use std::{ffi::c_void, io::Cursor};

pub fn read_from_file(path : &str) -> (u32, u32, Vec<u8>){
    let mut img = image::ImageReader::open(path)
        .expect("cannot open image")
        .decode()
        .expect("cannot decode image");
    img.apply_orientation(image::metadata::Orientation::Rotate270FlipH);
    (img.width(), img.height(), img.into_rgba8().to_vec())
}
