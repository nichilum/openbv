use std::collections::VecDeque;

use image::GrayImage;
use rayon::iter::ParallelIterator;

use crate::math::histogram::HistogramFromImageExt;

pub trait MedianCutExt {
    fn median_cut(&mut self, bucket_count: usize);
}

impl MedianCutExt for GrayImage {
    fn median_cut(&mut self, bucket_count: usize) {
        let hist = self.histogram();

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
            q.push_back((hist.median(left, bucket_edge), left, bucket_edge));

            // right: edge to right
            q.push_back((hist.median(bucket_edge, right), bucket_edge, right));
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
            let median = hist.median(last, edge);
            medians.push(median);
            last = edge;
        }

        self.par_pixels_mut().for_each(|pixel| {
            let value = pixel[0] as usize;
            let mut last = 0;
            for (index, &edge) in bucket_edges.iter().enumerate() {
                if value >= last && value <= edge {
                    *pixel = image::Luma([medians[index] as u8]);
                    break;
                }
                last = edge;
            }
        });
    }
}
