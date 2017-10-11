#![deny(missing_docs)]

//! Main driver application for the "dsp" library.

// Channels for buffering up samples in between threads:
extern crate chan;

// other `use` statements:
use std::thread;
use std::time;

// Audio module
extern crate portaudio;
pub mod audio;

// Graphing module
extern crate gnuplot;
pub mod graph;

// DSP module
pub mod dsp;
use dsp::generators;

// Traits:
use dsp::traits::Signal;

// Constants:
const SAMPLE_RATE: f64 = 44100.0;
const FREQUENCY: f64 = 440.0;

fn main() {
    // Collect all our threads:
    let mut children = vec![];

    // Signal flow:
    // (signal generators + filters + combinations) --> (portaudio) --> (grapher - first x samples)

    // Create a channel for (signals) --> (portaudio)
    let (send_audio, recv_audio) = chan::sync(SAMPLE_RATE as usize);

    // Create a channel for (portaudio) --> (grapher)
    let (send_points, recv_points) = chan::sync(SAMPLE_RATE as usize);
    
    // signal generators:
    children.push(thread::spawn(move || {
        let mut some_generator = generators::Triangle::new(0.8, FREQUENCY, 0.0);

        loop {
            send_audio.send(some_generator.evaluate());
        }
    }));

    // PortAudio:
    children.push(thread::spawn(move || {
        thread::sleep(time::Duration::new(0, 100_000));
        audio::run(recv_audio, send_points).unwrap()
    }));

    // Grapher:
    children.push(thread::spawn(move || {
        graph::run(recv_points);
    }));

    // Wait for all the child threads to finish:
    for child in children {
        let _ = child.join();
    }
}