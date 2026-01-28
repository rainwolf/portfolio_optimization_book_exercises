use super::utils::load_data;

pub fn exercise02_02() {
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

    use plotlars::{Axis, AxisType, Plot, TimeSeriesPlot};
    TimeSeriesPlot::builder()
        .data(&plot_data)
        .x("Date")
        .y("BTC_returns")
        .y_axis(&Axis::new().axis_type(AxisType::Log))
        .build()
        .plot();
    TimeSeriesPlot::builder()
        .data(&plot_data)
        .x("Date")
        .y("BTC_returns")
        .y_axis(&Axis::new().axis_type(AxisType::Linear))
        .build()
        .plot();

    // returns are much more erratic in log scale
}
