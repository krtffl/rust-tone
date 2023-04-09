use crate::preprocessing::WindowType;

pub struct PreprocessingConfig {
    pub frame_length: usize,
    pub frame_shift: usize,
    pub window_type: WindowType,
}

pub struct SpectralConfig {
    pub n_mfcc: usize,
    pub n_fft: usize,
}

pub fn get_configs() -> (PreprocessingConfig, SpectralConfig, usize, f32, f32) {
    let preprocessing_config = PreprocessingConfig {
        frame_length: 512,
        frame_shift: 256,
        window_type: WindowType::Hamming,
    };

    let spectral_config = SpectralConfig {
        n_mfcc: 13,
        n_fft: 1024 / 2,
    };

    let sample_rate: usize = 16000;
    let min_pitch = 75.0;
    let max_pitch = 300.0;

    (
        preprocessing_config,
        spectral_config,
        sample_rate,
        min_pitch,
        max_pitch,
    )
}
