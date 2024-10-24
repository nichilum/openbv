use opencv::{
    calib3d,
    core::{sort, DMatch, KeyPoint, KeyPointTraitConst, MatTraitConst, Point2f, VecN, Vector},
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
    // draw_matches(
    //     &good_matches,
    //     &mut out_image,
    //     VecN::new(255., 255., 0., 255.),
    //     VecN::new(255., 0., 0., 255.),
    //     &Vector::new(),
    //     features2d::DrawMatchesFlags::DRAW_RICH_KEYPOINTS,
    // )?;

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
    opencv::core::perspective_transform(&template_corners, &mut template_corners_transformed, &h)?;

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
