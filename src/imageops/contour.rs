use image::GrayImage;

use super::binarize::BinaryImage;

pub struct Contour {
    pub points: Vec<(u32, u32)>,
}

pub struct Moments;

pub trait ContourExt {
    fn find_contours(&self) -> Vec<Contour>;
    fn trace_contour(&self, x: u32, y: u32, internal: bool) -> Contour;
    fn handle_fg_pixel(
        &self,
        output_img: &mut GrayImage,
        x: u32,
        y: u32,
        label: &mut u32,
        r: &mut u32,
        outer_contours: &mut Vec<Contour>,
    );
    fn handle_bg_pixel(
        &self,
        output_img: &mut GrayImage,
        x: u32,
        y: u32,
        label: &mut u32,
        inner_contours: &mut Vec<Contour>,
    );
    fn tracer(
        &self,
        x: u32,
        y: u32,
        prev: (u32, u32),
        start: bool,
        internal: bool,
    ) -> Option<(u32, u32)>;
}

impl ContourExt for BinaryImage {
    fn find_contours(&self) -> Vec<Contour> {
        let img: &image::ImageBuffer<image::Luma<u8>, Vec<u8>> = &self.0;
        let mut output_img = self.0.clone();
        output_img.pixels_mut().for_each(|x| {
            *x = image::Luma([0]);
        });
        
        let mut outer_contours = vec![];
        let mut inner_contours = vec![];

        let mut r = 1;
        for y in 0..img.dimensions().1 {
            let mut label = 50;
            for x in 0..img.dimensions().0 {
                if img.get_pixel(x, y)[0] == 255 {
                    self.handle_fg_pixel(
                        &mut output_img,
                        x,
                        y,
                        &mut label,
                        &mut r,
                        &mut outer_contours,
                    );
                } else {
                    self.handle_bg_pixel(&mut output_img, x, y, &mut label, &mut inner_contours);
                }
            }
        }

        output_img.save("output.png").unwrap();

        outer_contours
    }

    fn handle_fg_pixel<'a>(
        &self,
        output_img: &mut GrayImage,
        x: u32,
        y: u32,
        label: &'a mut u32,
        r: &'a mut u32,
        outer_contours: &'a mut Vec<Contour>,
    ) {
        if *label != 0 {
            output_img.put_pixel(x, y, image::Luma([*label as u8]))
        } else {
            *label = output_img.get_pixel(x, y)[0] as u32;
            if *label == 0 {
                // *r = (*r + 1) % 254 + 1;
                *r = *r + 1;
                *label = *r;
                let contour = self.trace_contour(x, y, false);
                outer_contours.push(contour);
                output_img.put_pixel(x, y, image::Luma([*label as u8]))
            }
        }
    }

    fn handle_bg_pixel(
        &self,
        output_img: &mut GrayImage,
        x: u32,
        y: u32,
        label: &mut u32,
        inner_contours: &mut Vec<Contour>,
    ) {
        if *label != 0 {
            if output_img.get_pixel(x, y)[0] == 0 {
                let contour = self.trace_contour(x.saturating_sub(1), y, true);
                inner_contours.push(contour);
            }
            *label = 0;
        }
    }

    fn trace_contour(&self, x: u32, y: u32, internal: bool) -> Contour {
        let start = (x, y);
        let second = if let Some((x, y)) = self.tracer(x, y, (0, 0), true, internal) {
            (x, y)
        } else {
            return Contour {
                points: vec![start],
            };
        };

        let mut points = vec![start, second];
        loop {
            let cur = (points.last().unwrap().0, points.last().unwrap().1);
            let prev = (points[points.len() - 2].0, points[points.len() - 2].1);
            let next = self.tracer(cur.0, cur.1, prev, false, internal);
            if let Some((x, y)) = next {
                if cur == start && (x, y) == second {
                    break;
                }

                points.push((x, y));
            }
        }

        Contour { points }
    }

    fn tracer(
        &self,
        x: u32,
        y: u32,
        prev: (u32, u32),
        start: bool,
        internal: bool,
    ) -> Option<(u32, u32)> {
        let init_pos = if !start {
            let dif = (prev.0 as i32 - x as i32, prev.1 as i32 - y as i32);
            let d = match dif {
                (1, 0) => 0,
                (1, -1) => 1,
                (0, -1) => 2,
                (-1, -1) => 3,
                (-1, 0) => 4,
                (-1, 1) => 5,
                (0, 1) => 6,
                (1, 1) => 7,
                _ => unreachable!(),
            };

            (d + 2) % 8
        } else if internal {
            3
            // (-1, -1)
        } else {
            7
            // (1, 1)
        };

        let mut pos = init_pos;
        let x = x as i32;
        let y = y as i32;
        let img = &self.0;
        for _ in 0..8 {
            let (dx, dy) = match pos {
                0 => (1, 0),
                1 => (1, -1),
                2 => (0, -1),
                3 => (-1, -1),
                4 => (-1, 0),
                5 => (-1, 1),
                6 => (0, 1),
                7 => (1, 1),
                _ => unreachable!(),
            };

            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0
                && ny >= 0
                && nx < img.dimensions().0 as i32
                && ny < img.dimensions().1 as i32
            {
                if img.get_pixel(nx as u32, ny as u32)[0] == 255 {
                    return Some((nx as u32, ny as u32));
                }
            }

            pos = (pos + 1) % 8;
        }

        None
    }
}

impl Contour {
    pub fn convex_hull(&self) {
        todo!()
    }
    pub fn approx_poly_db(&self) {
        todo!()
    }
    pub fn contour_area(&self) {
        todo!()
    }
    pub fn arc_length(&self) {
        todo!()
    }
    pub fn moments(&self) -> Moments {
        todo!()
    }
}

impl Moments {
    pub fn hu_moments(&self) {
        todo!()
    }
}
