use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug)]
pub struct Histogram(pub Vec<usize>);
#[derive(Debug)]
pub struct NormalizedHistogram(pub Vec<f64>);
#[derive(Debug)]
pub struct CumulatedHistogram(pub Vec<usize>);
#[derive(Debug)]
pub struct CumulatedNormalizedHistogram(pub Vec<f64>);

pub trait HistogramFromImageExt {
    fn histogram(&self) -> Histogram;
}

impl HistogramFromImageExt for image::GrayImage {
    fn histogram(&self) -> Histogram {
        let (width, height) = self.dimensions();
        let mut histogram = vec![0; 256];

        for y in 0..height {
            for x in 0..width {
                let pixel = self.get_pixel(x, y)[0];
                histogram[pixel as usize] += 1;
            }
        }

        Histogram(histogram)
    }
}

impl Histogram {
    pub fn normalize(&self) -> NormalizedHistogram {
        let res = self.0.iter().sum::<usize>();
        NormalizedHistogram(
            self.0
                .par_iter()
                .map(|&x| x as f64 / res as f64)
                .collect::<Vec<_>>(),
        )
    }

    pub fn cumulate(&self) -> CumulatedHistogram {
        CumulatedHistogram(
            self.0
                .iter()
                .scan(0, |acc, &x| {
                    *acc += x;
                    Some(*acc)
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn median(&self, start: usize, end: usize) -> u8 {
        let all: usize = self.0[start..=end].iter().sum();

        let mut sum = 0;
        for index in start..=end {
            sum += self.0[index];

            if sum >= all / 2 {
                return index as u8;
            }
        }

        unreachable!("NOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO")
    }
}

impl NormalizedHistogram {
    pub fn cumulate(&self) -> CumulatedNormalizedHistogram {
        CumulatedNormalizedHistogram(
            self.0
                .iter()
                .scan(0.0, |acc, &x| {
                    *acc += x;
                    Some(*acc)
                })
                .collect::<Vec<_>>(),
        )
    }
}

impl CumulatedHistogram {
    pub fn normalize(&self) -> CumulatedNormalizedHistogram {
        todo!()
    }
}
