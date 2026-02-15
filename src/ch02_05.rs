use super::utils::{auto_correlation, load_crypto_data, show_plot};
use plotly::{Scatter, Trace};
use polars::prelude::cov::pearson_corr;
use polars::prelude::*;

pub fn exercise02_05() {
    let data_set = load_crypto_data();

    use polars::prelude::*;
    let plot_data = data_set
        .select([
            col("Date").cast(DataType::Datetime(TimeUnit::Nanoseconds, None)),
            col("BTC"),
        ])
        .collect()
        .unwrap();
    let mut cols = plot_data
        .get_column_names()
        .iter()
        .map(|&s| col(s.to_string()))
        .collect::<Vec<Expr>>();
    cols.push(col("BTC_returns"));
    // create a returns column
    let plot_data = plot_data
        .lazy()
        .with_column((col("BTC") / col("BTC").shift(1.into()) - lit(1)).alias("BTC_returns"))
        .select(&cols)
        .filter(col("BTC_returns").is_not_null())
        .collect()
        .unwrap();
    // create a log returns column
    cols.push(col("BTC_log_returns"));
    let plot_data = plot_data
        .lazy()
        .with_column(
            (col("BTC_returns") + lit(1))
                .log(lit(2))
                .alias("BTC_log_returns"),
        )
        .select(&cols)
        .filter(col("BTC_log_returns").is_not_null())
        .collect()
        .unwrap();
    let lags = 1..=30;
    let auto_corrs: Vec<f64> = lags
        .clone()
        .map(|lag| auto_correlation(&plot_data, "BTC_log_returns", lag).unwrap_or(0.0))
        .collect();
    let auto_corr_plot = Scatter::new(lags.clone().map(|x| x as f64).collect(), auto_corrs)
        .mode(plotly::common::Mode::Lines) as Box<dyn Trace>;
    let mut plots = vec![auto_corr_plot];
    // print!("{:?}", plot_data);

    cols.push(col("BTC_returns_squared"));
    let plot_data = plot_data
        .lazy()
        .with_column((col("BTC_returns").pow(2)).alias("BTC_returns_squared"))
        .select(&cols)
        .filter(col("BTC_returns_squared").is_not_null())
        .collect()
        .unwrap();
    let auto_corrs: Vec<f64> = lags
        .clone()
        .map(|lag| auto_correlation(&plot_data, "BTC_returns_squared", lag).unwrap_or(0.0))
        .collect();
    let auto_corr_plot = Scatter::new(lags.clone().map(|x| x as f64).collect(), auto_corrs)
        .mode(plotly::common::Mode::Lines) as Box<dyn Trace>;
    plots.push(auto_corr_plot);
    show_plot(plots);
}
