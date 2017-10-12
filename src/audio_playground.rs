//! Audio Playground module.
//!
//! A driver application for the DSP module where I play sounds and graph graphs.

use chan;
use std::thread;

// Modules defined within this project:
use dsp;
use audio;
use graph;

// Traits:
use chan::Sender;
use dsp::traits::Signal;

/// Audio playback samplerate, in Hz
pub const SAMPLE_RATE: f64 = 44100.0;

/// Main driver function for the "audio playground"
pub fn audio_playground() {
    // The general signal flow for our program is currently:
    // (audio processing) --> (audio playing) --> (grapher - first X samples)
    //
    // Each component runs in its own thread, and the arrows represent channels that the threads
    // send data over.

    // Create the channels for communication:
    let (send_audio, recv_audio) = chan::sync(SAMPLE_RATE as usize);
    let (send_graph_points, recv_graph_points) = chan::sync(SAMPLE_RATE as usize);

    // Collect all our threads so we can .join() later:
    let mut children = vec![];

    // Create the audio processing thread:
    children.push(thread::spawn(move || {
        generate_audio(send_audio);
    }));

    // Create the audio playing thread:
    children.push(thread::spawn(move || {
        audio::run(recv_audio, send_graph_points).unwrap()
    }));

    // Create the grapher thread:
    children.push(thread::spawn(move || {
        graph::run(recv_graph_points);
    }));

    // Wait for all the child threads to finish:
    for child in children {
        let _ = child.join();
    }
}

fn generate_audio(send_audio: Sender<f64>) {
    let mut some_generator = dsp::generators::Triangle::new(0.1, 440.0, 0.0);

    loop {
        send_audio.send(some_generator.evaluate());
    }
}