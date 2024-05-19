use image::{DynamicImage, RgbaImage};

use super::binarize::BinaryImage;
use crate::math::hull::{approx_hull, graham_scan};

#[derive(Debug)]
pub struct Contour {
    pub points: Vec<(u32, u32)>,
    pub label: u8,
}

#[derive(Debug)]
pub struct Hull {
    pub points: Vec<(u32, u32)>,
}

pub struct Moments;

impl Contour {
    pub fn convex_hull(&self) -> Hull {
        let hull = graham_scan(&self.points);
        Hull { points: hull }
    }

    pub fn poly_hull(&self, epsilon: f32) -> Hull {
        let hull = approx_hull(&self.points, epsilon);
        Hull { points: hull }
    }

    pub fn contour_area(&self) -> u32 {
        // from: https://github.com/opencv/opencv/blob/76d9f7aaeb8c9ba8aea80bdb155b60c78da1e309/modules/imgproc/src/shapedescr.cpp#L308
        let mut area = 0;
        for i in 0..self.points.len() {
            let p1 = self.points[i];
            let p2 = self.points[(i + 1) % self.points.len()];
            area += p1.0 as i32 * p2.1 as i32 - p1.1 as i32 * p2.0 as i32;
        }
        (area / 2).unsigned_abs()
    }

    pub fn arc_length(&self) -> usize {
        self.points.len()
    }

    pub fn get_center(&self) -> (u32, u32) {
        let mut sum_x = 0;
        let mut sum_y = 0;
        for (x, y) in &self.points {
            sum_x += x;
            sum_y += y;
        }

        (
            sum_x / self.points.len() as u32,
            sum_y / self.points.len() as u32,
        )
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

impl Hull {
    // works better for poly hulls than convex hulls
    pub fn get_center(&self) -> (u32, u32) {
        let mut sum_x = 0;
        let mut sum_y = 0;
        for (x, y) in &self.points {
            sum_x += x;
            sum_y += y;
        }

        (
            sum_x / self.points.len() as u32,
            sum_y / self.points.len() as u32,
        )
    }
}

pub trait ContourExt {
    fn find_contours(&self) -> (Vec<Contour>, Vec<Contour>);
    fn trace_contour(
        &self,
        output_img: &mut RgbaImage,
        x: u32,
        y: u32,
        label: u32,
        internal: bool,
    ) -> Contour;
    fn handle_fg_pixel(
        &self,
        output_img: &mut RgbaImage,
        x: u32,
        y: u32,
        label: &mut u32,
        r: &mut u32,
        outer_contours: &mut Vec<Contour>,
    );
    fn handle_bg_pixel(
        &self,
        output_img: &mut RgbaImage,
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
    /// (inner, outer)
    fn find_contours(&self) -> (Vec<Contour>, Vec<Contour>) {
        let img: &image::ImageBuffer<image::Luma<u8>, Vec<u8>> = &self.0;
        let mut output_img = DynamicImage::ImageLuma8(self.0.clone()).to_rgba8();
        output_img.pixels_mut().for_each(|x| {
            *x = image::Rgba([0, 0, 0, 0]);
        });

        let mut outer_contours = vec![];
        let mut inner_contours = vec![];

        let mut r = 1;
        for y in 0..img.dimensions().1 {
            let mut label = 0;
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

        (inner_contours, outer_contours)
    }

    fn handle_fg_pixel<'a>(
        &self,
        output_img: &mut RgbaImage,
        x: u32,
        y: u32,
        label: &'a mut u32,
        r: &'a mut u32,
        outer_contours: &'a mut Vec<Contour>,
    ) {
        if *label != 0 {
            let cur_pixel_alpha = output_img.get_pixel(x, y)[3];
            output_img.put_pixel(
                x,
                y,
                image::Rgba([*label as u8, *label as u8, *label as u8, cur_pixel_alpha]),
            )
        } else {
            *label = output_img.get_pixel(x, y)[0] as u32;
            if *label == 0 {
                *r += 1;
                *label = *r;
                let contour = self.trace_contour(output_img, x, y, *label, false);
                outer_contours.push(contour);
                // maybe doppelt gemoppelt
                output_img.put_pixel(
                    x,
                    y,
                    image::Rgba([*label as u8, *label as u8, *label as u8, 255]),
                )
            }
        }
    }

    fn handle_bg_pixel(
        &self,
        output_img: &mut RgbaImage,
        x: u32,
        y: u32,
        label: &mut u32,
        inner_contours: &mut Vec<Contour>,
    ) {
        if *label != 0 {
            if output_img.get_pixel(x, y)[0] == 0
                && output_img.get_pixel(x.saturating_sub(1), y)[3] != 255
            {
                let contour = self.trace_contour(output_img, x.saturating_sub(1), y, *label, true);
                // inner contour is counter clockwise
                // for the convex hull clockwise rotation is needed
                let rev_points = contour
                    .points
                    .into_iter()
                    .rev()
                    .collect::<Vec<(u32, u32)>>();
                inner_contours.push(Contour {
                    points: rev_points,
                    label: *label as u8,
                });
            }
            *label = 0;
        }
    }

    fn trace_contour(
        &self,
        output_img: &mut RgbaImage,
        x: u32,
        y: u32,
        label: u32,
        internal: bool,
    ) -> Contour {
        let start = (x, y);
        let second = if let Some((x, y)) = self.tracer(x, y, (0, 0), true, internal) {
            (x, y)
        } else {
            return Contour {
                points: vec![start],
                label: label as u8,
            };
        };

        let mut points = vec![start, second];
        loop {
            let cur = *points.last().unwrap();
            let prev = points[points.len() - 2];
            let next = self.tracer(cur.0, cur.1, prev, false, internal);
            if let Some((x, y)) = next {
                // use next so start point isn't twice in vector
                if next.unwrap() == start {
                    break;
                }

                // if cur == start && (x, y) == second {
                //     break;
                // } more triage needed

                points.push((x, y));
                output_img.put_pixel(
                    x,
                    y,
                    image::Rgba([label as u8, label as u8, label as u8, 255]),
                )
            }
        }

        Contour {
            points,
            label: label as u8,
        }
    }

    fn tracer(
        &self,
        x: u32,
        y: u32,
        prev: (u32, u32),
        start: bool,
        internal: bool,
    ) -> Option<(u32, u32)> {
        let init_index = if !start {
            let dif = (prev.0 as i32 - x as i32, prev.1 as i32 - y as i32);
            let d = match dif {
                (1, 0) => 0,
                (1, 1) => 1,
                (0, 1) => 2,
                (-1, 1) => 3,
                (-1, 0) => 4,
                (-1, -1) => 5,
                (0, -1) => 6,
                (1, -1) => 7,
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

        let mut index = init_index;
        let x = x as i32;
        let y = y as i32;
        let img = &self.0;
        for _ in 0..8 {
            let (dx, dy) = match index {
                0 => (1, 0),
                1 => (1, 1),
                2 => (0, 1),
                3 => (-1, 1),
                4 => (-1, 0),
                5 => (-1, -1),
                6 => (0, -1),
                7 => (1, -1),
                _ => unreachable!(),
            };

            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0
                && ny >= 0
                && nx < img.dimensions().0 as i32
                && ny < img.dimensions().1 as i32
                && img.get_pixel(nx as u32, ny as u32)[0] == 255
            {
                return Some((nx as u32, ny as u32));
            }

            index = (index + 1) % 8;
        }

        None
    }
}
