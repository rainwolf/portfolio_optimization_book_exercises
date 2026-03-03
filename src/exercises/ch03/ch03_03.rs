use crate::utils::utils::show_plot;
use plotly::Plot;
use plotly::Scatter;
use plotly::Trace;
use plotly::color::NamedColor;
use plotly::common::Mode;
use polars::prelude::Series;
use polars::prelude::*;
use rand::distributions::Distribution;
use rand::prelude::SmallRng;

pub fn exercise03_03() {
    use statrs::distribution::Normal;
    let true_mean = 0.1;
    let true_std = 1.0;
    let n = Normal::new(true_mean, true_std).unwrap();

    // Sample from the distribution using Distribution trait
    let mut rng: SmallRng = rand::SeedableRng::from_entropy();

    let d = 2;
    let number_of_iid_vars = 20;
    let number_of_experiments = 1000;
    fn generate_d_dimensional_n_normal_samples(
        n: &Normal,
        rng: &mut SmallRng,
        d: usize,
        number_of_iid_vars: usize,
    ) -> Vec<Vec<f64>> {
        (0..number_of_iid_vars)
            .map(|_| (0..d).map(|_| n.sample(rng)).collect::<Vec<f64>>())
            .collect::<Vec<Vec<f64>>>()
    }
    fn mean_of_d_dimensional_samples(samples: &Vec<Vec<f64>>) -> Vec<f64> {
        let d = samples[0].len();
        (0..d)
            .map(|i| {
                Series::from_iter(samples.iter().map(|sample| sample[i]))
                    .mean()
                    .unwrap()
            })
            .collect::<Vec<f64>>()
    }

    fn element_wise_median_of_n_dimensional_samples(samples: &Vec<Vec<f64>>) -> Vec<f64> {
        let d = samples[0].len();
        (0..d)
            .map(|i| {
                Series::from_iter(samples.iter().map(|sample| sample[i]))
                    .median()
                    .unwrap()
            })
            .collect::<Vec<f64>>()
    }

    fn weiszfeld_geometric_median(points: &Vec<Vec<f64>>, max_iterations: usize) -> Vec<f64> {
        let d = points[0].len();
        // mean as initial value for iteration
        let points_as_series: Vec<Series> = points
            .iter()
            .map(|point| Series::from_iter(point.iter()))
            .collect();
        let mut median: Series = points_as_series
            .iter()
            .fold(Series::from_iter(vec![0.0; d]), |acc, x| {
                (&acc + x).unwrap()
            })
            / (points.len() as f64);

        for _ in 0..max_iterations {
            let distances_to_median: Vec<f64> = points_as_series
                .iter()
                .map(|point| {
                    let difference = (point - &median).unwrap();
                    (&difference * &difference)
                        .unwrap()
                        .f64()
                        .unwrap()
                        .iter()
                        .flatten()
                        .sum::<f64>()
                        .sqrt()
                })
                .collect();
            let mut numerator: Series = points_as_series
                .iter()
                .zip(&distances_to_median)
                .map(|(x, y)| x / *y)
                .fold(Series::from_iter(vec![0.0; d]), |acc, x| (acc + x).unwrap());
            let mut denominator = distances_to_median
                .iter()
                .map(|&distance| 1.0 / distance)
                .sum::<f64>();
            let new_median = &numerator / denominator;
            let difference_to_previous_median = &(&median - &new_median).unwrap();
            let distance_to_previous_median = (difference_to_previous_median
                * difference_to_previous_median)
                .unwrap()
                .sum::<f64>()
                .unwrap()
                .sqrt();
            if distance_to_previous_median > 1e-10 {
                median = new_median;
            } else {
                break;
            }
        }
        median
            .f64()
            .unwrap()
            .into_iter()
            .flatten()
            .collect::<Vec<f64>>()
    }

    let data = generate_d_dimensional_n_normal_samples(&n, &mut rng, d, number_of_iid_vars);
    let mean = mean_of_d_dimensional_samples(&data);
    let median = element_wise_median_of_n_dimensional_samples(&data);
    let geometric_median = weiszfeld_geometric_median(&data, 100);
    let mut plot = Plot::new();
    let scatter = Scatter::new(
        data.iter().map(|point| point[0]).collect::<Vec<f64>>(),
        data.iter().map(|point| point[1]).collect::<Vec<f64>>(),
    )
    .mode(Mode::Markers) as Box<dyn Trace>;
    plot.add_trace(scatter);
    let mean_scatter = Scatter::new(vec![mean[0]], vec![mean[1]])
        .name("Mean")
        .marker(
            plotly::common::Marker::new()
                .color(NamedColor::Red)
                .size(12)
                .symbol(plotly::common::MarkerSymbol::Square),
        ) as Box<dyn Trace>;
    plot.add_trace(mean_scatter);
    let median_scatter = Scatter::new(vec![median[0]], vec![median[1]])
        .name("Median")
        .marker(
            plotly::common::Marker::new()
                .color(NamedColor::Green)
                .size(12)
                .symbol(plotly::common::MarkerSymbol::Diamond),
        ) as Box<dyn Trace>;
    plot.add_trace(median_scatter);
    let geometric_median_scatter =
        Scatter::new(vec![geometric_median[0]], vec![geometric_median[1]])
            .name("Geometric Median")
            .marker(
                plotly::common::Marker::new()
                    .color(NamedColor::Blue)
                    .size(12)
                    .symbol(plotly::common::MarkerSymbol::Cross),
            ) as Box<dyn Trace>;
    plot.add_trace(geometric_median_scatter);
    let true_mean_scatter = Scatter::new(vec![true_mean], vec![true_mean])
        .name("True Mean")
        .marker(
            plotly::common::Marker::new()
                .color(NamedColor::Black)
                .size(12)
                .symbol(plotly::common::MarkerSymbol::X),
        ) as Box<dyn Trace>;
    plot.add_trace(true_mean_scatter);
    plot.show();

    fn mse_to_data(data: &Vec<Vec<f64>>, estimator: &Vec<f64>) -> Vec<f64> {
        data.iter()
            .map(|point| {
                let difference = point
                    .iter()
                    .zip(estimator.iter())
                    .map(|(x, e)| (x - e).powi(2))
                    .sum::<f64>();
                difference.sqrt()
            })
            .collect()
    }
    println!("\x1B[2J"); // clear the terminal
    println!();
}
