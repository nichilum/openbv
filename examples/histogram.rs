use openbv::{math::histogram::HistogramFromImageExt, open_gray};

fn main() {
    let image = open_gray("./images/Set03.jpg").unwrap();
    let histogram = image.histogram();
    println!("{:?}", histogram);

    let normalized_histogram = histogram.normalize();
    println!("{:?}", normalized_histogram);

    let cumulated_histogram = histogram.cumulate();
    println!("{:?}", cumulated_histogram);

    let cumulated_normalized_histogram = normalized_histogram.cumulate();
    println!("{:?}", cumulated_normalized_histogram);

}
