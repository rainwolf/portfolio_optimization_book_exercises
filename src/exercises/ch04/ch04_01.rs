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

    // Compute simple returns at all frequencies, then log returns from each
    let data = data
        // Daily
        .with_column(
            (polars::prelude::col(&colname) / polars::prelude::col(&colname).shift(1.into())
                - polars::prelude::lit(1))
            .alias(format!("{}_daily_return", colname)),
        )
        .with_column(
            (polars::prelude::col(format!("{}_daily_return", colname)) + polars::prelude::lit(1))
                .log(polars::prelude::lit(2))
                .alias(format!("{}_daily_logreturn", colname)),
        )
        // Weekly
        .with_column(
            (polars::prelude::col(&colname) / polars::prelude::col(&colname).shift(5.into())
                - polars::prelude::lit(1))
            .alias(format!("{}_weekly_return", colname)),
        )
        .with_column(
            (polars::prelude::col(format!("{}_weekly_return", colname)) + polars::prelude::lit(1))
                .log(polars::prelude::lit(2))
                .alias(format!("{}_weekly_logreturn", colname)),
        )
        // Monthly
        .with_column(
            (polars::prelude::col(&colname) / polars::prelude::col(&colname).shift(21.into())
                - polars::prelude::lit(1))
            .alias(format!("{}_monthly_return", colname)),
        )
        .with_column(
            (polars::prelude::col(format!("{}_monthly_return", colname)) + polars::prelude::lit(1))
                .log(polars::prelude::lit(2))
                .alias(format!("{}_monthly_logreturn", colname)),
        )
        // Quarterly
        .with_column(
            (polars::prelude::col(&colname) / polars::prelude::col(&colname).shift(63.into())
                - polars::prelude::lit(1))
            .alias(format!("{}_quarterly_return", colname)),
        )
        .with_column(
            (polars::prelude::col(format!("{}_quarterly_return", colname)) + polars::prelude::lit(1))
                .log(polars::prelude::lit(2))
                .alias(format!("{}_quarterly_logreturn", colname)),
        )
        .select([
            polars::prelude::col(&colname),
            polars::prelude::col(format!("{}_daily_return", colname)),
            polars::prelude::col(format!("{}_daily_logreturn", colname)),
            polars::prelude::col(format!("{}_weekly_return", colname)),
            polars::prelude::col(format!("{}_weekly_logreturn", colname)),
            polars::prelude::col(format!("{}_monthly_return", colname)),
            polars::prelude::col(format!("{}_monthly_logreturn", colname)),
            polars::prelude::col(format!("{}_quarterly_return", colname)),
            polars::prelude::col(format!("{}_quarterly_logreturn", colname)),
        ])
        .filter(polars::prelude::col(format!("{}_quarterly_logreturn", colname)).is_not_null())
        .collect()
        .unwrap();

    let lag_ranges: &[(&str, i32)] = &[
        ("daily", 30),
        ("weekly", 30),
        ("monthly", 30),
        ("quarterly", 30),
    ];

    let mut traces: Vec<Box<dyn Trace>> = Vec::new();

    for (label, lag_range) in lag_ranges {
        let freq_col = format!("{}_{}_logreturn", colname, label);
        let auto_corrs: Vec<f64> = (0..=*lag_range)
            .map(|lag| {
                crate::utils::utils::auto_correlation(&data, &freq_col, lag).unwrap_or(0.0)
            })
            .collect();

        let auto_corr_plot = Scatter::new(
            (0..=*lag_range).map(|x| x as f64).collect(),
            auto_corrs,
        )
            .name(label)
            .mode(plotly::common::Mode::Lines) as Box<dyn Trace>;

        traces.push(auto_corr_plot);
    }

    let title = format!("Autocorrelation of Log-Returns for {}", colname);
    let plot_title = Some(title.as_str());
    show_plot_traces(traces.clone(), plot_title);
    crate::utils::utils::show_plot_traces_in_one_plot(traces, plot_title);

    println!("{:?}", data);
}
