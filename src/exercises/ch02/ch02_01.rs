use crate::utils::utils;

pub fn exercise02_01() {
    // Choose one asset and plot the price time series using both
    // a linear and a logarithmic scale. Compare the plots and comment.
    let data_set = utils::load_crypto_data();

    use polars::prelude::*;
    let plot_data = data_set
        .select([
            col("Date").cast(DataType::Datetime(TimeUnit::Nanoseconds, None)),
            col("BTC"),
        ])
        .collect()
        .unwrap()
        .into();

    use plotlars::{Axis, AxisType, Plot, TimeSeriesPlot};
    TimeSeriesPlot::builder()
        .data(&plot_data)
        .x("Date")
        .y("BTC")
        .y_axis(&Axis::new().axis_type(AxisType::Log))
        .build()
        .plot();
    TimeSeriesPlot::builder()
        .data(&plot_data)
        .x("Date")
        .y("BTC")
        .y_axis(&Axis::new().axis_type(AxisType::Linear))
        .build()
        .plot();

    // small differences are more pronounced in log scale
}
