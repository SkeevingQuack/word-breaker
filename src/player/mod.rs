use ::candidate::{Letter, Candidate};
use ::wordlist::Wordlist;

use std::io;

pub fn play() {
    let mut list = Wordlist::new();
    let mut word_record: Vec<Candidate> = Vec::new();
    let mut score_record: Vec<u8> = Vec::new();

    let mut temp_blacklist = String::new();
    let mut temp_hints = 0u8;
    loop {
        println!("{}", &temp_blacklist);
        match list.get_word("", &temp_blacklist) {
            Some(word) => {
                match receive_score(word) {
                    Some(num) => {
                        score_record.push(num);
                        temp_hints += num;
                        word_record.push(word);
                        temp_blacklist = format!("{}{}", temp_blacklist, word);
                        
                    }
                    None => continue
                }
                if temp_hints == 5 { break; }
                assert!(temp_hints < 6);
            }
            None => break
        }
    }

    for (word, score) in word_record.iter().zip(score_record.iter()) {
        println!("{}: {}", word, score);
    }
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
