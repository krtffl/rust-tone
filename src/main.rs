mod feature_extraction;
mod preprocessing;

use feature_extraction::{extract_features, SpectralConfig};
use ndarray::prelude::*;
use ndarray_npy::ReadNpyExt;
use preprocessing::{preprocess, PreprocessingConfig, WindowType};
use std::fs::File;

fn main() {
    let signal = Array1::<f32>::read_npy(File::open("path/to/signal.npy").unwrap()).unwrap();

    let config = PreprocessingConfig {
        frame_length: 512,
        frame_shift: 256,
        window_type: WindowType::Hamming,
    };

    let preprocessed_signal = preprocess(&signal, &config);

    let config = SpectralConfig {
        n_mfcc: 13,
        n_fft: 1024 / 2,
    };

    let sample_rate = 16000;
    let min_pitch = 75.0;
    let max_pitch = 300.0;

    let (mfcc, pitch_values, voiced_frames) = extract_features(
        &preprocessed_signal,
        sample_rate,
        &config,
        min_pitch,
        max_pitch,
    );

    println!("MFCC: {:?}", mfcc);
    println!("Pitch values: {:?}", pitch_values);
    println!("Voiced/unvoiced decisions: {:?}", voiced_frames);

    // Perform further processing or analysis on the preprocessed_signal
}
