use std::collections::VecDeque;

use image::Luma;

use image::GrayImage;

use super::binarize::BinaryImage;

pub trait RegionLabelExt {
    fn region_label(&self) -> GrayImage;
}

impl RegionLabelExt for BinaryImage {
    fn region_label(&self) -> GrayImage {
        let mut new_img = self.0.clone();
        let mut label = 2;

        for x in 0..new_img.dimensions().0 {
            for y in 0..new_img.dimensions().1 {
                if new_img.get_pixel(x, y).0[0] == 255 {
                    flood_fill_queue(&mut new_img, x, y, label);
                    // TODO: what if img has more regions??
                    label = label.saturating_add(1);
                }
            }
        }

        new_img
    }
}

fn flood_fill_queue(binary_img: &mut image::GrayImage, x: u32, y: u32, label: u8) {
    let mut q: VecDeque<(i32, i32)> = VecDeque::from([(x as i32, y as i32)]);
    while !q.is_empty() {
        let (cur_x, cur_y) = q.pop_front().unwrap();
        if cur_x < 0
            || cur_x >= binary_img.dimensions().0 as i32
            || cur_y < 0
            || cur_y >= binary_img.dimensions().1 as i32
        {
            continue;
        }
        if binary_img.get_pixel(cur_x as u32, cur_y as u32).0[0] == 255 {
            binary_img.put_pixel(cur_x as u32, cur_y as u32, Luma([label]));
            q.push_back((cur_x + 1, cur_y));
            q.push_back((cur_x, cur_y + 1));
            q.push_back((cur_x - 1, cur_y));
            q.push_back((cur_x, cur_y - 1));
        }
    }
}
