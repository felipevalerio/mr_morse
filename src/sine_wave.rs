use std::i16::MAX;
use std::fs::File;
use std::f32::consts::PI;
use std::io::{BufReader, BufWriter, Read, Write};
use rodio::buffer::SamplesBuffer;
use rodio::{source, OutputStreamHandle};
use rodio::{OutputStream, source::Source};


// const AMPLITUDE: f32 = 1.0;
const FREQUENCY: f32 = 440.0;
const SAMPLE_RATE: f32 = 44100.0;
const CHANNELS: usize = 1;
const BYTES_PER_SAMPLE: usize = 2;


pub fn create_sine_wave(writer: &mut BufWriter<File>, duration: f32) -> std::io::Result<()> {

	let num_samples = (SAMPLE_RATE * duration) as usize;

	for i in 0..num_samples {
		let sample = (2.0 * PI * FREQUENCY * i as f32 / SAMPLE_RATE).sin();
		let int_sample = (sample * MAX as f32) as i16;
		writer.write_all(&int_sample.to_le_bytes())?;
	}

	Ok(())
}


pub fn silence_periods(writer: &mut BufWriter<File>, duration: f32) -> std::io::Result<()> {

	let num_samples = (SAMPLE_RATE * duration) as usize;

	for _ in 0..num_samples {
		writer.write_all(&0i16.to_le_bytes())?;
	}

	Ok(())
}


fn raw_pcm_format(buffer: Vec<u8>) -> SamplesBuffer<i16> {

	
	// Converter os bytes lidos em amostras de i16 (áudio 16-bit)
	let samples: Vec<i16> = buffer
		.chunks_exact(BYTES_PER_SAMPLE)
		.map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
		.collect();
	
	// Criar uma fonte a partir do buffer de amostras
	let source = SamplesBuffer::new(CHANNELS as u16, SAMPLE_RATE as u32, samples.clone());

	source
}


pub fn play_audio() {

	// inicia o stream de audio
	let (_stream, stream_handle) = OutputStream::try_default().unwrap();
	
	// carrega o arquivo binário
	let mut file = BufReader::new(File::open("C:/Users/Pichau/Documents/Rust/morse_encode_decode/sine_wave.bin").unwrap());
	let mut buffer = Vec::new();
	file.read_to_end(&mut buffer).unwrap();

	let source = raw_pcm_format(buffer);

	// calcula a duração do aúdio
	// total de samples / (sample_rate (frequencia 44100) * channels (1 ou seja Mono))
	let total_samples = samples.len() as u64;
	let duration_secs = total_samples / (SAMPLE_RATE as u64 * CHANNELS as u64);

    let _ = stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(duration_secs)); // toca durante o tempo do áudio
}



// y(t): valor da onda no tempo tt.
// A: amplitude da onda, que determina o "tamanho" da onda (ou quão alta ela é).
// f: frequência da onda em hertz (Hz), que determina quantas oscilações completas ocorrem por segundo.
// t: tempo.
// φ: fase inicial, que desloca a onda no tempo.

// y(t)=A⋅sin(2πft+φ) -> 2.0 * PI * FREQUENCY * i as f32 / SAMPLE_RATE).sin();
// ffplay -f s16le -ar 44100 sine_wave.bin