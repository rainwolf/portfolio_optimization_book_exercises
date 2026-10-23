use crate::utils::utils::{load_stocks_data, show_plot_traces};
use plotly::{Scatter, Trace};

pub fn exercise04_01() {
    // (Autocorrelation function of returns) Choose one asset and plot the autocorrelation
    // function of the log-returns at different frequencies.
    let data = load_stocks_data();
    let unwrapped_data = data.clone().collect().unwrap();
    let colname = unwrapped_data.get_column_names()
        [rand::random_range(1..unwrapped_data.get_column_names().len())]
    .clone()
    .to_string();
    let return_colname = format!("{}_return", colname);
    let logreturn_colname = format!("{}_logreturn", colname);

    let data = data
        // .lazy()
        .with_column(
            (polars::prelude::col(&colname) / polars::prelude::col(&colname).shift(1.into())
                - polars::prelude::lit(1))
            .alias(&return_colname),
        )
        .with_column(
            (polars::prelude::col(&return_colname) + polars::prelude::lit(1))
                .log(polars::prelude::lit(2))
                .alias(&logreturn_colname),
        )
        .select([
            polars::prelude::col(&colname),
            polars::prelude::col(&return_colname),
            polars::prelude::col(&logreturn_colname),
        ])
        .filter(polars::prelude::col(&logreturn_colname).is_not_null())
        .collect()
        .unwrap();

    let lag_range = 10;
    fn lags(n: i32) -> std::ops::RangeInclusive<i32> {
        0..=n
    }
    let auto_corrs = lags(lag_range)
        .map(|lag| {
            crate::utils::utils::auto_correlation(&data, &logreturn_colname, lag).unwrap_or(0.0)
        })
        .collect::<Vec<f64>>();

    let auto_corr_plot = Scatter::new(lags(lag_range).map(|x| x as f64).collect(), auto_corrs)
        .mode(plotly::common::Mode::Lines) as Box<dyn Trace>;

    show_plot_traces(
        vec![auto_corr_plot],
        Some(format!("Autocorrelation of Log-Returns for {}", colname).as_str()),
    );

    println!("{:?}", data);
}
