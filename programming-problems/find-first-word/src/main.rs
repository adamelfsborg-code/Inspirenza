use std::io::{stdin,stdout,Write};

fn main() {
    let mut sentence=String::new();
    loop {
        print!("Please enter some text: ");
        stdout().flush().unwrap();
        match stdin().read_line(&mut sentence) {
            Err(err) => {
                println!("{err}")
            },
            Ok(_) => {break}
        }
    }
    let word = find_first_word(&mut sentence);
    println!("The first word is {}", word)
}


fn find_first_word(sentence: &mut String) -> &str {
    let mut word = sentence.split(' ');
    word.next().unwrap()
}