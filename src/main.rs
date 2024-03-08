use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rand::{seq::SliceRandom, thread_rng};
use std::io::{self, Write};
use std::time::Instant;

fn main() {
    // big list of random words
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
        "blueberry",
        "Artichoke",
        "Asparagus",
        "Beet",
        "Broccoli",
        "Cabbage",
        "Carrot",
        "Cauliflower",
        "Celery",
        "Corn",
        "Cucumber",
        "Eggplant",
        "car",
        "truck",
        "bus",
        "Asphalt",
        "Bicycle",
        "warehouse",
        "Cement",
        "yellow",
        "Daisy",
        "Dandelion",
        "Daffodil",
    ]
    .to_vec();

    let total_words = 18; //words.len();
    let mut words_per_line = 10;
    let mut correct = 0;
    let mut words_typed = 0;
    //randomly sort the words
    let mut rng = thread_rng();
    let mut continue_word = false;
    words.shuffle(&mut rng);

    let start = Instant::now();
    while words_typed < total_words {
        if words_per_line + words_typed > total_words {
            words_per_line = total_words - words_typed;
        }
        let mut end_word = words_typed + words_per_line;
        if words_typed + words_per_line > total_words {
            end_word = total_words;
        }
        let current_words = words[words_typed..end_word].join(" ");
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
                        if words_typed == total_words {
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
            if continue_word && words_typed % 10 % words_per_line == 0 || words_typed >= total_words
            {
                continue_word = false;
                break;
            }
        }
        let _ = disable_raw_mode();
        println!("");

        if words_typed >= total_words {
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

    println!("You got {} out of {} correct!", correct, total_words);
}
