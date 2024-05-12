pub fn region_label(
    binary_img: &image::GrayImage,
    f: fn(&mut image::GrayImage, u32, u32, u8),
) -> image::GrayImage {
    let mut new_img = binary_img.clone();
    let mut label = 30;

    for x in 0..new_img.dimensions().0 {
        for y in 0..new_img.dimensions().1 {
            if new_img.get_pixel(x, y).0[0] == 255 {
                flood_fill_queue(&mut new_img, x, y, label);
                // f(&mut new_img, x, y, label);
                // flood_fill_stack(&mut new_img, x, y, label);
                // flood_fill_rec(&mut new_img, x as i32, y as i32, label);
                label = ((label as u32 + 25) % 140 + 35) as u8;
                // return new_img; // only use for in progress images
            }
        }
    }

    new_img
}

pub fn flood_fill_queue(binary_img: &mut image::GrayImage, x: u32, y: u32, label: u8) {
    let mut q: VecDeque<(i32, i32)> = VecDeque::from([(x as i32, y as i32)]);
    // let mut c = 0;
    while !q.is_empty() {
        // break for in progress images
        // c += 1;
        // if c == 200000 {
        //     break;
        // }
        // actual flood fill
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
