use serde_derive::{Deserialize, Serialize};

use crate::preprocessing::WindowType;

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtractedFeatures {
    pub filename: String,
    pub mfcc: Vec<Vec<f32>>,
    pub pitch_values: Vec<f32>,
    pub voiced_frames: Vec<bool>,
}

#[derive(Debug)]
pub struct PreprocessingConfig {
    pub frame_length: usize,
    pub frame_shift: usize,
    pub window_type: WindowType,
    pub sample_rate: usize,
}

#[derive(Debug)]

pub struct SpectralConfig {
    pub n_mfcc: usize,
    pub n_fft: usize,
}

#[derive(Debug)]

pub struct PitchConfig {
    pub min_pitch: f32,
    pub max_pitch: f32,
}

#[derive(Debug)]

pub struct FeatureConfig {
    pub zcr_threshold: f32,
    pub energy_threshold: f32,
}

pub fn get_configs() -> (
    PreprocessingConfig,
    SpectralConfig,
    PitchConfig,
    FeatureConfig,
) {
    let preprocessing_config = PreprocessingConfig {
        frame_length: 1024,
        frame_shift: 256,
        window_type: WindowType::Hamming,
        sample_rate: 44100,
    };

    let spectral_config = SpectralConfig {
        n_mfcc: 13,
        n_fft: preprocessing_config.frame_length / 2,
    };

    let pitch_config = PitchConfig {
        min_pitch: 75.0,  // 100 for female
        max_pitch: 300.0, // 500 for female
    };

    let feature_config = FeatureConfig {
        zcr_threshold: 0.5,
        energy_threshold: 0.01,
    };

    (
        preprocessing_config,
        spectral_config,
        pitch_config,
        feature_config,
    )
}
