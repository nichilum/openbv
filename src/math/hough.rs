use std::f32::consts::PI;

use image::{DynamicImage, GrayImage, RgbImage};

use crate::binary_image::draw_line;

#[derive(Debug)]
pub struct Line {
    /// Distance from the origin
    pub r: f32,

    /// Angle in radians
    pub theta: f32,
}

pub fn hough(image: &GrayImage, r_step: f32, theta_step: f32, threshold: u32) -> Vec<Line> {
    let (width, height) = image.dimensions();
    let center = (width / 2, height / 2);
    let max_r = (((width / 2).pow(2) + (height / 2).pow(2)) as f32).sqrt();
    let n_theta = (std::f32::consts::PI / theta_step).ceil() as u32;
    let n_r = (2. * max_r / r_step).ceil() as u32;
    let mut acc = vec![0; n_r as usize * n_theta as usize];

    image.enumerate_pixels().for_each(|(x, y, pixel)| {
        if pixel[0] != 255 {
            return;
        }

        let (x, y) = (x as i32 - center.0 as i32, y as i32 - center.1 as i32);
        for i_theta in 0..n_theta {
            let theta = theta_step * i_theta as f32;
            let r = x as f32 * theta.cos() + y as f32 * theta.sin();
            let i_r = (n_r as f32 / 2. + (r / r_step).round()) as u32;
            acc[i_r as usize * n_theta as usize + i_theta as usize] += 1;
        }
    });

    let line_params = acc
        .iter()
        .filter(|&&value| value >= threshold)
        .enumerate()
        .map(|(i, _)| {
            let r = (i as u32 / n_theta) as f32 * r_step - max_r;
            let theta = (i as u32 % n_theta) as f32 * theta_step;

            Line {
                r: r.abs(),
                theta: theta,
            }
        })
        .collect::<Vec<_>>();

    line_params
}

pub fn draw_lines(image: &GrayImage, lines: &[Line]) -> RgbImage {
    let mut out_img = DynamicImage::ImageLuma8(image.clone()).to_rgb8();

    for line in lines {
        let r = line.r;
        let theta = line.theta;
        // when set exactly to 90 degrees it fails ._.

        // line points
        let x1 = theta.cos() * r;
        let y1 = theta.sin() * r;
        let x2 = x1 + y1;
        let y2 = y1 - x1;

        let mut intersections: Vec<(f32, f32)> = vec![];
        let walls: Vec<(f32, f32, f32, f32)> = vec![
            (0., 0., image.width() as f32 - 1., 0.),
            (
                image.width() as f32 - 1.,
                0.,
                image.width() as f32 - 1.,
                image.height() as f32 - 1.,
            ),
            (0., 0., 0., image.width() as f32 - 1.),
            (
                0.,
                image.height() as f32 - 1.,
                image.width() as f32 - 1.,
                image.height() as f32 - 1.,
            ),
        ];

        for (x3, y3, x4, y4) in walls {
            println!("{},{},{},{}", x3, y3, x4, y4);
            let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
            if denom != 0. {
                intersections.push((
                    ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4)) / denom,
                    ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4)) / denom,
                ));
            }
        }

        let intersections = intersections
            .iter()
            .filter(|(x, y)| {
                *x < image.width() as f32 && *x >= 0. && *y < image.height() as f32 && *y >= 0.
            })
            .collect::<Vec<&(f32, f32)>>();

        assert!(intersections.len() == 2);

        draw_line(
            &mut out_img,
            intersections[0].0 as u32,
            intersections[0].1 as u32,
            intersections[1].0 as u32,
            intersections[1].1 as u32,
            image::Rgb([0, 255, 0]),
        );

        let x1 = 0;
        let y1 = (r / theta.cos()) as u32;
        let x2 = image.width();
        let y2 = ((r - x2 as f32 * theta.cos()) / theta.sin()) as u32;
        draw_line(&mut out_img, x1, y1, x2, y2, image::Rgb([255, 0, 0]));
    }

    out_img
}
