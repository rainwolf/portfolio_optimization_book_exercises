use crate::utils::utils::{mse_to_matrix_data, show_plot_traces, vec_to_matrix};
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

    let sigma = single_factor_cov(&data);

    println!("{:?}", sigma);

    let mean_squared_error = mse_to_matrix_data(&vec![sigma], &true_cov);
    println!("Mean Squared Error: {:?}", mean_squared_error);

    let mean_squared_errors = (10..=100)
        .step_by(10)
        .map(|t| {
            let data = generate_t_multivariable_samples(t, dimension, &distribution);
            let sigma = single_factor_cov(&data);
            mse_to_matrix_data(&vec![sigma], &true_cov)
        })
        .collect::<Vec<f64>>();

    let plot = plotly::Scatter::new(
        (10..=100).step_by(10).collect::<Vec<usize>>(),
        mean_squared_errors,
    )
    .mode(plotly::common::Mode::LinesMarkers)
    .name("MSE of Factor Model Estimator") as Box<dyn Trace>;
    let plots = vec![plot];
    show_plot_traces(plots, "MSE of Factor Model Estimator".into());
}
