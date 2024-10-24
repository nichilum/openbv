use opencv::{
    core::{sort, KeyPoint, VecN, Vector},
    features2d::{self, draw_keypoints, draw_matches, draw_matches_def, draw_matches_knn, BFMatcher},
    highgui, imgcodecs,
    prelude::{DescriptorMatcherTrait, DescriptorMatcherTraitConst, Feature2DTrait},
    Result,
};

fn main() -> anyhow::Result<()> {
    let base_image = imgcodecs::imread("images/Vildkatten/Vildkatten.jpg", 0)?;
    let template = imgcodecs::imread("images/Vildkatten/VildkattenKarte01.png", 0)?;

    let mut sift = features2d::SIFT::create(0, 3, 0.04, 10., 1.6, false)?;

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

    let mut out_image = imgcodecs::imread("images/Vildkatten/Vildkatten.jpg", imgcodecs::IMREAD_COLOR)?;

    let matcher = BFMatcher::new_def()?;

    let mut matches = Vector::new();
    matcher.train_match_def(&base_descriptors, &template_descriptors, &mut matches)?;
    let mut good_matches = matches.as_slice().clone();
    good_matches.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());


    draw_matches_def(&base_image, &base_keypoints, &template, &template_keypoints, &matches.into(), &mut out_image)?;
    highgui::named_window("hello opencv!", 0)?;
    highgui::imshow("hello opencv!", &out_image)?;
    highgui::wait_key(0)?;

    Ok(())
}
