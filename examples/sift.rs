use opencv::{
    core::{KeyPoint, VecN, Vector},
    features2d::{self, draw_keypoints},
    highgui, imgcodecs,
    prelude::Feature2DTrait,
    Result,
};

fn main() -> Result<()> {
    let image = imgcodecs::imread("images/laute.jpg", 0)?;
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

    let mut out_image = imgcodecs::imread("images/laute.jpg", imgcodecs::IMREAD_COLOR)?;
    draw_keypoints(
        &image,
        &keypoints,
        &mut out_image,
        VecN::new(255., 0., 0., 1.),
        features2d::DrawMatchesFlags::NOT_DRAW_SINGLE_POINTS,
    )
    .unwrap();

    highgui::named_window("hello opencv!", 0)?;
    highgui::imshow("hello opencv!", &out_image)?;
    highgui::wait_key(0)?;

    println!("{:?}", keypoints);

    Ok(())
}
