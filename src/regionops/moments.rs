use image::{GrayImage, Luma};

#[derive(Debug)]
pub struct HuMoments {
    pub hu_moments: [f64; 7],
    pub eccentricity: f64,
}

impl HuMoments {
    pub fn new(image: &GrayImage) -> Self {
        // https://www.researchgate.net/publication/321494904_Classification_of_Alzheimer_Disease_based_on_Normalized_Hu_Moment_Invariants_and_Multiclassifier
        // scale invariance
        let mu00 = Self::central_moment(0, 0, image);

        let mu02 = Self::central_moment(0, 2, image) / (mu00.powf((0. + 2.) / 2. + 1.));
        let mu03 = Self::central_moment(0, 3, image) / (mu00.powf((0. + 3.) / 2. + 1.));
        let mu11 = Self::central_moment(1, 1, image) / (mu00.powf((1. + 1.) / 2. + 1.));
        let mu12 = Self::central_moment(1, 2, image) / (mu00.powf((1. + 2.) / 2. + 1.));
        let mu20 = Self::central_moment(2, 0, image) / (mu00.powf((2. + 0.) / 2. + 1.));
        let mu21 = Self::central_moment(2, 1, image) / (mu00.powf((2. + 1.) / 2. + 1.));
        let mu30 = Self::central_moment(3, 0, image) / (mu00.powf((3. + 0.) / 2. + 1.));

        // rotation invariance
        let hu1 = mu20 + mu02;
        let hu2 = (mu20 - mu02).powi(2) + 4.0 * mu11.powi(2);
        let hu3 = (mu30 - 3.0 * mu12).powi(2) + (3.0 * mu21 - mu03).powi(2);
        let hu4 = (mu30 + mu12).powi(2) + (mu21 + mu03).powi(2);
        let hu5 = (mu30 - 3.0 * mu12)
            * (mu30 + mu12)
            * ((mu30 + mu12).powi(2) - 3.0 * (mu21 + mu03).powi(2))
            + (3.0 * mu21 - mu03)
                * (mu21 + mu03)
                * (3.0 * (mu30 + mu12).powi(2) - (mu21 + mu03).powi(2));
        let hu6 = (mu20 - mu02) * ((mu30 + mu12).powi(2) - (mu21 + mu03).powi(2))
            + 4.0 * mu11 * (mu30 + mu12) * (mu21 + mu03);
        let hu7 = (3.0 * mu21 - mu03)
            * (mu30 + mu12)
            * ((mu30 + mu12).powi(2) - 3.0 * (mu21 + mu03).powi(2))
            - (mu30 - 3.0 * mu12)
                * (mu21 + mu03)
                * (3.0 * (mu30 + mu12).powi(2) - (mu21 + mu03).powi(2));

        HuMoments {
            hu_moments: [hu1, hu2, hu3, hu4, hu5, hu6, hu7],
            eccentricity: (mu20 + mu02 + ((mu20 - mu02).powi(2) + 4. * mu11 * mu11).sqrt())
                / (mu20 + mu02 - ((mu20 - mu02).powi(2) + 4. * mu11 * mu11).sqrt()),
        }
    }

    pub fn raw_image_moment(i: u32, j: u32, image: &GrayImage) -> u32 {
        let mut moment = 0;
        for y in 0..image.height() {
            for x in 0..image.width() {
                if *image.get_pixel(x, y) == Luma([255]) {
                    moment += (x as u32).pow(i) * (y as u32).pow(j);
                }
            }
        }
        moment
    }

    pub fn central_moment(i: u32, j: u32, image: &GrayImage) -> f64 {
        let m00 = Self::raw_image_moment(0, 0, image) as f64;
        let m10 = Self::raw_image_moment(1, 0, image) as f64;
        let m01 = Self::raw_image_moment(0, 1, image) as f64;
        let x_bar = m10 / m00;
        let y_bar = m01 / m00;

        let mut moment = 0.0;
        for y in 0..image.height() {
            for x in 0..image.width() {
                if *image.get_pixel(x, y) == Luma([255]) {
                    moment +=
                        ((x as f64) - x_bar).powi(i as i32) * ((y as f64) - y_bar).powi(j as i32);
                }
            }
        }

        moment
    }
}
