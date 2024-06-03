use std::collections::VecDeque;

use openbv::{math::histogram::HistogramFromImageExt, open_gray};

fn main() {
    let image = open_gray("./images/Set03_single.jpg").unwrap();

    let hist = image.histogram();

    // must be 2^i
    let bucket_count = 2usize.pow(3);

    let mut bucket_edges: Vec<u8> = vec![255];

    let mut q: VecDeque<(u8, usize, usize)> = VecDeque::new();
    q.push_back((hist.median(0, 255), 0, 255));

    loop {
        let (bucket_edge, left, right) = q.pop_front().unwrap();
        bucket_edges.push(bucket_edge);

        if bucket_edges.len() == bucket_count {
            break;
        }

        // left: left to edge
        q.push_back((
            hist.median(left, bucket_edge as usize),
            left,
            bucket_edge as usize,
        ));

        // right: edge to right
        q.push_back((
            hist.median(bucket_edge as usize, right),
            bucket_edge as usize,
            right,
        ));
    }

    bucket_edges.sort_unstable();

    // debug stuff
    println!("{:?}", bucket_edges);

    let mut last = 0;
    for edge in bucket_edges {
        let mut sum = 0;
        for index in last..=edge {
            sum += hist.0[index as usize];
        }
        println!("{sum}");
        last = edge;
    }
}
