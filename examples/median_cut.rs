use std::collections::VecDeque;

use openbv::{math::histogram::HistogramFromImageExt, open_gray};

fn main() {
    let image = open_gray("./images/Set03_single.jpg").unwrap();

    let hist = image.histogram();

    let bucket_count = 8;

    let mut bucket_edges: Vec<u8> = vec![];
    let mut q: VecDeque<(u8, usize, usize)> = VecDeque::new();
    q.push_back((hist.median(0, 255), 0, 255));

    let mut count = 1;
    while !q.is_empty() && count < bucket_count {
        let (bucket_edge, left, right) = q.pop_front().unwrap();
        bucket_edges.push(bucket_edge);

        // left: left to edge
        q.push_back(
            ((
                hist.median(left, bucket_edge as usize),
                left,
                bucket_edge as usize,
            )),
        );

        // right: edge to right
        q.push_back(
            ((
                hist.median(bucket_edge as usize, right),
                bucket_edge as usize,
                right,
            )),
        );

        count += 1;
    }

    bucket_edges.sort_unstable();

    println!("{:?}", bucket_edges);
}
