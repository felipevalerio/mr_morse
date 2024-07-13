mod sine_wave;
use std::collections::HashMap;


fn main() {
    
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

    let mut duration: f32;
    let phrase = "SOS";
    let mut string_encoded: Vec<&str> = Vec::new();

    for letter in phrase.chars() {

        let key: String = letter.to_string();
        let key_str = key.as_str();
        
        if let Some(&value) = morse_code.get(key_str) {

            string_encoded.push(value);
        } else {
            println!("Erro");
        }
    }

    for code in string_encoded {
        
        if code == "." {
            duration = 0.1;
            sine_wave::sine_wave(duration);
        } else if code == "-" {
            duration = 0.3;
            sine_wave::sine_wave(duration);
        }
    }


}
