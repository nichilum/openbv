use opencv::{
    core::{sort, DMatch, KeyPoint, VecN, Vector},
    features2d::{
        self, draw_keypoints, draw_matches, draw_matches_def, draw_matches_knn,
        draw_matches_with_thickness_def, BFMatcher, FlannBasedMatcher,
    },
    highgui, imgcodecs,
    prelude::{DescriptorMatcherTrait, DescriptorMatcherTraitConst, Feature2DTrait},
    Result,
};

fn main() -> anyhow::Result<()> {
    let base_image = imgcodecs::imread("images/Vildkatten/Vildkatten.jpg", 1)?;
    let template = imgcodecs::imread("images/Vildkatten/VildkattenKarte01.png", 1)?;

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

    let matcher = FlannBasedMatcher::new_def()?;
    let mut matches = Vector::new();
    let mut good_matches = Vector::<DMatch>::new();

    matcher.knn_train_match_def(&base_descriptors, &template_descriptors, &mut matches, 2)?;
    for i in 0..matches.len() {
        if matches.get(i).unwrap().get(0).unwrap().distance
            < 0.75 * matches.get(i).unwrap().get(1).unwrap().distance
        {
            good_matches.push(matches.get(i).unwrap().get(0).unwrap());
        }
    }

    // debug stuff
    let mut out_image =
        imgcodecs::imread("images/Vildkatten/Vildkatten.jpg", imgcodecs::IMREAD_COLOR)?;

    // draw_keypoints(
    //     &template,
    //     &template_keypoints,
    //     &mut out_image,
    //     VecN::new(255., 0., 0., 0.),
    //     features2d::DrawMatchesFlags::DRAW_RICH_KEYPOINTS,
    // )?;
    draw_matches(
        &base_image,
        &base_keypoints,
        &template,
        &template_keypoints,
        &good_matches,
        &mut out_image,
        VecN::new(255., 255., 0., 255.),
        VecN::new(255., 0., 0., 255.),
        &Vector::new(),
        features2d::DrawMatchesFlags::DRAW_RICH_KEYPOINTS,
    )?;
    highgui::named_window("hello opencv!", 0)?;
    highgui::imshow("hello opencv!", &out_image)?;
    highgui::wait_key(0)?;

    Ok(())
}
