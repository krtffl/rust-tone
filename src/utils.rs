use std::{fs::File, io::Write, ops::Range, path::Path};

use image::RgbImage;
use ndarray::{Array2, Axis};
use plotters::prelude::*;

use crate::config::ExtractedFeatures;

pub fn plot_features(
    mfcc: &Array2<f32>,
    pitch_values: &[f32],
    voiced_frames: &[bool],
    index: usize,
) {
    let num_frames = pitch_values.len();
    let width = 1200;
    let height = 400;

    {
        let mut mfcc_image = RgbImage::new(width, height);

        {
            let root = BitMapBackend::with_buffer(mfcc_image.as_mut(), (width, height))
                .into_drawing_area();
            root.fill(&WHITE).unwrap();

            let mut mfcc_chart = ChartBuilder::on(&root)
                .caption("MFCCs computation", ("Arial", 20).into_font())
                .margin(5)
                .x_label_area_size(30)
                .y_label_area_size(60)
                .build_cartesian_2d::<Range<usize>, Range<f32>>(0..num_frames, -1.0..1.0)
                .unwrap();

            mfcc_chart
                .configure_mesh()
                .x_desc("time (frames)")
                .y_desc("amplitude")
                .draw()
                .unwrap();

            for (i, mfcc_row) in mfcc.axis_iter(Axis(0)).enumerate() {
                let line_data: Vec<(usize, f32)> = mfcc_row
                    .iter()
                    .enumerate()
                    .map(|(j, &value)| (j, value))
                    .collect();
                mfcc_chart
                    .draw_series(LineSeries::new(line_data, &Palette99::pick(i)))
                    .unwrap();
            }
        }

        mfcc_image
            .save(format!("mffc_plots/mfcc_plot-{}.png", index))
            .unwrap();
    }

    {
        let mut pitch_image = RgbImage::new(width, height);

        {
            let root = BitMapBackend::with_buffer(pitch_image.as_mut(), (width, height))
                .into_drawing_area();
            root.fill(&WHITE).unwrap();

            let max_pitch = pitch_values
                .iter()
                .copied()
                .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap_or(0.0);

            let mut pitch_chart = ChartBuilder::on(&root)
                .caption("Pitch analysis", ("Arial", 20).into_font())
                .margin(5)
                .x_label_area_size(30)
                .y_label_area_size(60)
                .build_cartesian_2d::<Range<usize>, Range<f32>>(
                    0..num_frames,
                    0.0..(max_pitch * 1.1),
                )
                .unwrap();

            pitch_chart
                .configure_mesh()
                .x_desc("time (frames)")
                .y_desc("frequency (Hz)")
                .draw()
                .unwrap();
            pitch_chart
                .draw_series(LineSeries::new(
                    pitch_values.iter().enumerate().map(|(i, &v)| (i, v)),
                    &RED,
                ))
                .unwrap();
        }

        pitch_image
            .save(format!("pitch_plots/pitch_plot-{}.png", index))
            .unwrap();
    }

    {
        let mut voiced_image = RgbImage::new(width, height);

        {
            let root = BitMapBackend::with_buffer(voiced_image.as_mut(), (width, height))
                .into_drawing_area();
            root.fill(&WHITE).unwrap();

            let mut voiced_chart = ChartBuilder::on(&root)
                .caption("Is voiced?", ("Arial", 20).into_font())
                .margin(5)
                .x_label_area_size(30)
                .y_label_area_size(60)
                .build_cartesian_2d(0..num_frames, 0..1)
                .unwrap();

            voiced_chart
                .configure_mesh()
                .x_desc("time (frames)")
                .y_desc("voiced (bool)")
                .x_labels(10)
                .y_labels(2)
                .draw()
                .unwrap();

            let voiced_data: Vec<(usize, i32)> = voiced_frames
                .iter()
                .enumerate()
                .map(|(i, &v)| (i, v as i32))
                .collect();

            voiced_chart
                .draw_series(LineSeries::new(voiced_data, &BLUE))
                .unwrap();
        }

        voiced_image
            .save(format!("voiced_plots/voiced_plot-{}.png", index))
            .unwrap();
    }
}

pub fn save_extracted_features_to_json(filename: &str, features_list: &[ExtractedFeatures]) {
    let json = serde_json::to_string_pretty(features_list).unwrap();
    let path = Path::new(filename);
    let mut file = File::create(&path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
