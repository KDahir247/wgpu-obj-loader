mod dds;
mod hdr;
mod jpg;
mod png;
mod tiff;

use crate::texture::container::FlipAxis;
pub use dds::*;
pub use hdr::*;
pub use jpg::*;
pub use png::*;
pub use tiff::*;

#[derive(Clone, Debug)]
pub struct TextureDescriptor {
    pub flip_axis: FlipAxis,
}

impl Default for TextureDescriptor {
    // Explicit default for format and usage so it doesn't = to 0
    fn default() -> Self {
        Self {
            flip_axis: Default::default(),
        }
    }
}

#[cfg(test)]
mod codecs_test {
    use crate::texture::codecs::TextureDescriptor;
    #[test]
    fn data_alignment() {
        let tex_desc = std::mem::size_of::<TextureDescriptor>();
        assert_eq!(tex_desc & (tex_desc - 1), 0);
    }
}
