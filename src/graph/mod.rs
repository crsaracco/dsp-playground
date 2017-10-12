//! Graphing module
//!
//! Send samples here, get graph of points

use chan::Receiver;
use itertools_num::linspace;
use criterion_plot::prelude::*;
use std::path::Path;
use std::string::String;
use dsp;

// Constants:
use audio_playground::SAMPLE_RATE;

/// Grapher
pub struct Grapher {
}

/// "Run" the grapher
/// Probably want to run this in a separate thread and send samples over a channel.
pub fn run(recv_points: Receiver<f64>) {
    // Get 0.1 seconds' worth of samples, and then plot it:
    let points: Vec<f64> = recv_points.iter().take((SAMPLE_RATE * 0.1) as usize).collect();
    plot_vector(points.clone(), "audio", "audio.svg", false);

    let (magnitude, phase) = dsp::dft::vec_to_polar(points);
    plot_vector(magnitude, "magnitude", "magnitude.svg", true);

    // TODO: Just close the channel instead of constantly recv-ing here
    loop {
        recv_points.recv();
    }
}

/// Plot an arbitrary vector.
pub fn plot_vector(y_values: Vec<f64>, dataname: &'static str, filename: &'static str, log: bool) {
    let x_values = linspace::<f64>(0.0, y_values.len() as f64, y_values.len()).collect::<Vec<_>>();

    // Make a new Figure to plot our vector:
    let mut f = Figure::new();

    // Configure settings for the output of the plot:
    f.set(Font("Helvetica"));
    f.set(FontSize(16.0));
    f.set(Output(Path::new(filename)));
    f.set(Size(1336, 768));

    // If log, set y axis to log mode:
    if log {
        f.configure(Axis::LeftY, |a| a
            .set(Scale::Logarithmic)
            .set(Range::Limits(1e-2, 1e3))
        );
    }

    // Configure the key for the plot
    f.configure(Key, |k| {
        k.set(Boxed::Yes)
            .set(Position::Inside(Vertical::Top, Horizontal::Left))
    });

    // Plot the vector (in memory):
    f.plot(
        Lines {
            x: x_values,
            y: y_values,
        },
        |l| {
            l.set(Color::Rgb(255, 0, 0))
                .set(Label(dataname))
                .set(LineType::Solid)
        }
    );

    // Spit out the plot to a .svg file:
    f.draw()
        .ok()
        .and_then(|gnuplot| {
            gnuplot.wait_with_output()
                .ok()
                .and_then(|p| String::from_utf8(p.stderr).ok())
        }).expect("ERROR occurred while plotting");
}