use ndarray::prelude::*;
use rustdct::DctPlanner;
use rustfft::{num_complex::Complex, FftPlanner};

use crate::config::SpectralConfig;

pub fn extract_features(
    signal: &Array2<f32>,
    sample_rate: usize,
    config: &SpectralConfig,
    min_pitch: f32,
    max_pitch: f32,
) -> (Array2<f32>, Vec<f32>, Vec<bool>) {
    let magnitude_spectrum = compute_magnitude_spectrum(signal, config.n_fft);
    let mfcc = compute_mfcc(&magnitude_spectrum, config.n_mfcc, sample_rate);
    let normalized_mfcc = apply_cmvn(&mfcc);

    let num_frames = signal.shape()[0];
    let mut pitch_values = Vec::new();
    let mut voiced_frames = Vec::new();

    for i in 0..num_frames {
        let frame_view = signal.slice(s![i, ..]);
        let frame = frame_view.to_owned();
        let pitch = compute_pitch(&frame, sample_rate, min_pitch, max_pitch);
        pitch_values.push(pitch);

        let zcr = compute_zcr(&frame);
        let energy = compute_energy(&frame);

        // Define your thresholds for ZCR and energy
        let zcr_threshold = 0.15;
        let energy_threshold = 1000.0;

        // Decide if the frame is voiced or unvoiced
        let is_voiced = zcr < zcr_threshold && energy > energy_threshold;
        voiced_frames.push(is_voiced);
    }

    (normalized_mfcc, pitch_values, voiced_frames)
}

fn apply_cmvn(mfcc: &Array2<f32>) -> Array2<f32> {
    let mean = mfcc.mean_axis(Axis(0)).unwrap();
    let std_dev = mfcc.std_axis(Axis(0), 0.0);

    let mut normalized_mfcc = mfcc.clone();

    for (index, mut feature) in normalized_mfcc.axis_iter_mut(Axis(1)).enumerate() {
        feature.mapv_inplace(|v| v - mean[index]);
        feature.mapv_inplace(|v| v / std_dev[index]);
    }

    normalized_mfcc
}

fn compute_zcr(frame: &Array1<f32>) -> f32 {
    let zcr = frame
        .windows(2)
        .into_iter()
        .map(|w| if w[0] * w[1] < 0.0 { 1.0 } else { 0.0 })
        .sum::<f32>();

    zcr / (frame.len() - 1) as f32
}

fn compute_energy(frame: &Array1<f32>) -> f32 {
    frame.iter().map(|x| x * x).sum::<f32>()
}

fn compute_pitch(audio: &Array1<f32>, sample_rate: usize, min_pitch: f32, max_pitch: f32) -> f32 {
    let frame_size = (sample_rate as f32 / min_pitch).ceil() as usize;
    let min_lag = (sample_rate as f32 / max_pitch).ceil() as usize;

    let mut autocorrelation = Array1::zeros(frame_size);

    for i in 0..frame_size {
        let sub_audio = audio.slice(s![i..]);
        let sub_audio_shifted = audio.slice(s![..audio.len() - i]);

        autocorrelation[i] = sub_audio_shifted
            .iter()
            .zip(sub_audio.iter())
            .map(|(x, y)| x * y)
            .sum::<f32>();
    }

    let mut max_corr = 0.0;
    let mut max_lag = min_lag;

    for lag in min_lag..frame_size {
        if autocorrelation[lag] > max_corr {
            max_corr = autocorrelation[lag];
            max_lag = lag;
        }
    }

    (sample_rate as f32) / max_lag as f32
}

fn compute_magnitude_spectrum(signal: &Array2<f32>, n_fft: usize) -> Array2<f32> {
    let n_frames = signal.shape()[0];
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(n_fft);
    let mut spectrum = Array2::zeros((n_frames, n_fft / 2 + 1));

    for (i, frame) in signal.outer_iter().enumerate() {
        let mut complex_frame: Vec<Complex<f32>> = frame.mapv(|x| Complex::new(x, 0.0)).to_vec();
        fft.process(&mut complex_frame);

        let magnitude_frame = complex_frame
            .iter()
            .take(n_fft / 2 + 1)
            .map(|x| x.norm())
            .collect::<Array1<f32>>();
        spectrum.row_mut(i).assign(&magnitude_frame);
    }

    spectrum
}

fn compute_mfcc(spectrum: &Array2<f32>, n_mfcc: usize, sample_rate: usize) -> Array2<f32> {
    let n_fft = 2 * (spectrum.shape()[1] - 1);
    let lower_frequency = 0.0;
    let upper_frequency = sample_rate as f32 / 2.0;

    let mel_filter_bank =
        create_mel_filter_bank(n_mfcc, n_fft, sample_rate, lower_frequency, upper_frequency);
    let mel_spectrum = spectrum.dot(&mel_filter_bank.t());
    let log_mel_spectrum = mel_spectrum.mapv(|x| x.ln());

    let mut mfcc = Array2::zeros(log_mel_spectrum.raw_dim());
    let mut dct_planner = DctPlanner::new();
    let dct = dct_planner.plan_dct2(log_mel_spectrum.shape()[1]);

    for (mut mfcc_row, _) in mfcc.outer_iter_mut().zip(log_mel_spectrum.outer_iter()) {
        let mut mfcc_row_slice = mfcc_row.as_slice_mut().unwrap();
        dct.process_dct2(&mut mfcc_row_slice);
    }

    mfcc.slice(s![.., ..n_mfcc]).to_owned()
}

fn create_mel_filter_bank(
    n_filters: usize,
    n_fft: usize,
    sample_rate: usize,
    lower_frequency: f32,
    upper_frequency: f32,
) -> Array2<f32> {
    let lower_mel = hertz2mel(lower_frequency);
    let upper_mel = hertz2mel(upper_frequency);

    let mel_points = Array::linspace(lower_mel, upper_mel, n_filters + 2);
    let freq_points = mel_points.mapv(mel2hertz);
    let fft_bins: Array1<usize> =
        freq_points.mapv(|x| (x * (n_fft as f32) / (sample_rate as f32)).round() as usize);

    let mut filter_bank = Array2::zeros((n_filters, n_fft / 2 + 1));

    for i in 0..n_filters {
        for (j, value) in filter_bank
            .slice_mut(s![i, fft_bins[i]..fft_bins[i + 1]])
            .iter_mut()
            .enumerate()
        {
            *value = j as f32 / (fft_bins[i + 1] - fft_bins[i]) as f32;
        }
        for (j, value) in filter_bank
            .slice_mut(s![i, fft_bins[i + 1]..fft_bins[i + 2]])
            .iter_mut()
            .enumerate()
        {
            *value = 1.0 - (j as f32 / (fft_bins[i + 2] - fft_bins[i + 1]) as f32);
        }
    }

    filter_bank
}

fn hertz2mel(frequency: f32) -> f32 {
    2595.0 * (1.0 + frequency / 700.0).log10()
}

fn mel2hertz(mel: f32) -> f32 {
    700.0 * (10.0f32.powf(mel / 2595.0) - 1.0)
}
