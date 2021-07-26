use crate::{ColorType, Extent3d, FlipAxis, Texture, TextureDescriptor};
use image::GenericImageView;

#[derive(Default, Clone)]
pub struct JpgTextureLoader;

// Jpg File Format
impl JpgTextureLoader {
    //Decoder
    pub fn load<P: AsRef<std::path::Path>>(
        &self,
        path: P,
        texture_descriptor: &TextureDescriptor,
    ) -> anyhow::Result<Texture> {
        let file = std::fs::File::open(path.as_ref())?;

        let jpg_decoder = image::codecs::jpeg::JpegDecoder::new(file)?;

        let dyn_img = image::DynamicImage::from_decoder(jpg_decoder)?;

        match texture_descriptor.flip_axis {
            FlipAxis::FlipX => dyn_img.fliph(),
            FlipAxis::FlipY => dyn_img.flipv(),
            FlipAxis::FlipZ => unimplemented!(),
        };

        let jpg_texture = Texture {
            data: dyn_img.to_bytes(),
            size: Extent3d {
                width: dyn_img.width(),
                height: dyn_img.height(),
                depth_or_array_layers: 1,
            },
            sample_count: 1,
            mip_level: 0,
            color_type: dyn_img.color().into(),
            rows_per_image: dyn_img.width() * 4,
        };

        Ok(jpg_texture)
    }
}

#[cfg(test)]
mod jpg_loader_codecs {

    use crate::codecs::*;
    use crate::texture::common::*;
    #[test]
    fn load_jpg() {
        let jpg_loader = JpgTextureLoader::default();
        let jpgyellow = jpg_loader
            .load(
                JPG_TEST_TEXTURE,
                &TextureDescriptor {
                    flip_axis: Default::default(),
                },
            )
            .unwrap();

        assert!(!jpgyellow.data.is_empty());

        let img =
            image::RgbImage::from_raw(jpgyellow.size.width, jpgyellow.size.height, jpgyellow.data)
                .unwrap();

        img.save_with_format("D:\\Study\\Fabled Engine\\crates\\mani\\fabled_render\\src\\texture\\texture\\test\\albedo\\test2.jpg", image::ImageFormat::Jpeg).unwrap();
    }
}
