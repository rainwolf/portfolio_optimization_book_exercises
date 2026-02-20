use plotly::Trace;

use crate::utils::utils::{load_crypto_data, show_plot};

pub fn exercise02_04() {
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
    // create a log returns column
    let plot_data = plot_data
        .lazy()
        .with_column(
            (col("BTC_returns") + lit(1))
                .log(lit(2))
                .alias("BTC_log_returns"),
        )
        .select([col("Date"), col("BTC_log_returns")])
        .filter(col("BTC_log_returns").is_not_null())
        .collect()
        .unwrap();

    // Plot histograms of the log-returns at different frequencies. Compare the plots and comment.
    use plotly::{Histogram, Scatter};
    let bins = vec![5, 10, 20, 50, 100];
    let mut plots = bins
        .iter()
        .map(|&b| {
            Histogram::new(
                plot_data
                    .column("BTC_log_returns")
                    .unwrap()
                    .f64()
                    .unwrap()
                    .to_vec(),
            )
            .n_bins_x(b) as Box<dyn Trace>
        })
        .collect::<Vec<Box<dyn Trace>>>();
    // show_plot(plots);
    // bins becomes smaller -> histogram becomes smoother

    // Draw Q–Q plots to focus on the tail distribution. Do the returns follow a Gaussian distribution?
    use statrs::distribution::ContinuousCDF;
    use statrs::distribution::{Normal, Uniform};
    let n = plot_data.height();
    let qq_plot_data = plot_data
        .clone()
        .lazy()
        .select([col("BTC_log_returns")
            .sort(SortOptions::default().with_order_descending(false))
            .alias("sorted_BTC_log_returns")])
        .with_row_index("index", Some(1))
        .with_column((col("index") / lit(n as f64 + 1.0)).alias("quantiles"))
        .with_column(
            col("index")
                .cast(DataType::Float64)
                .apply(
                    |series| {
                        let normal = Normal::new(0.0, 1.0).unwrap();
                        let result = series.f64().unwrap().iter().map(|v| match v {
                            Some(x) => normal.inverse_cdf((x - 0.5) / series.len() as f64),
                            None => 0.0,
                        });
                        Ok(Series::from_iter(result).into())
                    },
                    |_schema, field| Ok(field.clone()),
                )
                .alias("normal_quantiles"),
        )
        .with_column(
            col("index")
                .cast(DataType::Float64)
                .apply(
                    |series| {
                        let uniform = Uniform::new(0.0, 1.0).unwrap();
                        let result = series.f64().unwrap().iter().map(|v| match v {
                            Some(x) => uniform.inverse_cdf((x - 0.5) / series.len() as f64),
                            None => 0.0,
                        });
                        Ok(Series::from_iter(result).into())
                    },
                    |_schema, field| Ok(field.clone()),
                )
                .alias("uniform_quantiles"),
        )
        .collect()
        .unwrap();
    // ScatterPlot::builder()
    //     .data(&qq_plot_data)
    //     .x("normal_quantiles")
    //     .y("quantiles")
    //     .build().plot();
    let normal_qq = Scatter::new(
        qq_plot_data
            .column("normal_quantiles")
            .unwrap()
            .f64()
            .unwrap()
            .to_vec(),
        qq_plot_data
            .column("quantiles")
            .unwrap()
            .f64()
            .unwrap()
            .to_vec(),
    )
    .mode(plotly::common::Mode::Markers)
    .name("Q-Q Plot") as Box<dyn Trace>;
    let uniform_qq = Scatter::new(
        qq_plot_data
            .column("uniform_quantiles")
            .unwrap()
            .f64()
            .unwrap()
            .to_vec(),
        qq_plot_data
            .column("quantiles")
            .unwrap()
            .f64()
            .unwrap()
            .to_vec(),
    )
    .mode(plotly::common::Mode::Markers)
    .name("Q-Q Plot") as Box<dyn Trace>;
    plots.push(normal_qq);
    plots.push(uniform_qq);
    show_plot(plots, Some("Q-Q Plots of BTC Log Returns"));
    // println!("{:?}", qq_plot_data);
    // log returns seem uniformly distributed, not Gaussian

    let kurtosis_skewness_data = plot_data
        .column("BTC_log_returns")
        .unwrap()
        .f64()
        .unwrap()
        .clone()
        .into_series();
    let kurtosis = kurtosis_skewness_data
        .kurtosis(false, false)
        .unwrap()
        .unwrap();
    let skewness = kurtosis_skewness_data.skew(false).unwrap().unwrap();
    println!("\n\nKurtosis of BTC log returns: {}\n\n", kurtosis);
    println!("\n\nSkewness of BTC log returns: {}\n\n", skewness);

    // kurtosis and skewness indicate that the distribution of log returns has heavier tails
    // and is more skewed than a normal distribution,
    // which is consistent with the observation from the Q-Q plot.
}
