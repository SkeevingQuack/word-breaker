use std::fs::File;

use ::rand::Rng;
use ::rand;
use std::io;
use std::io::Read;
use std::io::Seek;

use rand::seq::IteratorRandom;
use ::candidate::{Candidate, Letter};

pub struct Wordlist {
    _name: String,
    _source: String,
    rng: rand::rngs::ThreadRng,
    full_list: Vec<String>,
    //TODO: Make list references to full_list
    // maybe raw pointer?
    list: Vec<String>
}

impl Wordlist {
    pub fn new() -> Wordlist {
        let mut contents = String::new();
        //File::open("src/wordlist/legacy.txt")
        File::open("src/wordlist/final.txt")  
            .expect("Could not open file")
            .read_to_string(&mut contents)
            .expect("Couldn't put in String");
        let mut full_list = Vec::new();
        for word in contents.split("\n") {
            let word = word.to_string();
            full_list.push(word);
        }
        
        Wordlist {
            _name: "legacy".to_string(),
            _source: "src/wordlist/legacy.txt".to_string(),
            rng: rand::thread_rng(),
            list: full_list.clone(),
            full_list
        }
    }
    pub fn get_word(&mut self) -> Option<Candidate> {
        match self.list.iter().choose(&mut self.rng) {
            Some(word) => Some(Candidate::from_str(word.as_str())),
            None => None
        }
    }
    pub fn reset_list(&mut self) {
        // let newlist = Vec::new();
        // for word in self.full_list {
        //     newlist.push(&word[..]);
        // }
        // self.list = newlist;
        self.list = self.full_list.clone()
    }
    //pub fn exclude(&mut self, letter: Letter) {
    //    let letter = String::from(letter.get_alphabet_index() as u8 + 0x61);
    pub fn exclude(&mut self, letter: &str) {
        self.list.retain(|e| !e.contains(letter));
        // for (i, word) in self.list.iter().enumerate() {
        //     if !word.contains(letter) {
        //         self.list.remove(i);
        //     }
        // }
    }
    pub fn include(&mut self, letter: &str) {
        self.list.retain(|e| e.contains(letter));
    }
}
        

pub fn choose_secret() -> Candidate {
    //deprecated
    let mut f = File::open("src/wordlist/final.txt")
        .expect("File not found");
    let mut secret = String::new();

    //final.txt is currently 5454 words long
    //Must have unix line endings
    let word_offset = rand::thread_rng().gen_range(0, 5454);
    f.seek(io::SeekFrom::Start(word_offset*6))
        .expect("seek failure");
    let mut handle = f.take(5);
    handle.read_to_string(&mut secret)
        .expect("read failed");

    Candidate::from_str(&secret)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    //List runs out of options has weird error

    #[test]
    fn new_good1() {
        let list = Wordlist::new();
        assert_eq!("abets".to_string(), list.list[0].clone());
    }
    
    #[test]
    fn choose_secret_good1() {
        //Just make sure it doesn't panic
        choose_secret();
    }
}
