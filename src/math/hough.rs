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
    let max_r = ((width.pow(2) + height.pow(2)) as f32).sqrt();
    // println!("max_r: {max_r}");
    let n_theta = (std::f32::consts::PI / theta_step).ceil() as u32;
    let n_r = (max_r / r_step).ceil() as u32;
    // println!("nr: {n_r}");
    let mut acc = vec![vec![0; n_theta as usize]; n_r as usize];

    image.enumerate_pixels().for_each(|(x, y, pixel)| {
        if pixel[0] != 255 {
            return;
        }

        // println!("{}, {}", x, y);
        for i_theta in 0..n_theta {
            let theta = theta_step * i_theta as f32;
            // println!("{theta}");
            let r = x as f32 * theta.cos() + y as f32 * theta.sin();
            if r == 0. {
                println!("{}, {}", x, y);
            }
            let i_r = (r / r_step).round() as u32;
            acc[i_r as usize][i_theta as usize] += 1;
        }
    });

    // println!("{:?}", acc);

    let mut line_params = vec![];
    for (r, row) in acc.iter().enumerate() {
        for (theta, &value) in row.iter().enumerate() {
            if value >= threshold {
                let r = r as f32 * r_step;
                let theta = theta as f32 * theta_step;

                line_params.push(Line { r, theta });
            }
        }
    }

    line_params
}

pub fn draw_lines(image: &GrayImage, lines: &[Line]) -> RgbImage {
    let mut out_img = DynamicImage::ImageLuma8(image.clone()).to_rgb8();

    for line in lines {
        let r = line.r;
        let theta = line.theta;

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
            // println!("{},{},{},{}", x3, y3, x4, y4);
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

        if intersections.len() < 2 {
            continue;
        }

        draw_line(
            &mut out_img,
            intersections[0].0 as u32,
            intersections[0].1 as u32,
            intersections[1].0 as u32,
            intersections[1].1 as u32,
            image::Rgb([0, 255, 0]),
        );
    }

    out_img
}
