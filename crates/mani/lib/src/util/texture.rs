use super::constant;
use crate::component::render_component::Texture;
use fabled_render::texture::conversion::codecs::dds::*;
use image::GenericImageView;
use std::convert::TryFrom;

//todo clean solution.
pub fn load<P: AsRef<std::path::Path>>(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    path: P,
) -> anyhow::Result<Texture> {
    //-------------------------For Png and JPEG Support-------------------------
    //-------------------------Works-------------------------

    /*let dyn_img = if path.as_ref().exists() {
            match path.as_ref().extension() {
                None => image::open(std::path::Path::new(constant::invalid_map_path().as_str()))?,
                Some(_) => image::open(path.as_ref())?, //todo check extension if it is a jpg use rgb. if it is a png use rgba
            }
        } else {
            image::open(std::path::Path::new(constant::invalid_map_path().as_str()))?
        };

    //-------------------------For DDS Support-------------------------
    //-------------------------Works-------------------------

    */
    /*let path_to = path.as_ref();
    let dds = DdsTextureLoader::default();
    let dyn_img = dds.load(path_to).unwrap();*/

    //-------------------------For KTX2 Support-------------------------

    from_image(
        device,
        queue,
        image::DynamicImage::new_rgb8(100, 100),
        path.as_ref().to_str().unwrap().to_string(),
    )
}

pub fn create_depth_texture(device: &wgpu::Device, size: winit::dpi::PhysicalSize<u32>) -> Texture {
    let extend3d = wgpu::Extent3d {
        width: size.width,
        height: size.height,
        depth_or_array_layers: 1,
    };

    let depth_texture = create_texture(
        device,
        extend3d,
        wgpu::TextureFormat::Depth32Float,
        wgpu::TextureUsage::RENDER_ATTACHMENT | wgpu::TextureUsage::SAMPLED,
        Some("Depth Texture"),
        1,
    );

    let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

    let depth_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: None,
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Linear,
        lod_min_clamp: -100.0,
        lod_max_clamp: 100.0,
        compare: Some(wgpu::CompareFunction::LessEqual),
        ..Default::default()
    });

    Texture {
        view: depth_view,
        sampler: depth_sampler,
    }
}

#[allow(dead_code)] //TODO note Dead Code
pub fn from_bytes(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    img_buffer: &[u8],
) -> anyhow::Result<Texture> {
    let dyn_img = image::load_from_memory(img_buffer)?;

    from_image(device, queue, dyn_img, "".to_string())
}

fn from_image(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    dyn_img: image::DynamicImage,
    _test: String,
) -> anyhow::Result<Texture> {
    let rgba8_img = dyn_img.flipv().to_rgba8();

    let dimensions = rgba8_img.dimensions();
    let ktx =
        fabled_render::KtxTextureLoader::from_stream(std::fs::File::open(_test.as_str()).unwrap());

    let mut extend_test = wgpu::Extent3d::default();
    let mut mip_level = 0;
    ktx.iterate_levels(|mip, face, width, height, depth, pix| {
        extend_test.width = width as u32;
        extend_test.height = height as u32;
        extend_test.depth_or_array_layers = depth as u32;
        if mip_level <= mip {
            mip_level = mip;
        }
        Ok(())
    })
    .unwrap();

    extend_test.physical_size(wgpu::TextureFormat::Rgba8UnormSrgb);

    let target = ktx
        .data()
        .chunks(extend_test.width as usize * 4)
        .rev()
        .flat_map(|row| row.iter())
        .cloned()
        .collect::<Vec<_>>();

    let extend3d = wgpu::Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth_or_array_layers: 1,
    };

    let diffuse_texture = create_texture(
        device,
        extend_test,
        wgpu::TextureFormat::Rgba8UnormSrgb,
        wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED,
        Some("Diffuse Texture"),
        1,
    );

    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &diffuse_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        &target,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(core::num::NonZeroU32::new(ktx.row_pitch(0) as u32).unwrap()),
            rows_per_image: Some(core::num::NonZeroU32::new(extend_test.height).unwrap()),
        },
        extend_test,
    );

    let diffuse_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());

    let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("Diffuse Sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Linear,
        anisotropy_clamp: None,
        ..Default::default()
    });

    Ok(Texture {
        view: diffuse_view,
        sampler: diffuse_sampler,
    })
}

fn create_texture(
    device: &wgpu::Device,
    size: wgpu::Extent3d,
    format: wgpu::TextureFormat,
    usage: wgpu::TextureUsage,
    label: Option<&str>,
    mip_map_level: u8,
) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label,
        size,
        mip_level_count: mip_map_level as u32,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage,
    })
}
