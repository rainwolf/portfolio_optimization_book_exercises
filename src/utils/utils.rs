use build_html::*;
use faer::Mat;
use plotly::{Plot, Trace};
use polars::prelude::*;
use rand::distributions::Distribution;
use rand::prelude::SmallRng;
use statrs::statistics::Statistics;
use std::{io::Write, process::Command};
use tempfile::NamedTempFile;

pub fn load_crypto_data() -> LazyFrame {
    let data_set = LazyFrame::scan_parquet(
        PlRefPath::new("./cryptos_2017to2021_daily.parquet"),
        Default::default(),
    )
    .unwrap();
    // let print_set = data_set.clone();
    // println!("{:?}", print_set.first().collect().unwrap());
    data_set
}

pub fn load_stocks_data() -> LazyFrame {
    let data_set = LazyFrame::scan_parquet(
        PlRefPath::new("./SP500_stocks_2015to2020.parquet"),
        Default::default(),
    )
    .unwrap();
    // let print_set = data_set.clone();
    // println!("{:?}", print_set.first().collect().unwrap());
    data_set
}

pub fn load_index_data() -> LazyFrame {
    let data_set = LazyFrame::scan_parquet(
        PlRefPath::new("./SP500_index_2015to2020.parquet"),
        Default::default(),
    )
    .unwrap();
    let print_set = data_set.clone();
    println!("{:?}", print_set.first().collect().unwrap());
    data_set
}

pub fn show_plot_traces(traces: Vec<Box<dyn Trace>>, title: Option<&str>) {
    let mut base = HtmlPage::new()
        .with_title(title.unwrap_or("Plotly-rs Multiple Plots"))
        .with_script_link("https://cdn.plot.ly/plotly-latest.min.js")
        .with_header(
            1,
            title.unwrap_or("Multiple Plotly plots on the same HTML page"),
        );
    use plotly::Plot;
    for (i, trace) in traces.iter().enumerate() {
        let mut plot = Plot::new();
        plot.add_trace(trace.clone());
        base.add_raw(
            plot.to_inline_html(Some(format!("test_{}", i).as_str()))
                .as_str(),
        );
    }
    let html = base.to_html_string();

    let (mut file, path) = NamedTempFile::with_suffix(".html").unwrap().keep().unwrap();
    // Save the rendered plot to the temp file.
    file.write_all(html.as_bytes())
        .expect("failed to write html output");
    file.flush().unwrap();
    Command::new("open")
        .args([path.to_str().unwrap()])
        .output()
        .expect("DEFAULT_HTML_APP_NOT_FOUND");
}

pub fn show_plotly_plots(plots: Vec<Plot>, title: Option<&str>) {
    let mut base = HtmlPage::new()
        .with_title(title.unwrap_or("Plotly-rs Multiple Plots"))
        .with_script_link("https://cdn.plot.ly/plotly-latest.min.js")
        .with_header(
            1,
            title.unwrap_or("Multiple Plotly plots on the same HTML page"),
        );
    for (i, plot) in plots.iter().enumerate() {
        base.add_raw(
            plot.to_inline_html(Some(format!("test_{}", i).as_str()))
                .as_str(),
        );
    }

    let (mut file, path) = NamedTempFile::with_suffix(".html").unwrap().keep().unwrap();
    // Save the rendered plot to the temp file.
    file.write_all(base.to_html_string().as_bytes())
        .expect("failed to write html output");
    file.flush().unwrap();
    Command::new("open")
        .args([path.to_str().unwrap()])
        .output()
        .expect("DEFAULT_HTML_APP_NOT_FOUND");
}

use polars::prelude::cov::pearson_corr;
pub fn auto_correlation(data: &DataFrame, column: &str, lag: i32) -> Option<f64> {
    let col1 = data
        .column(column)
        .unwrap()
        .cast(&DataType::Float64)
        .unwrap();
    let col2 = col1.shift(lag.into());
    pearson_corr(col1.f64().unwrap(), col2.f64().unwrap())
}

pub fn cross_correlation(data: &DataFrame, column1: &str, column2: &str) -> f64 {
    let col1 = data
        .column(column1)
        .unwrap()
        .cast(&DataType::Float64)
        .unwrap();
    let col2 = data
        .column(column2)
        .unwrap()
        .cast(&DataType::Float64)
        .unwrap();
    pearson_corr(col1.f64().unwrap(), col2.f64().unwrap()).unwrap()
}

pub fn mean_of_d_dimensional_samples(samples: &[Vec<f64>]) -> Vec<f64> {
    let d = samples[0].len();
    (0..d)
        .map(|i| {
            Series::from_iter(samples.iter().map(|sample| sample[i]))
                .mean()
                .unwrap()
        })
        .collect::<Vec<f64>>()
}

pub fn element_wise_median_of_n_dimensional_samples(samples: &[Vec<f64>]) -> Vec<f64> {
    let d = samples[0].len();
    (0..d)
        .map(|i| {
            Series::from_iter(samples.iter().map(|sample| sample[i]))
                .median()
                .unwrap()
        })
        .collect::<Vec<f64>>()
}

pub fn weiszfeld_geometric_median(points: &[Vec<f64>], max_iterations: usize) -> Vec<f64> {
    let d = points[0].len();
    let points_as_series: Vec<Series> = points
        .iter()
        .map(|point| Series::from_iter(point.iter()))
        .collect();
    // mean as initial value for iteration
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
        let numerator: Series = points_as_series
            .iter()
            .zip(&distances_to_median)
            .map(|(x, y)| x / *y)
            .fold(Series::from_iter(vec![0.0; d]), |acc, x| (acc + x).unwrap());
        let denominator = distances_to_median
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

pub fn mse_to_data(data: &[Vec<f64>], estimator: &[f64]) -> f64 {
    let estimator_series = Series::from_iter(estimator.iter());
    data.iter()
        .map(|point| {
            let point_series = Series::from_iter(point.iter());
            let difference = (&point_series - &estimator_series).unwrap();
            (&difference * &difference)
                .unwrap()
                .f64()
                .unwrap()
                .iter()
                .flatten()
                .sum::<f64>()
        })
        .mean()
}

pub fn generate_d_dimensional_samples<T>(
    distribution: &T,
    dimension: usize,
    number_of_iid_vars: usize,
) -> Vec<Vec<f64>>
where
    T: Distribution<f64>,
{
    let mut rng: SmallRng = rand::SeedableRng::from_entropy();
    (0..number_of_iid_vars)
        .map(|_| {
            (0..dimension)
                .map(|_| distribution.sample(&mut rng))
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>()
}

pub fn vec_to_series(vec: &Vec<f64>) -> Series {
    Series::from_iter(vec.iter())
}

pub fn series_to_vec(series: &Series) -> Vec<f64> {
    series.f64().unwrap().into_iter().flatten().collect()
}

pub fn frobenius_norm_squared(mat: &Mat<f64>) -> f64 {
    mat.norm_l2().powi(2)
}

pub fn vec_to_matrix(vec: &[f64]) -> Mat<f64> {
    Mat::from_fn(vec.len(), 1, |i, _| vec[i])
}
