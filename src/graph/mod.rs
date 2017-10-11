//! Graphing module
//!
//! Send samples here, get graph of points

use gnuplot::{Figure, PlotOption, HELIX};
use gnuplot::AxesCommon;
use chan::Receiver;
use SAMPLE_RATE;
use FREQUENCY;

/// "Run" the grapher
/// Probably want to run this in a separate thread and send samples over a channel.
pub fn run(recv_points: Receiver<f64>) {
    let mut xs = vec![];
    let mut ys = vec![];

    let period = SAMPLE_RATE / FREQUENCY;

    for i in 0..((period*10.0) as usize) {
        let sample = recv_points.recv().unwrap();
        xs.push(i as f64);
        ys.push(sample);
    }

    let mut fig = Figure::new();
    let plot_options = [
        PlotOption::PointSymbol('o'),
        PlotOption::LineWidth(2.0),
        PlotOption::Caption("Signal"),
        PlotOption::Color("black"),
    ];
    fig.axes2d().set_palette(HELIX).lines_points(&xs, &ys, &plot_options);
    fig.show();

    // TODO: Just close the channel instead of constantly recv-ing here
    loop {
        recv_points.recv();
    }
}
