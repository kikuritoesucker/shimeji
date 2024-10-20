use std::{ffi::c_void, io::Cursor};
use gl::types::*;

pub struct Texture {
    pub id: u32,
}

pub enum GLTexture {
    /// `(width, height, data, mipmap)`
    Texture2D(i32, i32, *const c_void, bool),
}

impl Texture {
    pub fn new(tex: GLTexture) -> Self {
        let mut id: u32 = 0;

        unsafe {
            gl::GenTextures(1, &mut id);

            match tex {
                GLTexture::Texture2D(width, height, data, generate_mipmap) => {
                    gl::BindTexture(gl::TEXTURE_2D, id);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

                    gl::TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        gl::RGBA as i32,
                        width,
                        height,
                        0,
                        gl::RGBA,
                        gl::UNSIGNED_BYTE,
                        data,
                    );
                    if generate_mipmap {
                        gl::GenerateMipmap(id);
                    }
                    
                }
                _ => (),
            }
        }
        Self { id }
    }

    pub fn read_from_file(path : &str) -> (u32, u32, Vec<u8>){
        let mut img = image::ImageReader::open(path).expect("cannot open image").decode().expect("cannot decode image");
        img.apply_orientation(image::metadata::Orientation::Rotate270FlipH);
        (img.width(), img.height(), img.into_rgba8().to_vec())
    }
}
