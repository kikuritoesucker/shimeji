use std::ffi::c_void;

use gl::types::*;

pub struct Texture {
    id: u32,
}

pub enum GLTexture {
    /// `(width, height, data, mipmap)`
    Texture2D(i32, i32, *const c_void, bool),
}

// impl Default for Texture {
//     fn default() -> Self {
//         Self::new(gl::TEXTURE_2D)
//     }
// }

impl Texture {
    pub fn new(target: GLTexture) -> Self {
        let mut id: u32 = 0;

        unsafe {
            gl::GenTextures(1, &mut id);

            match target {
                GLTexture::Texture2D(width, height, data, generate_mipmap) => {
                    
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT.try_into().unwrap());
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT.try_into().unwrap());
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR.try_into().unwrap());
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR.try_into().unwrap());

                    gl::BindTexture(gl::TEXTURE_2D, id);
                    gl::TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        gl::RGB.try_into().unwrap(),
                        width,
                        height,
                        0,
                        gl::RGB,
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
}
