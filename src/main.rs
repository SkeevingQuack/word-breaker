extern crate rand;

//use rand::seq::SliceRandom;
use rand::Rng;

use std::cmp::Ordering;

use std::io;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::fs::File;

fn main() {
    let mut f = File::open("src/wordlist/final.txt")
        .expect("File not found");
    let mut secret = String::new();

    //final.txt is currently 5454 words long
    //Must have unix line endings
    let word_offset = rand::thread_rng().gen_range(0, 5454);
    f.seek(SeekFrom::Start(word_offset*6))
        .expect("seek failure");
    let mut handle = f.take(5);
    handle.read_to_string(&mut secret)
        .expect("read failed");
    
    println!("The secret word is {:?}", secret);


    
    loop {
        println!("Please input your guess.");
        //following line can't be outside loop. Why?
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim().cmp(&secret) {
            Ordering::Less => println!("Too early!"),
            Ordering::Greater => println!("Too late!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

}
