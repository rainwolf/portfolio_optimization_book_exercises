use polars::prelude::*;
use build_html::*;
use std::env;
use std::{fs::File, io::Write, process::Command};
use plotly::Trace;

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

pub fn show_plot(traces: Vec<Box<dyn Trace>>) {
    let mut base = HtmlPage::new()
        .with_title("Plotly-rs Multiple Plots")
        .with_script_link("https://cdn.plot.ly/plotly-latest.min.js")
        .with_header(1, "Multiple Plotly plots on the same HTML page");
    use plotly::Plot;
    for (i, trace) in traces.iter().enumerate() {
        let mut plot = Plot::new();
        plot.add_trace(trace.clone());
        base.add_raw(plot.to_inline_html(Some(format!("test_{}", i).as_str())).as_str());
    }
    let html = base.to_html_string();

    let mut temp = env::temp_dir();
    temp.push("plotly.html");

    // Save the rendered plot to the temp file.
    let temp_path = temp.to_str().unwrap();
    {
        let mut file = File::create(temp_path).unwrap();
        file.write_all(html.as_bytes())
            .expect("failed to write html output");
        file.flush().unwrap();
    }
    Command::new("open")
        .args([temp_path])
        .output()
        .expect("DEFAULT_HTML_APP_NOT_FOUND");
}
