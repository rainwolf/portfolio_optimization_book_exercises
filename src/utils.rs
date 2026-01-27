use polars::prelude::*;

pub fn load_data() -> LazyFrame {
    let data_set = LazyFrame::scan_parquet(
        PlPath::new("./cryptos_2017to2021_daily.parquet"),
        Default::default(),
    )
    .unwrap();
    // let print_set = data_set.clone();
    // println!("{:?}", print_set.first().collect().unwrap());
    data_set
}
