use opencv::{
    calib3d,
    core::{DMatch, KeyPoint, KeyPointTraitConst, MatTraitConst, Point2f, VecN, Vector},
    features2d::{self, draw_keypoints, draw_matches_def, FlannBasedMatcher},
    highgui,
    imgcodecs::{self, IMREAD_GRAYSCALE},
    prelude::{DescriptorMatcherTraitConst, Feature2DTrait},
};

fn main() -> anyhow::Result<()> {
    let query_image =
        imgcodecs::imread("images/Vildkatten/VildkattenKarte02.png", IMREAD_GRAYSCALE)?;
    let train_image = imgcodecs::imread("images/Vildkatten/Vildkatten.jpg", IMREAD_GRAYSCALE)?;

    let mut sift = features2d::SIFT::create(0, 3, 0.04, 10., 1.6, false)?;
    let mut query_keypoints = Vector::<KeyPoint>::new();
    let mut query_descriptors = opencv::prelude::Mat::default();
    sift.detect_and_compute(
        &query_image,
        &opencv::prelude::Mat::default(),
        &mut query_keypoints,
        &mut query_descriptors,
        false,
    )?;

    let mut train_keypoints = Vector::<KeyPoint>::new();
    let mut train_descriptors = opencv::prelude::Mat::default();
    sift.detect_and_compute(
        &train_image,
        &opencv::prelude::Mat::default(),
        &mut train_keypoints,
        &mut train_descriptors,
        false,
    )?;

    let matcher = FlannBasedMatcher::new_def()?;
    let mut matches = Vector::new();
    let mut good_matches = Vector::<DMatch>::new();

    matcher.knn_train_match_def(&query_descriptors, &train_descriptors, &mut matches, 2)?;
    for i in 0..matches.len() {
        if matches.get(i).unwrap().get(0).unwrap().distance
            < 0.75 * matches.get(i).unwrap().get(1).unwrap().distance
        {
            good_matches.push(matches.get(i).unwrap().get(0).unwrap());
        }
    }

    let src_points = good_matches
        .iter()
        .map(|m| query_keypoints.get(m.query_idx as usize).unwrap().pt())
        .collect::<Vec<_>>();
    let dst_points = good_matches
        .iter()
        .map(|m| train_keypoints.get(m.train_idx as usize).unwrap().pt())
        .collect::<Vec<_>>();

    let h = calib3d::find_homography(
        &Vector::<Point2f>::from(dst_points),
        &Vector::<Point2f>::from(src_points),
        &mut opencv::prelude::Mat::default(),
        calib3d::RANSAC,
        5.0,
    )?;

    println!("{:?}", h);

    let template_corners = Vector::<Point2f>::from(vec![
        Point2f::new(0.0, 0.0),
        Point2f::new(train_image.cols() as f32, 0.0),
        Point2f::new(train_image.cols() as f32, train_image.rows() as f32),
        Point2f::new(0.0, train_image.rows() as f32),
    ]);
    let mut template_corners_transformed = Vector::<Point2f>::new();
    opencv::core::perspective_transform(&template_corners, &mut template_corners_transformed, &h)?;

    // println!("{:?}", template_corners_transformed);

    let mut out_image =
        imgcodecs::imread("images/Vildkatten/Vildkatten.jpg", imgcodecs::IMREAD_COLOR)?;
    // draw_matches_def(
    //     &query_image,
    //     &query_keypoints,
    //     &train_image,
    //     &train_keypoints,
    //     &good_matches.into(),
    //     &mut out_image,
    // )?;
    draw_keypoints(
        &query_image,
        &query_keypoints,
        &mut out_image,
        VecN::new(0., 0., 255., 0.),
        features2d::DrawMatchesFlags::DRAW_RICH_KEYPOINTS,
    )?;
    highgui::named_window("hello opencv!", 0)?;
    highgui::imshow("hello opencv!", &out_image)?;
    highgui::wait_key(0)?;

    Ok(())
}
