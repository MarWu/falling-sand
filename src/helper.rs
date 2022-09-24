use bevy::{prelude::*, render::{render_resource::{Extent3d, TextureFormat}, texture::ImageSampler}};

pub fn key_from_coords(x: i32, y: i32) -> u64 {
    ((x as u64) << 32) | y as u64
}

pub fn coords_from_key(key: u64) -> (i32, i32) {
    let x = (key & 0xFFFFFFFF00000000) >> 32;
    let y = key & 0xFFFFFFFF;

    (x as i32, y as i32)
}

pub fn create_chunk_image(width: u32, height: u32) -> Image {
    let mut image = Image::new_fill(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &[0, 255, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
    );
    image.sampler_descriptor = ImageSampler::nearest();
    image
}

#[cfg(test)]
mod tests {
    use super::{key_from_coords, coords_from_key};

    #[test]
    fn test_key_from_coords() {
        let x = 17;
        let y = 9;
        let key = key_from_coords(x, y);
        let coords = coords_from_key(key);
        assert_eq!(x, coords.0);
        assert_eq!(y, coords.1);
    }
}
