use opencv::{
    core::{KeyPoint, Vector},
    features2d::{self, draw_matches_def, BFMatcher},
    highgui, imgcodecs,
    prelude::{DescriptorMatcherTraitConst, Feature2DTrait},
};

fn main() -> anyhow::Result<()> {
    let base_image = imgcodecs::imread("images/Vildkatten/Vildkatten.jpg", 0)?;
    let template = imgcodecs::imread("images/Vildkatten/VildkattenKarte01.png", 0)?;

    let mut sift = features2d::SIFT::create(0, 3, 0.04, 10., 15., false)?;

    let mut base_keypoints = Vector::<KeyPoint>::new();
    let mut base_descriptors = opencv::prelude::Mat::default();
    sift.detect_and_compute(
        &base_image,
        &opencv::prelude::Mat::default(),
        &mut base_keypoints,
        &mut base_descriptors,
        false,
    )?;

    let mut template_keypoints = Vector::<KeyPoint>::new();
    let mut template_descriptors = opencv::prelude::Mat::default();
    sift.detect_and_compute(
        &template,
        &opencv::prelude::Mat::default(),
        &mut template_keypoints,
        &mut template_descriptors,
        false,
    )?;

    // let flann_matcher = features2d::FlannBasedMatcher::new_def()?;
    // let mut matches: Vector<Vector<opencv::core::DMatch>> = Vector::new();
    // flann_matcher.knn_train_match(
    //     &template_descriptors,
    //     &base_descriptors,
    //     &mut matches,
    //     2,
    //     &opencv::prelude::Mat::default(),
    //     false
    // )?;
    // println!("Matches: {}", matches.len());

    let mut out_image =
        imgcodecs::imread("images/Vildkatten/Vildkatten.jpg", imgcodecs::IMREAD_COLOR)?;

    let matcher = BFMatcher::new_def()?;

    let mut matches = Vector::new();
    matcher.train_match_def(&base_descriptors, &template_descriptors, &mut matches)?;
    matches
        .as_mut_slice()
        .sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    let good_matches = matches.iter().take(10).collect::<Vec<_>>();

    draw_matches_def(
        &base_image,
        &base_keypoints,
        &template,
        &template_keypoints,
        &good_matches.into(),
        &mut out_image,
    )?;
    highgui::named_window("hello opencv!", 0)?;
    highgui::imshow("hello opencv!", &out_image)?;
    highgui::wait_key(0)?;

    Ok(())
}
