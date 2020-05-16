use std::os::raw::c_void;

use image::GenericImageView;

static TEXTURES: include_dir::Dir = include_dir!("./resources/textures");
pub struct Texture {
    pub id: u32,
}

impl Texture {
    pub fn new(filename: &str) -> Texture {
        let mut texture = 0;

        // load image, create texture and generate mipmaps
        let img = image::load_from_memory(
            TEXTURES
                .get_file(format!("{}", filename))
                .unwrap()
                .contents(),
        )
        .unwrap();
        let data = img.to_bytes();
        let (width, height) = img.dimensions();
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width as i32,
                height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                &data[0] as *const u8 as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Texture { id: texture }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
