#![deny(missing_docs)]

//! Main driver application for the "dsp" library.

// PortAudio:
extern crate portaudio;
use portaudio as pa;

// DSP module
// TODO: remove all these "use" statements in favor of explicitly calling them in main.rs
pub mod dsp;
use dsp::generators;
use dsp::negate_signal::NegateSignal;
use dsp::add_signals::AddSignals;
use dsp::evaluatable::Evaluatable;

// Constants:
const NUM_CHANNELS: i32 = 2;
const NUM_SECONDS: i32 = 5;
const SAMPLE_RATE: f64 = 44100.0;
const FRAMES_PER_BUFFER: u32 = 64;
const BUFFER_SECONDS: f64 = 0.100;  // Buffer samples for 100ms -- reduces chances of underrun


fn main() {
    // Create input sound generators:
    let saw_generator1 = generators::Saw::new(44100.0, 100.0*3.0, 0.1);
    let saw_generator2 = generators::Saw::new(44100.0, 120.0*3.0, 0.1);
    let saw_generator3 = generators::Saw::new(44100.0, 150.0*3.0, 0.1);

    let add_vec: Vec<Box<Evaluatable>> = vec![Box::new(saw_generator1), Box::new(saw_generator2), Box::new(saw_generator3)];
    let add_signals = AddSignals::new(add_vec);

    // Play our resulting audio:
    run(add_signals).unwrap()
}


fn run<E: Evaluatable + 'static>(mut generator: E) -> Result<(), pa::Error> {
    // Fire up ye olde PortAudio:
    let pa = try!(pa::PortAudio::new());

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
            let samples = generator.evaluate();
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
    // Let's sleep for NUM_SECONDS to let PortAudio play sounds for a bit.
    // Note that pa.sleep() will sleep for *at least* NUM_SECONDS, but probably more -- don't depend
    // on pa.sleep() for musical timings or anything like that.
    println!("Play for {} seconds.", NUM_SECONDS);
    pa.sleep(NUM_SECONDS * 1_000);

    // We're done playing, gracefully shut down the stream:
    try!(stream.stop());
    try!(stream.close());

    Ok(())
}