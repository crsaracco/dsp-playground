#![deny(missing_docs)]

//! Main driver application for the "dsp" library.

// Extern crates:
extern crate chan;            // Channels for buffering up samples in between threads
extern crate portaudio;       // PortAudio for playing audio
extern crate itertools_num;   // Useful vector maker for plotting
extern crate criterion_plot;  // Criterion-plot takes samples and makes a .svg file

// Public modules:
pub mod dsp;               // dsp-related functions
pub mod audio;             // audio playback
pub mod graph;             // graphing
pub mod audio_playground;  // combining the above three into one "program"

fn main() {
    audio_playground::audio_playground();
}