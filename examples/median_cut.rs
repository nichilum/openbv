use std::collections::VecDeque;

use image::Luma;
use openbv::{math::histogram::HistogramFromImageExt, open_gray};
use rayon::iter::ParallelIterator;

fn main() {
    let mut image = open_gray("./images/Set03_single.jpg").unwrap();

    let hist = image.histogram();

    // must be 2^i (no)
    let bucket_count = 2usize;

    // stores the upper limit of each bucket (inclusive)
    let mut bucket_edges = vec![255];

    let mut q = VecDeque::new();
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
    // println!("{:?}", bucket_edges);

    // let mut last = 0;
    // for edge in bucket_edges {
    //     let mut sum = 0;
    //     for index in last..=edge {
    //         sum += hist.0[index as usize];
    //     }
    //     println!("{sum}");
    //     last = edge;
    // }

    let mut medians = Vec::new();
    let mut last = 0;
    for &edge in &bucket_edges {
        let median = hist.median(last, edge as usize);
        medians.push(median);
        last = edge;
    }
    // println!("{medians:?}");

    image.par_pixels_mut().for_each(|pixel| {
        let value = pixel[0] as usize;
        let mut last = 0;
        for (index, &edge) in bucket_edges.iter().enumerate() {
            if value >= last && value <= edge {
                *pixel = Luma([medians[index] as u8]);
                break;
            }
            last = edge;
        }
    });

    image.save("export/median_cut.png").unwrap();
}
