use std::fs::File;
//use rand::seq::SliceRandom;
use ::rand::Rng;
use ::rand;
use std::io;
use std::io::Read;
use std::io::Seek;

use ::candidate::Candidate;

pub fn choose_secret() -> Candidate {
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
    fn choose_secret_good1() {
        //Just make sure it doesn't panic
        choose_secret();
    }
}
