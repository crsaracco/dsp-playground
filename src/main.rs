#![deny(missing_docs)]

//! Main driver application for the "dsp" library.

// PortAudio:
extern crate portaudio;
use portaudio as pa;

// Channels for buffering up samples in between threads:
extern crate chan;

// other `use` statements:
use std::thread;
use std::time;

// DSP module
// TODO: remove all these "use" statements in favor of explicitly calling them in main.rs
pub mod dsp;
use dsp::generators;
use dsp::negate_signal::NegateSignal;
use dsp::add_signals::AddSignals;
use dsp::evaluatable::Evaluatable;


// Constants:
const NUM_CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 44100.0;
const FRAMES_PER_BUFFER: u32 = 1024;
const BUFFER_SECONDS: f64 = 0.100;  // Buffer samples for 100ms -- reduces chances of underrun
const FREQUENCY: f64 = 440.0;

fn main() {
    // Collect all our threads:
    let mut children = vec![];

    // Signal flow:
    // (signal generators + filters + combinations) --> (portaudio) --> (grapher - first 1000 samples)

    // Create a channel for (signals) --> (portaudio)
    let (send_audio, recv_audio) = chan::sync(SAMPLE_RATE as usize);

    // Create a channel for (portaudio) --> (grapher)
    let (send_points, recv_points) = chan::sync(SAMPLE_RATE as usize);

    // TODO: maybe split this out into different functions / modules

    // signal generators:
    children.push(thread::spawn(move || {
        // A-Minor chord
        let sine_generator1 = generators::Sine::new(SAMPLE_RATE, FREQUENCY * 1.0, 0.1);
        let sine_generator2 = generators::Sine::new(SAMPLE_RATE, FREQUENCY * 1.2 * 0.5, 0.1);
        let sine_generator3 = generators::Sine::new(SAMPLE_RATE, FREQUENCY * 1.5 * 0.5, 0.1);
        let add_vec: Vec<Box<Evaluatable>> = vec![Box::new(sine_generator1), Box::new(sine_generator2), Box::new(sine_generator3)];

        let mut add_signals = AddSignals::new(add_vec);
        loop {
            send_audio.send(add_signals.evaluate());
        }
    }));

    // PortAudio:
    children.push(thread::spawn(move || {
        thread::sleep(time::Duration::new(0, 1_000_000));
        run(recv_audio, send_points).unwrap()
    }));

    // Grapher:
    children.push(thread::spawn(move || {
        let mut x = vec![];
        let mut y = vec![];

        let period = SAMPLE_RATE / FREQUENCY;

        for i in 0..((period*50.0) as usize) {
            let sample = recv_points.recv().unwrap();
            x.push(i as f32);
            y.push(sample.0);
        }

        // TODO: all we need is a plotting library to plot here...

        // TODO: Just close the channel instead of constantly recv-ing here
        loop {
            recv_points.recv();
        }
    }));

    // Wait for all the child threads to finish:
    for child in children {
        let _ = child.join();
    }
}


fn run(recv_audio: chan::Receiver<(f32, f32)>, send_points: chan::Sender<(f32, f32)>) -> Result<(), pa::Error> {
    // Fire up ye olde PortAudio:
    println!("=============");
    let pa = try!(pa::PortAudio::new());
    println!("=============");

    // Set up our settings - set a buffer amount to try to reduce underruns:
    let mut settings = try!(pa.default_output_stream_settings(NUM_CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER));
    settings.params.suggested_latency = BUFFER_SECONDS;

    // This callback function will be called by PortAudio when it needs more audio samples.
    // It may be called at interrupt level on some machines, so don't do anything that could mess
    // up the system like dynamic resource allocation or I/O. (although doing so seems to be fine on
    // my machine...?)
    //
    // The job of this callback is to fill up the buffer that PortAudio tells us to fill up.
    // Each "frame" represents one sample for each channel that we have, so we need to put a total
    // of (NUM_CHANNELS * frames) samples into the buffer.
    // The samples are "interleaved" by default, so the structure of buffer looks like:
    // [ch0_sample0, ch1_sample0, ch0_sample1, ch1_sample1, ch0_sample2, ch1_sample2, ...]
    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        let mut i = 0;
        for _ in 0..frames {
            let samples = recv_audio.recv().unwrap();
            send_points.send(samples.clone());
            buffer[i]   = samples.0;
            buffer[i+1] = samples.1;
            i += 2;
        }
        pa::Continue
    };

    // Now that we have the settings and the callback function set up, we can finally open the
    // stream, through which we will actually play audio:
    let mut stream = try!(pa.open_non_blocking_stream(settings, callback));

    // And now that we have the stream, we can start playing sounds!
    try!(stream.start());

    // We're using PortAudio in non-blocking mode, so execution will fall through immedately.
    // Loop indefinitely to let audio keep playing.
    loop {
        thread::sleep(time::Duration::new(1, 0));
    }

    // We're done playing, gracefully shut down the stream:
    try!(stream.stop());
    try!(stream.close());

    Ok(())
}