use crate::utils::utils::load_stocks_data;

pub fn exercise04_03() {
    //   Choose one asset and try the EWMA model
    // on the log-returns and log-prices for different values of the
    // memory α. Compute the mean squared error of the forecast.
    let data = load_stocks_data();
    let unwrapped_data = data.clone().collect().unwrap();
    let colname = unwrapped_data.get_column_names()
        [rand::random_range(1..unwrapped_data.get_column_names().len())]
    .clone()
    .to_string();
    let daily_logprice_colname = format!("{}_daily_logprice", colname);
    let daily_logprice_moving_average_colname =
        format!("{}_daily_logprice_moving_average", colname);
    let daily_logreturn_colname = format!("{}_daily_logreturn", colname);
    let daily_logreturn_moving_average_colname =
        format!("{}_daily_logreturn_moving_average", colname);

    // Log-prices y_t = ln(p_t); log-returns x_t = y_t - y_{t-1}
    let data = data
        .with_column(
            polars::prelude::col(&colname)
                .log(polars::prelude::lit(std::f64::consts::E))
                .alias(&daily_logprice_colname),
        )
        // Daily
        .with_column(
            (polars::prelude::col(&daily_logprice_colname)
                - polars::prelude::col(&daily_logprice_colname).shift(1.into()))
            .alias(&daily_logreturn_colname),
        )
        // Weekly
        .select([
            polars::prelude::col(&colname),
            polars::prelude::col(&daily_logreturn_colname),
            polars::prelude::col(&daily_logprice_colname),
        ])
        // Weighted rolling_mean panics on nulls: drop the leading null log-return
        // (from shift) and any missing prices.
        .drop_nulls(None);

    fn compute_moving_average_with_memory_mean_squared_error(
        data: &polars::prelude::LazyFrame,
        memory: f64,
        daily_logprice_colname: String,
        daily_logreturn_colname: String,
        daily_logprice_moving_average_colname: String,
        daily_logreturn_moving_average_colname: String,
    ) -> (f64, f64) {
        let window_size = 20;
        let weights = (0..window_size)
            .map(|i| memory * (1.0 - memory).powi(i as i32))
            .rev()
            .collect::<Vec<f64>>();
        let moving_averages = data
            .clone()
            .with_column(
                polars::prelude::col(&daily_logprice_colname)
                    .rolling_mean(polars::prelude::RollingOptionsFixedWindow {
                        window_size,
                        min_periods: 1,
                        center: false,
                        weights: Some(weights.clone()),
                        ..Default::default()
                    })
                    .alias(&daily_logprice_moving_average_colname),
            )
            .with_column(
                polars::prelude::col(&daily_logreturn_colname)
                    .rolling_mean(polars::prelude::RollingOptionsFixedWindow {
                        window_size,
                        min_periods: 1,
                        center: false,
                        weights: Some(weights.clone()),
                        ..Default::default()
                    })
                    .alias(&daily_logreturn_moving_average_colname),
            );

        let logprice_squared_error_col = "predicted_price_squared_error";
        let logreturn_squared_error_col = "predicted_return_squared_error";
        let squared_errors = moving_averages
            .with_column(
                (polars::prelude::col(&daily_logprice_colname)
                    - polars::prelude::col(&daily_logprice_moving_average_colname).shift(1.into()))
                .pow(polars::prelude::lit(2))
                .alias(logprice_squared_error_col),
            )
            .with_column(
                (polars::prelude::col(&daily_logprice_colname)
                    - (polars::prelude::col(&daily_logprice_colname).shift(1.into())
                        + polars::prelude::col(daily_logreturn_moving_average_colname)
                            .shift(1.into())))
                .pow(polars::prelude::lit(2))
                .alias(logreturn_squared_error_col),
            )
            .select([
                polars::prelude::col(logprice_squared_error_col),
                polars::prelude::col(logreturn_squared_error_col),
            ]);

        let logprice_mean_squared_error = squared_errors
            .clone()
            .select([polars::prelude::col(logprice_squared_error_col).mean()])
            .collect()
            .unwrap()
            .column(logprice_squared_error_col)
            .unwrap()
            .f64()
            .unwrap()
            .get(0)
            .unwrap();

        let logreturn_mean_squared_error = squared_errors
            .select([polars::prelude::col(logreturn_squared_error_col).mean()])
            .collect()
            .unwrap()
            .column(logreturn_squared_error_col)
            .unwrap()
            .f64()
            .unwrap()
            .get(0)
            .unwrap();

        (logprice_mean_squared_error, logreturn_mean_squared_error)
    }

    let memory_values: Vec<f64> = vec![0.1, 0.3, 0.5, 0.7, 0.9];
    let mse = memory_values
        .clone()
        .iter()
        .map(|&memory| {
            compute_moving_average_with_memory_mean_squared_error(
                &data,
                memory,
                daily_logprice_colname.clone(),
                daily_logreturn_colname.clone(),
                daily_logprice_moving_average_colname.clone(),
                daily_logreturn_moving_average_colname.clone(),
            )
        })
        .collect::<Vec<(f64, f64)>>();
    // create a dataframe with a windows column and window_sizes as values
    let dataframe = polars::prelude::DataFrame::new(
        memory_values.len(),
        vec![
            polars::prelude::Column::new(
                "memory".into(),
                polars::prelude::Series::from_iter(memory_values.iter()),
            ),
            polars::prelude::Column::new(
                "logprice_mse".into(),
                polars::prelude::Series::from_iter(
                    mse.iter().map(|(logprice_mse, _)| *logprice_mse),
                ),
            ),
            polars::prelude::Column::new(
                "logreturn_mse".into(),
                polars::prelude::Series::from_iter(
                    mse.iter().map(|(_, logreturn_mse)| *logreturn_mse),
                ),
            ),
        ],
    )
    .unwrap();

    print!("{:?}", dataframe);
}
