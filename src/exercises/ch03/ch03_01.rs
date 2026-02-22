use plotly::Histogram;
use plotly::Scatter;
use plotly::plot;
use polars::prelude::*;
use rand::distributions::Distribution;
use rand::rngs::SmallRng;

use crate::utils::utils::show_plot;

pub fn exercise03_01() {
    use statrs::distribution::Normal;
    let true_mean = 0.1;
    let true_std = 1.0;
    let n = Normal::new(true_mean, true_std).unwrap();

    // Sample from the distribution using Distribution trait
    let mut rng: SmallRng = rand::SeedableRng::from_entropy();
    // let sample = n.sample(&mut rng);
    let bins = 21;
    let number_of_experiments = 1000;

    let number_of_iid_vars = 10;
    let plot_data = (0..number_of_experiments)
        .map(|_| {
            Series::from_iter((0..number_of_iid_vars).map(|_| n.sample(&mut rng)))
                .mean()
                .unwrap()
        })
        .collect::<Vec<f64>>();
    let plot = Histogram::new(plot_data.clone()).n_bins_x(bins) as Box<dyn plotly::Trace>;
    let mut plots = vec![plot];
    let mut plot_stds = Vec::new();
    let plot_data_mean = Series::from_iter(plot_data.iter()).mean().unwrap();
    let plot_data_std = Series::from_iter(plot_data.iter()).std(0).unwrap();
    let theoretical_std = true_std / (number_of_iid_vars as f64).sqrt();
    println!(
        "For {number_of_iid_vars} iid variables, the theoretical mean is {true_mean} and the theoretical standard deviation is {theoretical_std}"
    );
    println!("Sample mean: {plot_data_mean}, Sample standard deviation: {plot_data_std}");

    let number_of_iid_vars = 20;
    let plot_data = (0..number_of_experiments)
        .map(|_| {
            Series::from_iter((0..number_of_iid_vars).map(|_| n.sample(&mut rng)))
                .mean()
                .unwrap()
        })
        .collect::<Vec<f64>>();
    let plot = Histogram::new(plot_data.clone()).n_bins_x(bins) as Box<dyn plotly::Trace>;
    plots.push(plot);
    plot_stds.push(Series::from_iter(plot_data.iter()).std(0).unwrap());
    let plot_data_mean = Series::from_iter(plot_data.iter()).mean().unwrap();
    let plot_data_std = Series::from_iter(plot_data.iter()).std(0).unwrap();
    let theoretical_std = true_std / (number_of_iid_vars as f64).sqrt();
    println!(
        "For {number_of_iid_vars} iid variables, the theoretical mean is {true_mean} and the theoretical standard deviation is {theoretical_std}"
    );
    println!("Sample mean: {plot_data_mean}, Sample standard deviation: {plot_data_std}");

    let number_of_iid_vars = (10..=100).step_by(10);
    let mse = number_of_iid_vars
        .clone()
        .map(|iid_vars_number| {
            let means = (0..number_of_experiments)
                .map(|_| {
                    Series::from_iter((0..iid_vars_number).map(|_| n.sample(&mut rng)))
                        .mean()
                        .unwrap()
                })
                .map(|mean| (mean - true_mean).powi(2));
            Series::from_iter(means).mean().unwrap()
        })
        .collect::<Vec<f64>>();
    let plot = Scatter::new(
        number_of_iid_vars.clone().map(|x| x as f64).collect(),
        mse.clone(),
    ) as Box<dyn plotly::Trace>;
    plots.push(plot);
    let plot = Scatter::new(
        number_of_iid_vars.clone().map(|x| x as f64).collect(),
        number_of_iid_vars
            .clone()
            .map(|x| true_std / (x as f64))
            .collect(),
    ) as Box<dyn plotly::Trace>;
    plots.push(plot);
    show_plot(plots, None);
}
