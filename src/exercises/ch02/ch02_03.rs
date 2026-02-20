use plotlars::PlotHelper;

use crate::utils::utils::load_crypto_data;

pub fn exercise02_03() {
    // Choose one asset and plot the price time series using both
    // a linear and a logarithmic scale. Compare the plots and comment.
    let data_set = load_crypto_data();

    use polars::prelude::*;
    let plot_data = data_set
        .select([
            col("Date").cast(DataType::Datetime(TimeUnit::Nanoseconds, None)),
            col("BTC"),
        ])
        .collect()
        .unwrap();
    // create a returns column
    let plot_data = plot_data
        .lazy()
        .with_column((col("BTC") / col("BTC").shift(1.into()) - lit(1)).alias("BTC_returns"))
        .select([col("Date"), col("BTC_returns")])
        .filter(col("BTC_returns").is_not_null())
        .collect()
        .unwrap();

    // Choose one asset and compute the volatility (square root of the average
    // of the squared returns over k samples) on a rolling-window basis in two ways:
    let k: usize = 30;
    let mut plot_data = plot_data
        .lazy()
        .with_column(
            col("BTC_returns")
                .pow(lit(2))
                .rolling_mean(RollingOptionsFixedWindow {
                    window_size: k,
                    min_periods: 1,
                    center: false,
                    ..Default::default()
                })
                .sqrt()
                .alias("volatility_not_centered"),
        )
        .with_column(
            col("BTC_returns")
                .pow(lit(2))
                .rolling_mean(RollingOptionsFixedWindow {
                    window_size: k,
                    min_periods: 1,
                    center: true,
                    ..Default::default()
                })
                .sqrt()
                .alias("volatility_centered"),
        )
        .collect()
        .unwrap();

    use plotlars::{Plot, SubplotGrid, TimeSeriesPlot};
    // centered or not draws a translated picture

    let k_s = vec![3, 9, 30, 90];
    let centered_col_strings: Vec<String> = k_s
        .iter()
        .map(|k| format!("volatility_centered_k{}", k))
        .collect();
    let uncentered_col_strings: Vec<String> = k_s
        .iter()
        .map(|k| format!("volatility_not_centered_k{}", k))
        .collect();
    for i in 0..k_s.len() {
        let k = k_s[i];
        plot_data = plot_data
            .lazy()
            .with_column(
                col("BTC_returns")
                    .pow(lit(2))
                    .rolling_mean(RollingOptionsFixedWindow {
                        window_size: k,
                        min_periods: 1,
                        center: true,
                        ..Default::default()
                    })
                    .sqrt()
                    .alias(&centered_col_strings[i]),
            )
            .with_column(
                col("BTC_returns")
                    .pow(lit(2))
                    .rolling_mean(RollingOptionsFixedWindow {
                        window_size: k,
                        min_periods: 1,
                        center: false,
                        ..Default::default()
                    })
                    .sqrt()
                    .alias(&uncentered_col_strings[i]),
            )
            .collect()
            .unwrap();
    }
    let mut strings = Vec::new();
    for i in 0..k_s.len() {
        strings.push(&centered_col_strings[i]);
        strings.push(&uncentered_col_strings[i]);
    }
    let plots: Vec<Box<dyn PlotHelper>> = strings
        .iter()
        .map(|&s| {
            Box::new(
                TimeSeriesPlot::builder()
                    .data(&plot_data)
                    .x("Date")
                    .y(s)
                    .plot_title(s)
                    .build(),
            ) as Box<dyn PlotHelper>
        })
        .collect();

    SubplotGrid::regular()
        .rows(strings.len())
        .cols(1)
        .plots(plots.iter().map(|p| p.as_ref()).collect())
        .h_gap(0.1)
        .build()
        .plot();

    // As the window size increases, the volatility estimate becomes smoother
    // and less sensitive to short-term fluctuations in returns, and lowers
}
