use bevy::prelude::{IVec2, Vec2};

pub fn key_from_coords(x: i32, y: i32) -> u64 {
    ((x as u64) << 32) | y as u64
}

pub fn coords_from_key(key: u64) -> (i32, i32) {
    let x = (key & 0xFFFFFFFF00000000) >> 32;
    let y = key & 0xFFFFFFFF;

    (x as i32, y as i32)
}

pub fn world_pos_to_coords(
    world_pos: Vec2,
    chunk_size: usize,
    chunk_scale: usize,
) -> (IVec2, IVec2) {
    let mut pixel_pos = (world_pos / chunk_scale as f32).floor().as_ivec2();
    println!("PP1: {}", pixel_pos);
    pixel_pos += (chunk_size / 2) as i32;
    println!("PP1: {}", pixel_pos);
    let x = global_pixel_to_chunk_dim(pixel_pos.x, chunk_size);
    let y = global_pixel_to_chunk_dim(pixel_pos.y, chunk_size);
    let chunk_coord = IVec2::new(x, y);
    println!("CC: {}", chunk_coord);

    let chunk_pixel = pixel_pos - chunk_coord * IVec2::new(chunk_size as i32, chunk_size as i32);
    println!("CP: {}", chunk_pixel);

    (chunk_coord, chunk_pixel)
}

fn global_pixel_to_chunk_dim(
    pixel_pos_single: i32,
    chunk_size: usize,
) -> i32 {
    if pixel_pos_single >= 0 {
        pixel_pos_single / chunk_size as i32
    }
    else {
        (pixel_pos_single - chunk_size as i32 - 1) / chunk_size as i32
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{IVec2, Vec2};

    use super::{coords_from_key, key_from_coords, world_pos_to_coords};

    #[test]
    fn test_key_from_coords() {
        let x = 17;
        let y = 9;
        let key = key_from_coords(x, y);
        let coords = coords_from_key(key);
        assert_eq!(x, coords.0);
        assert_eq!(y, coords.1);
    }

    #[test]
    fn test_world_pos_to_coords() {
        let world_pos = Vec2::new(0., 0.);
        let chunk_size = 10;
        let chunk_scale = 2;
        let (chunk_coord, _chunk_pixel) = world_pos_to_coords(world_pos, chunk_size, chunk_scale);
        assert_eq!(chunk_coord, IVec2::new(0, 0));
    }

    #[test]
    fn test_world_pos_to_negative_coords() {
        let world_pos = Vec2::new(-20., 0.);
        let chunk_size = 10;
        let chunk_scale = 2;
        let (chunk_coord, _chunk_pixel) = world_pos_to_coords(world_pos, chunk_size, chunk_scale);
        assert_eq!(chunk_coord, IVec2::new(-1, 0));
    }

    #[test]
    fn test_world_pos_to_negative_coords_border() {
        let world_pos_a = Vec2::new(-10., 0.);
        let world_pos_b = Vec2::new(-10.5, 0.);
        let chunk_size = 10;
        let chunk_scale = 2;
        let (chunk_coord_a, _chunk_pixel_a) = world_pos_to_coords(world_pos_a, chunk_size, chunk_scale);
        let (chunk_coord_b, _chunk_pixel_b) = world_pos_to_coords(world_pos_b, chunk_size, chunk_scale);
        assert_eq!(chunk_coord_a, IVec2::new(0, 0));
        assert_eq!(chunk_coord_b, IVec2::new(-1, 0));
    }
}
