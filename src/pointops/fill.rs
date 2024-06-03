use std::collections::VecDeque;

use image::{GenericImageView, Luma};

pub fn flood_fill(img: &mut image::GrayImage, x: i32, y: i32, value_to_fill: u8, value: u8) {
    let mut q = VecDeque::new();
    q.push_back((x, y));
    while !q.is_empty() {
        let (x, y) = q.pop_front().expect("queue is not empty");
        if x >= 0
            && y >= 0
            && img.in_bounds(x as u32, y as u32)
            && img.get_pixel(x as u32, y as u32)[0] == value_to_fill
        {
            img.put_pixel(x as u32, y as u32, Luma([value]));
            q.push_back((x + 1, y));
            q.push_back((x, y + 1));
            q.push_back((x, y - 1));
            q.push_back((x - 1, y));
        }
    }
}
