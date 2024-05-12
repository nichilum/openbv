use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

/// Returns a Gaussian kernel of size `size` x `size` and standard deviation `sigma`.
pub fn generate_gaussian_kernel(size: usize, sigma: f32) -> Vec<f32> {
    let mut kernel = vec![0.0; size * size];
    let offset = size / 2;
    let sigma_sq = 2.0 * sigma * sigma;
    let div = 1.0 / (std::f32::consts::PI * sigma_sq);

    kernel.par_iter_mut().enumerate().for_each(|(idx, k)| {
        let i = idx % size;
        let j = idx / size;
        let x = i as f32 - offset as f32;
        let y = j as f32 - offset as f32;
        *k = div * (-1.0 * (x * x + y * y) / sigma_sq).exp();
    });

    let norm = 1.0 / kernel.iter().sum::<f32>();
    kernel.par_iter_mut().for_each(|x| *x *= norm);

    kernel
}

#[rustfmt::skip]
pub const PLUS_FILTER: &[(i32, i32); 5] = &[
             (0,-1),
    (-1, 0), (0, 0), (1, 0),
             (0, 1),
];
