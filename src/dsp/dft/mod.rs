//! Discrete Fourier Transform
use std::f64;

/// Turn an arbitrary vector into a Rectangular DFT result
pub fn vec_to_rectangular(signal: Vec<f64>) -> (Vec<f64>, Vec<f64>) {
    let signal_length = signal.len();
    let result_length = signal.len() / 2 + 1;
    let mut re_x = vec![0f64; result_length];
    let mut im_x = vec![0f64; result_length];

    for k in 0..result_length {
        for i in 0..signal_length {;
            re_x[k] += signal[i] * (f64::consts::PI * 2.0 * (k as f64) * (i as f64) / (signal_length as f64)).cos()
        }
    }

    for k in 0..result_length {
        for i in 0..signal_length {;
            im_x[k] += -1.0 * signal[i] * (f64::consts::PI * 2.0 * (k as f64) * (i as f64) / (signal_length as f64)).sin()
        }
    }

    (re_x, im_x)
}

/// Turn a Rectangular DFT result into a Polar DFT result
pub fn rectangular_to_polar(real: Vec<f64>, imaginary: Vec<f64>) -> (Vec<f64>, Vec<f64>) {
    assert!(real.len() == imaginary.len());
    let result_length = real.len();

    let mut mag_x = vec![0f64; result_length];
    let mut phase_x = vec![0f64; result_length];

    for k in 0..result_length {
        mag_x[k] = ((real[k]).powi(2) + (imaginary[k]).powi(2)).sqrt();

        if real[k] == 0.0 {
            if imaginary[k] < 0.0 {
                phase_x[k] = -1.0 * f64::consts::PI / 2.0
            }
            else {
                phase_x[k] = f64::consts::PI / 2.0
            }
        }
        else {
            phase_x[k] = (imaginary[k] / real[k]).atan()
        }
    }

    (mag_x, phase_x)
}

/// Turn an arbitrary vector into a Polar DFT result
pub fn vec_to_polar(signal: Vec<f64>) -> (Vec<f64>, Vec<f64>) {
    let (real, imaginary) = vec_to_rectangular(signal);
    rectangular_to_polar(real, imaginary)
}