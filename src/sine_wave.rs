use std::fs;
use std::io::Write;
use std::f32::consts::PI;


const AMPLITUDE: f32 = 0.5;
const FREQUENCY: f32 = 440.0;
const SAMPLE_RATE: f32 = 44100.0;


pub fn create_sine_wave(duration: f32, mut buffer: [u8; 4]) {
	print!("Passei por aqui :)");

	let num_samples = (SAMPLE_RATE * duration) as usize;

	let mut data_file = fs::OpenOptions::new()
		.append(true)
		.open("out.bin")
		.expect("erro na abertura do arquivo");

	for i in 0..num_samples {
		let sample = AMPLITUDE * (2.0 * PI * FREQUENCY * i as f32 / SAMPLE_RATE).sin();
		buffer.copy_from_slice(&sample.to_le_bytes());
		data_file.write(&buffer).expect("erro");
	}
}