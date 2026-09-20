use crate::utils::utils::vec_to_matrix;
use faer::prelude::Mat;
use nalgebra::DVector;
use polars::polars_utils::itertools::Itertools;
use rand::distr::Distribution;
use rand::prelude::ThreadRng;
use statrs::distribution::MultivariateNormal;

pub fn exercise03_12() {
    fn generate_t_multivariable_samples(
        t: usize,
        distribution: &impl Distribution<DVector<f64>>,
    ) -> Vec<Mat<f64>> {
        let mut rng: ThreadRng = rand::rng();
        (0..t)
            .map(|_| {
                let sample = distribution.sample(&mut rng);
                vec_to_matrix(sample.as_slice())
            })
            .collect()
    }
    let dimension = 10;
    let number_of_samples = 20;
    let true_mean = vec![0.1; dimension];
    let true_cov: Mat<f64> = Mat::identity(dimension, dimension);
    let true_cov_vec: Vec<f64> = true_cov
        .col_iter()
        .map(|col| col.iter().map(|&x| x).collect_vec())
        .flatten()
        .collect::<Vec<f64>>();
    let distribution = MultivariateNormal::new(true_mean, true_cov_vec).unwrap();
    let data = generate_t_multivariable_samples(number_of_samples, &distribution);
}
