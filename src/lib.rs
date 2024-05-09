use image::{io::Reader as ImageReader, GenericImageView, GrayImage, Luma};

use rayon::iter::ParallelIterator;
use std::collections::VecDeque;

pub struct OpenBV;

impl OpenBV {
    fn open_rgb(path: &str) {}
    fn open_gray(path: &str) -> anyhow::Result<GrayImage> {
        Ok(ImageReader::open(path)?.decode()?.to_luma8())
    }
}
