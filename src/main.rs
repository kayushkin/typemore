use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rand::{thread_rng, Rng};
use std::io::{self, Write};

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
    let correct_words = 5; //words.len();
    let mut correct = 0;
    let mut words_typed = 0;
    //randomly sort the words
    words.sort_by(|_, _| thread_rng().gen::<u8>().cmp(&0));

    let mut words_per_line = 5;
    while words_typed < correct_words {
        if words_per_line + words_typed > correct_words {
            words_per_line = correct_words - words_typed;
        }
        let current_words = words[words_typed..words_typed + words_per_line].join(" ");
        println!("{}", current_words);
        let mut current_word = String::new();
        let _ = enable_raw_mode();
        loop {
            match read() {
                Ok(Event::Key(event)) => match event.code {
                    KeyCode::Char(' ') => {
                        print!("{}", " ");
                        if words[words_typed] == current_word {
                            correct += 1;
                        }
                        words_typed += 1;
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
            if words_typed == correct_words {
                break;
            }
        }
        let _ = disable_raw_mode();
    }

    println!("You got {} out of {} correct!", correct, correct_words);
}
