use opencv::{
    calib3d, core::{KeyPoint, KeyPointTraitConst, MatTraitConst, Point2f, Vector}, features2d::{self, draw_matches_def, BFMatcher}, highgui, imgcodecs, imgproc, prelude::{DescriptorMatcherTraitConst, Feature2DTrait}
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

    let src_points = good_matches
        .iter()
        .map(|m| base_keypoints.get(m.query_idx as usize).unwrap().pt())
        .collect::<Vec<_>>();
    let dst_points = good_matches
        .iter()
        .map(|m| template_keypoints.get(m.train_idx as usize).unwrap().pt())
        .collect::<Vec<_>>();

    let h = calib3d::find_homography(
        &Vector::<Point2f>::from(src_points),
        &Vector::<Point2f>::from(dst_points),
        &mut opencv::prelude::Mat::default(),
        calib3d::RANSAC,
        3.0,
    )?;

    
    let template_corners = Vector::<Point2f>::from(vec![
        Point2f::new(0.0, 0.0),
        Point2f::new(template.cols() as f32, 0.0),
        Point2f::new(template.cols() as f32, template.rows() as f32),
        Point2f::new(0.0, template.rows() as f32),
    ]);
    let mut template_corners_transformed = Vector::<Point2f>::new();
    opencv::core::perspective_transform(
        &template_corners,
        &mut template_corners_transformed,
        &h,
    )?;

    println!("{:?}", template_corners_transformed);

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
