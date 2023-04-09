extern crate ndarray;
extern crate ndarray_npy;
extern crate ndarray_rand;

use ndarray::prelude::*;

pub struct PreprocessingConfig {
    pub frame_length: usize,
    pub frame_shift: usize,
    pub window_type: WindowType,
}

pub enum WindowType {
    Hamming,
    Hanning,
}

pub fn preprocess(signal: &Array1<f32>, config: &PreprocessingConfig) -> Array2<f32> {
    let framed_signal = frame_signal(signal, config.frame_length, config.frame_shift);
    let windowed_signal = apply_window(&framed_signal, &config.window_type);
    windowed_signal
}

fn frame_signal(signal: &Array1<f32>, frame_length: usize, frame_shift: usize) -> Array2<f32> {
    let n_frames = 1 + (signal.len() - frame_length) / frame_shift;
    let mut frames = Array2::zeros((n_frames, frame_length));

    for (i, mut frame) in frames.outer_iter_mut().enumerate() {
        let start = i * frame_shift;
        let end = start + frame_length;
        frame.assign(&signal.slice(s![start..end]));
    }

    frames
}

fn apply_window(frames: &Array2<f32>, window_type: &WindowType) -> Array2<f32> {
    let frame_length = frames.shape()[1];
    let window = match window_type {
        WindowType::Hamming => hamming_window(frame_length),
        WindowType::Hanning => hanning_window(frame_length),
    };

    frames * &window
}

fn hamming_window(length: usize) -> Array1<f32> {
    Array::linspace(0.0, (length - 1) as f32, length)
        .mapv(|n| 0.54 - 0.46 * (2.0 * std::f32::consts::PI * n / ((length - 1) as f32)).cos())
}

fn hanning_window(length: usize) -> Array1<f32> {
    Array::linspace(0.0, (length - 1) as f32, length)
        .mapv(|n| 0.5 * (1.0 - (2.0 * std::f32::consts::PI * n / ((length - 1) as f32)).cos()))
}
