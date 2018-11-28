use ::candidate::{Letter, Candidate};
use ::wordlist::Wordlist;

use std::io;

pub fn play() {
    let mut list = Wordlist::new();

    let mut word_record: Vec<Candidate> = Vec::new();
    let mut score_record: Vec<u8> = Vec::new();
    
    loop {
        let word = list.get_word("", "").expect("Full list is empty");
    
        match receive_score(word) {
            Some(num) => score_record.push(num),
            None => continue
        }
        word_record.push(word);
        break
    }
    println!("{}: {}", word_record[0], score_record[0]);
}

fn receive_score(prompt: Candidate) -> Option<u8> {
    println!("{}", prompt);
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("stdin error");
        let input = input.trim();

        if input == "rejected" {return None};
        match input.parse::<u8>() {
            Ok(num) => {
                if num < 6 {return Some(num)}
                println!("Score too high. Try again");
                continue;
            }
            Err(_e) => {
                println!("Input not understood. 0-5 or 'rejected'");
                continue;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;


}
