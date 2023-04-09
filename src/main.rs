mod config;
mod feature_extraction;
mod preprocessing;
mod utils;

use feature_extraction::extract_features;
use hound;
use ndarray::prelude::*;
use preprocessing::preprocess;
use std::{env, path::Path};

use crate::{config::get_configs, utils::plot_features};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_signal.npy>", args[0]);
        return;
    }

    let wav_reader = hound::WavReader::open(Path::new(&args[1])).expect("Failed to open WAV file");
    let samples: Vec<f32> = wav_reader
        .into_samples::<f32>()
        .filter_map(Result::ok)
        .collect();
    let signal = Array1::from(samples);

    let (preprocessing_config, spectral_config, sample_rate, min_pitch, max_pitch) = get_configs();
    let preprocessed_signal = preprocess(&signal, &preprocessing_config);

    let (mfcc, pitch_values, voiced_frames) = extract_features(
        &preprocessed_signal,
        sample_rate,
        &spectral_config,
        min_pitch,
        max_pitch,
    );

    println!("MFCC: {:?}", mfcc);
    println!("Pitch values: {:?}", pitch_values);
    println!("Voiced/unvoiced decisions: {:?}", voiced_frames);

    plot_features(&mfcc, &pitch_values, &voiced_frames);

    // Perform further processing or analysis on the preprocessed_signal
}
