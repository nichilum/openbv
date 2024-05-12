use std::path::Path;

use image::{io::Reader as ImageReader, GrayImage, RgbImage};

pub mod imageops;
pub mod math;

pub fn open_rgb<P>(path: P) -> anyhow::Result<RgbImage>
where
    P: AsRef<Path>,
{
    Ok(ImageReader::open(path)?.decode()?.to_rgb8())
}

pub fn open_gray<P>(path: P) -> anyhow::Result<GrayImage>
where
    P: AsRef<Path>,
{
    Ok(ImageReader::open(path)?.decode()?.to_luma8())
}
