pub fn dilate(binary_img: &image::GrayImage) -> image::GrayImage {
    #[rustfmt::skip]
    let filter: [(i32, i32); 5] = [
                 (0,-1),
        (-1, 0), (0, 0), (1, 0),
                 (0, 1),
    ];
    let mut new_img = binary_img.clone();

    new_img
        .par_enumerate_pixels_mut()
        .for_each(|(x, y, pixel)| {
            'outer: for (x_filter, y_filter) in filter.iter() {
                let index_x = x as i32 - x_filter;
                let index_y = y as i32 - y_filter;

                if index_x >= 0
                    && index_x < binary_img.dimensions().0 as i32
                    && index_y >= 0
                    && index_y < binary_img.dimensions().1 as i32
                {
                    if binary_img.get_pixel(index_x as u32, index_y as u32).0[0] == 255 {
                        pixel.0[0] = 255;
                        break 'outer;
                    }
                }
            }
        });

    new_img
}
