use crate::utils::utils::{mse_to_matrix_data, show_plot_traces_in_one_plot};
use nalgebra::DVector;
use plotly::Trace;
use polars::polars_utils::itertools::Itertools;
use rand::distr::Distribution;
use rand::prelude::ThreadRng;
use statrs::distribution::MultivariateNormal;

pub fn exercise03_12() {
    fn generate_t_multivariable_samples(
        t: usize,
        n: usize,
        distribution: &impl Distribution<DVector<f64>>,
    ) -> Mat<f64> {
        let mut rng: ThreadRng = rand::rng();
        let samples = (0..t)
            .map(|_| distribution.sample(&mut rng))
            .collect::<Vec<DVector<f64>>>();
        Mat::from_fn(t, n, |i, j| samples[i][j])
    }
    let dimension = 10;
    let number_of_samples = 20;
    let number_of_experiments = 100;
    let true_mean = vec![0.1; dimension];
    let true_cov: Mat<f64> = Mat::identity(dimension, dimension);
    let true_cov_vec: Vec<f64> = true_cov
        .col_iter()
        .map(|col| col.iter().map(|&x| x).collect_vec())
        .flatten()
        .collect::<Vec<f64>>();
    let distribution = MultivariateNormal::new(true_mean, true_cov_vec).unwrap();
    let data = generate_t_multivariable_samples(number_of_samples, dimension, &distribution);

    use faer::{Col, Mat, Row};

    /// X: T×N (rows = observations, cols = assets)
    fn single_factor_cov(x: &Mat<f64>) -> Mat<f64> {
        let (t, n) = (x.nrows(), x.ncols());

        // Center columns
        let mut xc = x.clone();
        let mean = x.row_iter().fold(Row::<f64>::zeros(n), |acc, r| acc + r) / t as f64;
        xc.row_iter_mut().for_each(|mut r| r -= &mean);

        // SVD: Xc = U S V^T
        let svd = xc.thin_svd().expect("SVD failed");
        let s = svd.S().column_vector();
        let v = svd.V();

        // Eigenvalues of the sample covariance: lambda_k = s_k^2 / (T-1)
        let k = s.nrows(); // min(T, N)
        let lambda: Vec<f64> = (0..k).map(|i| s[i] * s[i] / (t - 1) as f64).collect();

        // Noise = mean of the trailing eigenvalues. If T < N, the missing ones are 0.
        let variance_noise = lambda[1..].iter().sum::<f64>() / (n - 1) as f64;
        let variance_pc1 = (lambda[0] - variance_noise).max(0.0);

        // beta = v1 * sqrt(lambda1 - sigma_eps^2)
        let scale = variance_pc1.sqrt();
        let beta: Col<f64> = Col::from_fn(n, |i| v[(i, 0)] * scale);

        // Sigma = beta beta^T + sigma_eps^2 I
        let mut sigma: Mat<f64> = &beta * beta.transpose();
        for i in 0..n {
            sigma[(i, i)] += variance_noise;
        }
        sigma
    }

    fn sample_cov(data: &Mat<f64>) -> Mat<f64> {
        let (t, n) = (data.nrows(), data.ncols());
        let mean = data.row_iter().fold(Row::<f64>::zeros(n), |acc, r| acc + r) / t as f64;
        let mut centered_data = data.clone();
        centered_data.row_iter_mut().for_each(|mut r| r -= &mean);
        let cov = &centered_data.transpose() * &centered_data / (t - 1) as f64;
        cov
    }

    let sigma = single_factor_cov(&data);
    println!("{:?}", sigma);

    let mean_squared_error = mse_to_matrix_data(&vec![sigma], &true_cov);
    println!("Mean Squared Error: {:?}", mean_squared_error);

    let mean_squared_errors_sfe = (10..=100)
        .step_by(10)
        .map(|t| {
            let datas = (0..number_of_experiments)
                .map(|_| {
                    let data = generate_t_multivariable_samples(t, dimension, &distribution);
                    single_factor_cov(&data)
                })
                .collect::<Vec<Mat<f64>>>();
            mse_to_matrix_data(&datas, &true_cov)
        })
        .collect::<Vec<f64>>();

    let mean_squared_error_sample_cov = (10..=100)
        .step_by(10)
        .map(|t| {
            let datas = (0..number_of_experiments)
                .map(|_| {
                    let data = generate_t_multivariable_samples(t, dimension, &distribution);
                    sample_cov(&data)
                })
                .collect::<Vec<Mat<f64>>>();
            mse_to_matrix_data(&datas, &true_cov)
        })
        .collect::<Vec<f64>>();

    let plot_sfe = plotly::Scatter::new(
        (10..=100).step_by(10).collect::<Vec<usize>>(),
        mean_squared_errors_sfe,
    )
    .mode(plotly::common::Mode::LinesMarkers)
    .name("MSE of Factor Model Estimator") as Box<dyn Trace>;
    let plot_sample_cov = plotly::Scatter::new(
        (10..=100).step_by(10).collect::<Vec<usize>>(),
        mean_squared_error_sample_cov,
    )
    .mode(plotly::common::Mode::LinesMarkers)
    .name("MSE of Sample Covariance Estimator") as Box<dyn Trace>;
    let plots = vec![plot_sfe, plot_sample_cov];
    show_plot_traces_in_one_plot(plots, "MSE of Factor Model Estimator".into());
}
