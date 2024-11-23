use std::{collections::HashMap, fs::File, io::BufWriter};

mod sine_wave;
use crate::sine_wave::{create_sine_wave, silence_periods, play_audio};

fn main() -> std::io::Result<()> {
    

    // variáveis para a definição de tempo para pontos, traços e de silêncio
    let dot_time: f32 = 0.1;
    let dash_time: f32 = dot_time * 3.0;
    let silence_time = dot_time;
    let space_time = dot_time * 7.0;
        
    // frase e codificação para morse
    let phrase = "SOS";
    let phrase_uppercase = phrase.to_uppercase();
    let mut string_encoded: Vec<&str> = Vec::new();
    
    // cria um arquivo binário
    let file = File::create("sine_wave.bin").unwrap();
    let mut writer = BufWriter::new(file);


    // alfabeto código morse
    let morse_code = HashMap::from([
            ("A", ".-"), ("B", "-..."), ("C", "-.-."), ("D", "-.."),
            ("E", "."), ("F", "..-."), ("G", "--."), ("H", "...."),
            ("I", ".."), ("J", ".---"), ("K", "-.-"), ("L", ".-.."),
            ("M", "--"), ("N", "-."), ("O", "---"), ("P", ".--."),
            ("Q", "--.-"), ("R", ".-."), ("S", "..."), ("T", "-"),
            ("U", "..-"), ("V", "...-"), ("W", ".--"), ("X", "-..-"),
            ("Y", "-.--"), ("Z", "--.."), ("1", ".----"), ("2", "..---"),
            ("3", "...--"), ("4", "....-"), ("5", "....."), ("6", "-...."),
            ("7", "--...."), ("8", "---.."), ("9", "----."), ("0", "-----")
    ]);
    


    for letter in phrase_uppercase.chars() {

        let key: String = letter.to_string();
        let key_str = key.as_str();
        
        if let Some(&value) = morse_code.get(key_str) {

            string_encoded.push(value);
        } 
        // else {
        //     println!("Erro");
        // }
    }


    for code in string_encoded {
        print!("{} ", code);

        for symbol in code.chars() {

            match symbol {
                '.' => create_sine_wave(&mut writer, dot_time)?,
                '-' => create_sine_wave(&mut writer, dash_time)?,
                _=> (),
            }
            let _ = silence_periods(&mut writer, silence_time);
        }
        
    }

    let _ = silence_periods(&mut writer, space_time);

    play_audio();

    Ok(())
}
