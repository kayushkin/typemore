use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rand::{seq::SliceRandom, thread_rng};
use std::io::{self, Write};
use std::time::Instant;

fn main() {
    let mut words: Vec<&str> = [
        "cat",
        "Gelatin",
        "VsCode",
        "Blue",
        "Vector",
        "dog",
        "frog",
        "mountain",
        "Garbage",
        "Rust",
        "apple",
        "banana",
        "cherry",
        "date",
        "fig",
        "grape",
        "kiwi",
        "lemon",
        "mango",
        "orange",
        "papaya",
        "pear",
        "quince",
        "raspberry",
        "strawberry",
        "watermelon",
        "avocado",
        "blackberry",
    ]
    .to_vec();
    let correct_words = 18; //words.len();
    let mut words_per_line = 10;
    let mut correct = 0;
    let mut words_typed = 0;
    //randomly sort the words
    let mut rng = thread_rng();
    let mut continue_word = false;
    words.shuffle(&mut rng);

    let start = Instant::now();
    while words_typed < correct_words {
        if words_per_line + words_typed > correct_words {
            words_per_line = correct_words - words_typed;
        }
        let current_words = words[words_typed..words_typed + words_per_line].join(" ");
        println!("{}", current_words);
        let _ = enable_raw_mode();
        let mut current_word = String::new();
        loop {
            match read() {
                Ok(Event::Key(event)) => match event.code {
                    KeyCode::Char(' ') => {
                        print!("{}", " ");
                        if words[words_typed] == current_word {
                            correct += 1;
                        }
                        words_typed += 1;
                        continue_word = true;
                        current_word.clear();
                        if words_typed == correct_words {
                            break;
                        }
                    }
                    KeyCode::Char(c) => {
                        print!("{}", c);
                        current_word.push(c);
                        io::stdout().flush().unwrap();
                    }
                    _ => {}
                },
                _ => {}
            }
            if continue_word && words_typed % words_per_line == 0 {
                continue_word = false;
                break;
            }
        }
        let _ = disable_raw_mode();
        println!("");
        if words_typed >= correct_words {
            let elapsed = start.elapsed();
            println!("Time: {:?}", elapsed);
            println!("Words typed: {}", words_typed);
            println!("time secs: {}", elapsed.as_secs_f64() as f64);
            println!(
                "{}: wpm",
                correct as f64 / (elapsed.as_secs_f64() / 60.0) as f64
            );
            break;
        }
    }

    println!("You got {} out of {} correct!", correct, correct_words);
}
