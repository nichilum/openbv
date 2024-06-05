use image::{Luma, Rgb};
use openbv::{colorops::hsv::HsvExt, open_gray, open_rgb};

use rand::{self, Rng};
use rayon::iter::ParallelIterator;

const COLOR_AMOUNT: usize = 2;
fn main() {
    let image = open_rgb("./images/Set03_single.jpg").unwrap();
    let hsv_img = image.to_hsv();

    // let mut centers = vec![];
    let mut clusters: Vec<Vec<&mut Rgb<f32>>>;

    let mut rng = rand::thread_rng();
}
