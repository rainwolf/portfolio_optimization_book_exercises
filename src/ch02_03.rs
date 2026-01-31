use plotlars::PlotHelper;

use super::utils::load_data;

pub fn exercise02_03() {
    // Choose one asset and plot the price time series using both
    // a linear and a logarithmic scale. Compare the plots and comment.
    let data_set = load_data();

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
    // TimeSeriesPlot::builder()
    //     .data(&plot_data)
    //     .x("Date")
    //     .y("volatility_not_centered")
    //     .build()
    //     .plot();
    // TimeSeriesPlot::builder()
    //     .data(&plot_data)
    //     .x("Date")
    //     .y("volatility_centered")
    //     .build()
    //     .plot();

    // centered or not draws a translated picture

    let k_s = vec![3, 9, 30, 90];
    let col_strings: Vec<String> = k_s
        .iter()
        .map(|k| format!("volatility_centered_k{}", k))
        .collect();
    for i in 0..k_s.len() {
        let k = k_s[i];
        let col_string = &col_strings[i];
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
                    .alias(col_string),
            )
            .collect()
            .unwrap();
    }
    let plots = col_strings
        .iter()
        .map(|s| {
            TimeSeriesPlot::builder()
                .data(&plot_data)
                .x("Date")
                .y(s)
                .build()
        })
        .collect::<Vec<TimeSeriesPlot>>();

    SubplotGrid::regular()
        .rows(k_s.len())
        .cols(1)
        .plots(vec![&plots[0], &plots[1], &plots[2], &plots[3]])
        .build()
        .plot();

    // As the window size increases, the volatility estimate becomes smoother
    // and less sensitive to short-term fluctuations in returns, and lowers
}
