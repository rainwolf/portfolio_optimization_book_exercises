use crate::utils::utils::generate_d_dimensional_normal_samples;
use statrs::distribution::Normal;

pub fn exercise03_07() {
    // Consider an N-dimensional i.i.d. time series with zero mean and identity covariance matrix
    //     Generate Gaussian data for N=10 and T=50 and estimate the covariance matrix Σ
    //   via the Gaussian ML estimator and the heavy-tailed ML estimator.
    // Run the experiment multiple times and compute the mean squared error of the estimators.

    let n = Normal::new(0.0, 1.0).unwrap();
    let d = 10;
    let number_of_iid_vars = 50;
    let number_of_experiments = 1000;
    let data = generate_d_dimensional_normal_samples(&n, d, number_of_iid_vars);
    fn estimate_covariance_matrix_gaussian_ml_estimator(data: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let d = data[0].len();
        let mut covariance_matrix = vec![vec![0.0; d]; d];
        for i in 0..d {
            for j in 0..d {
                let covariance_ij =
                    data.iter().map(|point| point[i] * point[j]).sum::<f64>() / (data.len() as f64);
                covariance_matrix[i][j] = covariance_ij;
                covariance_matrix[j][i] = covariance_ij; // since the covariance matrix is symmetric
            }
        }
        covariance_matrix
    }
    fn covariance_matrix_mse_to_true_covariance_matrix(
        covariance_matrix: &Vec<Vec<f64>>,
        true_covariance_matrix: &Vec<Vec<f64>>,
    ) -> f64 {
        covariance_matrix
            .iter()
            .zip(true_covariance_matrix)
            .map(|(row_estimated, row_true)| {
                row_estimated
                    .iter()
                    .zip(row_true)
                    .map(|(&estimated, &true_value)| (estimated - true_value).powi(2))
                    .sum::<f64>()
            })
            .sum::<f64>()
            / (covariance_matrix.len() * covariance_matrix[0].len()) as f64
    }
    let mut true_variance_matrix = vec![vec![0.0; d]; d];
    for i in 0..d {
        true_variance_matrix[i][i] = 1.0;
    }
    let covariance_matrix_gaussian_ml_estimator =
        estimate_covariance_matrix_gaussian_ml_estimator(&data);
    let mse_gaussian_ml_estimator = covariance_matrix_mse_to_true_covariance_matrix(
        &covariance_matrix_gaussian_ml_estimator,
        &true_variance_matrix,
    );
    println!("Gaussian ML Estimator MSE: {}", mse_gaussian_ml_estimator);
}
