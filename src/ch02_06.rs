use super::utils::{cross_correlation, load_index_data, load_stocks_data, show_plot};
use plotly::{HeatMap, Trace};
use polars::prelude::*;
use rand::seq::{IndexedRandom, index};

pub fn exercise02_06() {
    let data_set = load_stocks_data();
    // print!("{:?}", data_set.clone().first().collect().unwrap());
    // print!("{:?}", data_set.clone().collect().unwrap().schema());
    let col_names = data_set
        .clone()
        .collect()
        .unwrap()
        .schema()
        .iter()
        .map(|(p, _)| p.to_string())
        .filter(|s| s != "Date")
        .collect::<Vec<String>>();
    let number_of_cols = 10;
    let mut rng = &mut rand::rng();
    let random_cols = col_names
        .sample(&mut rng, number_of_cols)
        .into_iter()
        .map(|s| s.clone())
        .collect::<Vec<String>>();
    print!("{:?}", random_cols.clone());
    let data_cols = random_cols
        .iter()
        .map(|s| col(s.to_string()))
        .collect::<Vec<Expr>>();

    let mut corr_data_set = data_set.clone().select(data_cols).collect().unwrap();
    let mut correlations = vec![vec![1.0; number_of_cols]; number_of_cols];
    for i in 0..number_of_cols {
        for j in i + 1..number_of_cols {
            let col_i = &random_cols[i];
            let col_j = &random_cols[j];
            let corr_value = cross_correlation(&corr_data_set, col_i, col_j);
            correlations[i][j] = corr_value;
            correlations[j][i] = corr_value;
        }
    }
    let plot =
        HeatMap::new(random_cols.clone(), random_cols.clone(), correlations) as Box<dyn Trace>;
    let mut plots = vec![plot];
    // show_plot(plots);

    let index_data_set = load_index_data();
    let index_corr_data_set = data_set
        .join(
            index_data_set,
            [col("Date")],
            [col("Date")],
            JoinArgs::new(JoinType::Inner),
        )
        .collect()
        .unwrap();
    let mut index_correlations: Vec<f64> = Vec::new();
    for i in 0..number_of_cols {
        let col_i = &random_cols[i];
        let corr_value = cross_correlation(&index_corr_data_set, col_i, "SP500_INDEX");
        index_correlations.push(corr_value);
    }
    let plot = HeatMap::new(
        random_cols.clone(),
        vec!["SP500_INDEX".to_string()],
        vec![index_correlations],
    ) as Box<dyn Trace>;
    plots.push(plot);
    show_plot(plots);
}
