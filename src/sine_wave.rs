use std::fs::File;
use std::io::{BufWriter, Write};
use std::f32::consts::PI;


// const AMPLITUDE: f32 = 1.0;
const FREQUENCY: f32 = 440.0;
const SAMPLE_RATE: f32 = 44100.0;


pub fn create_sine_wave(writer: &mut BufWriter<File>, duration: f32) -> std::io::Result<()> {

	let num_samples = (SAMPLE_RATE * duration) as usize;

	for i in 0..num_samples {
		let sample = (2.0 * PI * FREQUENCY * i as f32 / SAMPLE_RATE).sin();
		let int_sample = (sample * i16::MAX as f32) as i16;
		writer.write_all(&int_sample.to_le_bytes())?;
	}

	Ok(())
}


pub fn silence_bet(writer: &mut BufWriter<File>, duration: f32) -> std::io::Result<()> {

	let num_samples = (SAMPLE_RATE * duration) as usize;

	for _ in 0..num_samples {
		writer.write_all(&0i16.to_le_bytes())?;
	}

	Ok(())
}
