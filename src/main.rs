// PortAudio:
extern crate portaudio;
use portaudio as pa;

// DSP module
// TODO: remove all these "use" statements in favor of explicitly calling them in main.rs
pub mod dsp;
use dsp::sine_generator::SineGenerator;
use dsp::negate_signal::NegateSignal;
use dsp::add_signals::AddSignals;
use dsp::evaluatable::Evaluatable;

// Constants:
const CHANNELS: i32 = 2;
const NUM_SECONDS: i32 = 5;
const SAMPLE_RATE: f64 = 44100.0;
const FRAMES_PER_BUFFER: u32 = 64;

fn main() {
    let sine_generator1 = SineGenerator::new(44100.0, 400.0, 0.3);
    let sine_generator2 = SineGenerator::new(44100.0, 400.0, 0.3);
    let negated_sine_2 = NegateSignal::new(sine_generator2);

    //let mut add_vec = vec![sine_generator1, sine_generator2];

    let add_vec: Vec<Box<Evaluatable>> = vec![Box::new(sine_generator1), Box::new(negated_sine_2)];

    let add_signals = AddSignals::new(add_vec);
    run(add_signals).unwrap()
}


fn run<E: Evaluatable + 'static>(mut generator: E) -> Result<(), pa::Error> {
    println!("PortAudio Test: output sine wave. SR = {}, BufSize = {}", SAMPLE_RATE, FRAMES_PER_BUFFER);

    let pa = try!(pa::PortAudio::new());

    let mut settings = try!(pa.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER));
    settings.params.suggested_latency = 0.1;

    // This routine will be called by the PortAudio engine when audio is needed. It may called at
    // interrupt level on some machines so don't do anything that could mess up the system like
    // dynamic resource allocation or IO.
    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        let mut idx = 0;
        for _ in 0..frames {
            let samples = generator.evaluate();
            buffer[idx]   = samples.0;
            buffer[idx+1] = samples.1;
            idx += 2;
        }
        pa::Continue
    };

    let mut stream = try!(pa.open_non_blocking_stream(settings, callback));

    try!(stream.start());

    println!("Play for {} seconds.", NUM_SECONDS);
    pa.sleep(NUM_SECONDS * 1_000);

    try!(stream.stop());
    try!(stream.close());

    println!("Test finished.");

    Ok(())
}