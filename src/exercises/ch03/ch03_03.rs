use crate::utils::utils::{
    element_wise_median_of_n_dimensional_samples, mean_of_d_dimensional_samples, mse_to_data,
    show_plotly_plots, weiszfeld_geometric_median,
};
use plotly::Plot;
use plotly::Scatter;
use plotly::Trace;
use plotly::color::NamedColor;
use plotly::common::Mode;
use rand::distributions::Distribution;
use rand::prelude::SmallRng;

pub fn exercise03_03() {
    use statrs::distribution::Normal;
    let true_mean = 0.1;
    let true_std = 1.0;
    let n = Normal::new(true_mean, true_std).unwrap();

    // Sample from the distribution using Distribution trait
    let mut rng: SmallRng = rand::SeedableRng::from_entropy();

    let d = 2;
    let number_of_iid_vars = 20;
    let number_of_experiments = 1000;
    fn generate_d_dimensional_n_normal_samples(
        n: &Normal,
        rng: &mut SmallRng,
        d: usize,
        number_of_iid_vars: usize,
    ) -> Vec<Vec<f64>> {
        (0..number_of_iid_vars)
            .map(|_| (0..d).map(|_| n.sample(rng)).collect::<Vec<f64>>())
            .collect::<Vec<Vec<f64>>>()
    }

    let data = generate_d_dimensional_n_normal_samples(&n, &mut rng, d, number_of_iid_vars);
    let mean = mean_of_d_dimensional_samples(&data);
    let median = element_wise_median_of_n_dimensional_samples(&data);
    let geometric_median = weiszfeld_geometric_median(&data, 100);
    let mut plot = Plot::new();
    let scatter = Scatter::new(
        data.iter().map(|point| point[0]).collect::<Vec<f64>>(),
        data.iter().map(|point| point[1]).collect::<Vec<f64>>(),
    )
    .mode(Mode::Markers) as Box<dyn Trace>;
    plot.add_trace(scatter);
    let mean_scatter = Scatter::new(vec![mean[0]], vec![mean[1]])
        .name("Mean")
        .marker(
            plotly::common::Marker::new()
                .color(NamedColor::Red)
                .size(12)
                .symbol(plotly::common::MarkerSymbol::Square),
        ) as Box<dyn Trace>;
    plot.add_trace(mean_scatter);
    let median_scatter = Scatter::new(vec![median[0]], vec![median[1]])
        .name("Median")
        .marker(
            plotly::common::Marker::new()
                .color(NamedColor::Green)
                .size(12)
                .symbol(plotly::common::MarkerSymbol::Diamond),
        ) as Box<dyn Trace>;
    plot.add_trace(median_scatter);
    let geometric_median_scatter =
        Scatter::new(vec![geometric_median[0]], vec![geometric_median[1]])
            .name("Geometric Median")
            .marker(
                plotly::common::Marker::new()
                    .color(NamedColor::Blue)
                    .size(12)
                    .symbol(plotly::common::MarkerSymbol::Cross),
            ) as Box<dyn Trace>;
    plot.add_trace(geometric_median_scatter);
    let true_mean_scatter = Scatter::new(vec![true_mean], vec![true_mean])
        .name("True Mean")
        .marker(
            plotly::common::Marker::new()
                .color(NamedColor::Black)
                .size(12)
                .symbol(plotly::common::MarkerSymbol::X),
        ) as Box<dyn Trace>;
    plot.add_trace(true_mean_scatter);

    let mut plots = vec![plot];

    let true_mean_point = vec![true_mean; d];

    let data = (10..=100)
        .step_by(10)
        .map(|number_of_iid_vars| {
            let data = (0..number_of_experiments)
                .map(|_| {
                    generate_d_dimensional_n_normal_samples(&n, &mut rng, d, number_of_iid_vars)
                })
                .collect::<Vec<Vec<Vec<f64>>>>();
            let means = data
                .iter()
                .map(|experiment_data| mean_of_d_dimensional_samples(experiment_data))
                .collect::<Vec<Vec<f64>>>();
            let medians = data
                .iter()
                .map(|experiment_data| {
                    element_wise_median_of_n_dimensional_samples(experiment_data)
                })
                .collect::<Vec<Vec<f64>>>();
            let geometric_medians = data
                .iter()
                .map(|experiment_data| weiszfeld_geometric_median(experiment_data, 100))
                .collect::<Vec<Vec<f64>>>();
            (
                number_of_iid_vars,
                mse_to_data(&means, &true_mean_point),
                mse_to_data(&medians, &true_mean_point),
                mse_to_data(&geometric_medians, &true_mean_point),
            )
        })
        .collect::<Vec<(usize, f64, f64, f64)>>();

    let mut plot = Plot::new();
    let mean_trace = Scatter::new(
        data.iter()
            .map(|(n, _, _, _)| *n as f64)
            .collect::<Vec<f64>>(),
        data.iter().map(|(_, mse, _, _)| *mse).collect::<Vec<f64>>(),
    )
    .mode(Mode::LinesMarkers)
    .name("Mean") as Box<dyn Trace>;
    plot.add_trace(mean_trace);
    let median_trace = Scatter::new(
        data.iter()
            .map(|(n, _, _, _)| *n as f64)
            .collect::<Vec<f64>>(),
        data.iter().map(|(_, _, mse, _)| *mse).collect::<Vec<f64>>(),
    )
    .mode(Mode::LinesMarkers)
    .name("Median") as Box<dyn Trace>;
    plot.add_trace(median_trace);
    let geometric_median_trace = Scatter::new(
        data.iter()
            .map(|(n, _, _, _)| *n as f64)
            .collect::<Vec<f64>>(),
        data.iter().map(|(_, _, _, mse)| *mse).collect::<Vec<f64>>(),
    )
    .mode(Mode::LinesMarkers)
    .name("Geometric Median") as Box<dyn Trace>;
    plot.add_trace(geometric_median_trace);

    plots.push(plot);
    show_plotly_plots(plots, None);
}
