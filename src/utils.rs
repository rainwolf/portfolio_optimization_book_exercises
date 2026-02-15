use build_html::*;
use plotly::Trace;
use polars::prelude::*;
use std::env;
use std::{fs::File, io::Write, process::Command};

pub fn load_crypto_data() -> LazyFrame {
    let data_set = LazyFrame::scan_parquet(
        PlPath::new("./cryptos_2017to2021_daily.parquet"),
        Default::default(),
    )
        .unwrap();
    // let print_set = data_set.clone();
    // println!("{:?}", print_set.first().collect().unwrap());
    data_set
}

pub fn load_stocks_data() -> LazyFrame {
    let data_set = LazyFrame::scan_parquet(
        PlPath::new("./SP500_stocks_2015to2020.parquet"),
        Default::default(),
    )
        .unwrap();
    // let print_set = data_set.clone();
    // println!("{:?}", print_set.first().collect().unwrap());
    data_set
}

pub fn show_plot(traces: Vec<Box<dyn Trace>>) {
    let mut base = HtmlPage::new()
        .with_title("Plotly-rs Multiple Plots")
        .with_script_link("https://cdn.plot.ly/plotly-latest.min.js")
        .with_header(1, "Multiple Plotly plots on the same HTML page");
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

    let mut temp = env::temp_dir();
    temp.push("plotly.html");

    // Save the rendered plot to the temp file.
    let temp_path = temp.to_str().unwrap();
    {
        let mut file = File::create(temp_path).unwrap();
        file.write_all(html.as_bytes())
            .expect("failed to write html output");
        file.flush().unwrap();
    }
    Command::new("open")
        .args([temp_path])
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
