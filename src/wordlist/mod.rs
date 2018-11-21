use std::fs::File;
//use rand::seq::SliceRandom;
use ::rand::Rng;
use ::rand;
use std::io;
use std::io::Read;
use std::io::Seek;

use ::candidate::Candidate;

struct Wordlist {
    _name: String,
    _source: String,
    list: Vec<String>
}

impl Wordlist {
    fn new() -> Wordlist {
        let mut contents = String::new();
        File::open("src/wordlist/legacy.txt")
            .expect("Could not open file")
            .read_to_string(&mut contents)
            .expect("Couldn't put in String");
        let mut list = Vec::new();
        for word in contents.split("\n") {
            list.push(word.to_string());
        }
        
        Wordlist {
            _name: "legacy".to_string(),
            _source: "src/wordlist/legacy.txt".to_string(),
            list: list
        }
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

    #[test]
    fn new_good1() {
        let list = Wordlist::new();
        assert_eq!("abets".to_string(), list.list[0]);
    }
    
    #[test]
    fn choose_secret_good1() {
        //Just make sure it doesn't panic
        choose_secret();
    }
}
