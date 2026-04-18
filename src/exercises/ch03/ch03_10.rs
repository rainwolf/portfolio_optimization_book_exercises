use crate::utils::utils::{
    element_wise_median_of_n_dimensional_samples, generate_d_dimensional_samples, mse_to_data,
    series_to_vec, show_plot_traces, vec_to_series,
};
use faer::Side;

use faer::prelude::{Col, Mat, Solve};
use plotly::Trace;
use polars::prelude::Series;
use statrs::distribution::Normal;

pub fn exercise03_10() {
    // Consider an N-dimensional i.i.d. time series with zero mean and identity covariance matrix
    //     Generate Gaussian data for N=10 and T=20 and
    // estimate the mean vector with the sample mean and with the shrinkage James–Stein estimator.

    let d = 10;
    let t = 20;
    let n = Normal::new(0.0, 1.0).unwrap();
    let data_gaussian = generate_d_dimensional_samples(&n, d, t);

    let unit_matrix: Mat<f64> = Mat::from_fn(d, d, |i, j| if i == j { 1.0 } else { 0.0 });

    fn james_stein_mean_estimator(data: &[Vec<f64>], cov: &Mat<f64>) -> Vec<f64> {
        let d = data[0].len();
        let t = data.len();
        let zero = Series::from_iter(vec![0.0; d].iter());
        let sample_mean: Series = data.iter().fold(zero, |acc, point| {
            (acc + vec_to_series(point) / t as f64).unwrap()
        });
        let target_mean: Series =
            vec_to_series(&element_wise_median_of_n_dimensional_samples(&data));
        let b = (&sample_mean - &target_mean).unwrap();
        let b_col: Col<f64> = Col::from_iter(series_to_vec(&b).into_iter());
        // should have been done outside if we need to estimate multiple times
        let cov_inv_b = cov.llt(Side::Lower).unwrap().solve(&b_col);
        let rho = (d + 2) as f64 / (d as f64 + 2.0 + t as f64 * (b_col.transpose() * &cov_inv_b));
        let rho = rho.min(1.0);
        let james_stein_estimator = sample_mean * rho + target_mean * (1.0 - rho);
        series_to_vec(&james_stein_estimator.unwrap())
    }
    println!(
        "JS estimator: {:?}",
        james_stein_mean_estimator(&data_gaussian, &unit_matrix)
    );

    let number_of_experiments = 1000;
    let true_mean = vec![0.0; d];
    let data = (0..number_of_experiments)
        .map(|_| {
            let data_gaussian = generate_d_dimensional_samples(&n, d, t);
            james_stein_mean_estimator(&data_gaussian, &unit_matrix)
        })
        .collect::<Vec<Vec<f64>>>();
    let mse = mse_to_data(&data, &true_mean);
    println!("MSE of JS estimator: {mse}");
    let mse_data = (10..100)
        .step_by(10)
        .map(|t| {
            let data = (0..number_of_experiments)
                .map(|_| {
                    let data_gaussian = generate_d_dimensional_samples(&n, d, t);
                    james_stein_mean_estimator(&data_gaussian, &unit_matrix)
                })
                .collect::<Vec<Vec<f64>>>();
            mse_to_data(&data, &true_mean)
        })
        .collect::<Vec<f64>>();
    let plot = plotly::Scatter::new((10..100).step_by(10).collect::<Vec<usize>>(), mse_data)
        .mode(plotly::common::Mode::LinesMarkers)
        .name("MSE of JS estimator") as Box<dyn Trace>;
    let plots = vec![plot];
    show_plot_traces(plots, "MSE of JS estimator".into());
}
