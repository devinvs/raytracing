use super::vector::Color;

fn clamp(target: f32, min: f32, max: f32) -> f32 {
    if target < min {
        return min;
    } else if target > max {
        return max;
    } else {
        return target
    }
}

pub fn write_color(pixel_color: Color, samples_per_pixel: usize) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / samples_per_pixel as f32;
    let r = (scale*r).sqrt();
    let g = (scale*g).sqrt();
    let b = (scale*b).sqrt();

    println!("{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)) as usize,
        (256.0 * clamp(g, 0.0, 0.999)) as usize,
        (256.0 * clamp(b, 0.0, 0.999)) as usize
    );
}


