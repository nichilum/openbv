use opencv::{
    core::{KeyPoint, Vector},
    features2d, highgui, imgcodecs,
    prelude::Feature2DTrait,
    Result,
};

fn main() -> Result<()> {
    let image: opencv::prelude::Mat = imgcodecs::imread("images/laute.jpg", 0)?;
    // highgui::named_window("hello opencv!", 0)?;
    // highgui::imshow("hello opencv!", &image)?;
    // highgui::wait_key(10000)?;

    let mut sift = features2d::SIFT::create(0, 3, 0.04, 10., 1.6, false).unwrap();

    let mut keypoints = Vector::<KeyPoint>::new();
    let mut descriptors = Vector::<opencv::prelude::Mat>::new();
    let _ = sift.detect_and_compute(
        &image,
        &opencv::prelude::Mat::default(),
        &mut keypoints,
        &mut descriptors,
        false,
    );

    println!("{:?}", keypoints);

    Ok(())
}
