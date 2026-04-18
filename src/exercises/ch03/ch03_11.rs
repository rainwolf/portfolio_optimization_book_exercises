use crate::utils::utils::{
    frobenius_norm_squared, generate_d_dimensional_samples, mse_to_matrix_data, show_plot_traces,
    vec_to_matrix,
};
use faer::prelude::Mat;
use plotly::Trace;
use statrs::distribution::Normal;

pub fn exercise03_11() {
    // Consider an N-dimensional i.i.d. time series with zero mean and identity covariance matrix
    //     Generate Gaussian data for N=10 and T=20 and
    // estimate the covariance matrix with the sample mean and with the shrinkage Ledoit–Wolf  estimator.

    let d = 10;
    let t = 20;
    let n = Normal::new(0.0, 1.0).unwrap();
    let data_gaussian = generate_d_dimensional_samples(&n, d, t);

    fn ledoit_wolf_covariance_estimator(data: &[Vec<f64>]) -> Mat<f64> {
        let d = data[0].len() as f64;
        let t = data.len() as f64;

        let data_mats = data
            .iter()
            .map(|point| vec_to_matrix(point))
            .collect::<Vec<Mat<f64>>>();
        let mean_col = data_mats
            .iter()
            .fold(Mat::zeros(d as usize, 1), |acc, mat| acc + mat)
            / t;
        let cov_mat = data_mats
            .iter()
            .map(|col| {
                let centered_col = &(col - &mean_col);
                centered_col * centered_col.transpose()
            })
            .fold(Mat::zeros(d as usize, d as usize), |acc, mat| acc + mat)
            / (t - 1.0);
        let trace = cov_mat.diagonal().column_vector().sum();
        let target_cov: Mat<f64> = Mat::identity(d as usize, d as usize) as Mat<f64> * (trace / d);
        let rho: f64 = data_mats
            .iter()
            .map(|x| frobenius_norm_squared(&(&cov_mat - x * x.transpose())))
            .sum::<f64>()
            / (t * frobenius_norm_squared(&(&cov_mat - &target_cov)));
        let rho = rho.min(1.0);
        target_cov * rho + cov_mat * (1.0 - rho)
    }
    println!(
        "LW estimator: {:?}",
        ledoit_wolf_covariance_estimator(&data_gaussian)
    );

    let number_of_experiments = 1000;
    let true_cov = Mat::identity(d, d) as Mat<f64>;
    let data = (0..number_of_experiments)
        .map(|_| {
            let data_gaussian = generate_d_dimensional_samples(&n, d, t);
            ledoit_wolf_covariance_estimator(&data_gaussian)
        })
        .collect::<Vec<Mat<f64>>>();
    let mse = mse_to_matrix_data(&data, &true_cov);
    println!("MSE of LW estimator: {mse}");

    let mse_data = (10..100)
        .step_by(10)
        .map(|t| {
            let data = (0..number_of_experiments)
                .map(|_| {
                    let data_gaussian = generate_d_dimensional_samples(&n, d, t);
                    ledoit_wolf_covariance_estimator(&data_gaussian)
                })
                .collect::<Vec<Mat<f64>>>();
            mse_to_matrix_data(&data, &true_cov)
        })
        .collect::<Vec<f64>>();
    let plot = plotly::Scatter::new((10..100).step_by(10).collect::<Vec<usize>>(), mse_data)
        .mode(plotly::common::Mode::LinesMarkers)
        .name("MSE of LW estimator") as Box<dyn Trace>;
    let plots = vec![plot];
    show_plot_traces(plots, "MSE of LW estimator".into());
}
