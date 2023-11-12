use std::io::{stdin, stdout, Write};

fn main() {
    let sentence = get_input("Please enter some text: ");
    let word = find_first_word(&sentence);
    println!("The first word is '{}'", word);
}

fn get_input(prompt: &str) -> String {
    loop {
        print!("{}", prompt);
        stdout().flush().unwrap();
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Err(err) => println!("{err}"),
            Ok(_) => return input.trim().to_string(),
        }
    }
}

fn find_first_word(sentence: &str) -> &str {
    sentence.split_whitespace().next().unwrap_or("")
}