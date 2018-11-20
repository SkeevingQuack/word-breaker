use ::candidate::{Letter, Candidate};
use self::Message::*;
use self::Response::*;

pub enum Message {
    //Make a guess
    Guess(Candidate),
    //Mark letter as confirmed
    MarkInclude(Letter),
    //Mark letter as unneeded
    MarkExclude(Letter),
    //Save state of current marks
    MarkBranch
}

pub enum Response<'a> {
    //Word guessed; advance level
    LevelUp(GameState),
    //Word not guessed in time; game over
    GameOver(GameState),
    //Duplicate guess, impossible hint, etc.
    Invalid(GameState, &'a str),
    //Message processed; game continues apace
    Step(GameState)
}

#[derive(Debug)]
pub struct GameState {
    secret: Candidate,
    score: u16,

    level: u8,
    guesses: u8,
    guess_array: [Option<Candidate>; 30],
    hint_array: [u8; 30],

    marked_include: [bool; 26],
    marked_exclude: [bool; 26]
}
impl GameState {
    pub fn init() -> GameState {
        GameState {
            secret: choose_secret(),
            score: 0,
            level: 1,
            guesses: 0,
            guess_array: [None; 30],
            hint_array: [0; 30],
            marked_include: [false; 26],
            marked_exclude: [false; 26]
        }
    } 

    pub fn process<'a>(mut self, message: Message) -> Response<'a> {
        match message {
            Guess(word) => {
                let pushed = self.push_guess(&word);
                if !pushed {
                    Invalid(self, "Duplicate guess")
                } else if word == self.secret {
                    LevelUp(self)
                    //TODO: win if any 5 result
                } else if self.guesses == self.guess_limit() {
                    GameOver(self)
                } else {
                    Step(self)
                }
            }
            MarkInclude(letter) => {
                Invalid(self, "Not implemented")
            }
            MarkExclude(letter) => {
                Invalid(self, "Not implemented")
            }
            MarkBranch => {
                Invalid(self, "Not implemented")
            }
        }        
    }

    fn push_guess(&mut self, guess: &Candidate) -> bool {
        //Look through guess_array
        //  If `guess` already exists, return false. Place `guess` in
        //  first open slot and update `guesses` and `hint_array`
        let mut i = 0;
        for opt in self.guess_array.iter() {
            i += 1;
            match opt {
                Some(word) => { if word == guess {return false;} }
                None => { break; }
            }   
        }
        self.guess_array[i] = Some(*guess);
        self.guesses += 1;
        self.hint_array[i] = self.secret.compare(*guess);
                    
        panic!("push_guess should not reach here");
    }
    fn guess_limit(&self) -> u8 {
        let mut limit = 127;
        if self.level > 5 {
            limit = 5;
        } else {
            limit = self.level - 1;
        }
        30 - limit * 4
    }
    fn clone(&self) -> GameState {
        GameState {
            secret: self.secret.clone(),
            score: self.score,
            level: self.level,
            guesses: self.guesses,
            guess_array: self.guess_array.clone(),
            hint_array: self.hint_array.clone(),
            marked_include: self.marked_include.clone(),
            marked_exclude: self.marked_exclude.clone()
        }
    }
}

use std::fs::File;
//use rand::seq::SliceRandom;
use ::rand::Rng;
use ::rand;
use std::io;
use std::io::Read;
use std::io::Seek;

fn choose_secret() -> Candidate {
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
    use super::*;

    #[test]
    fn choose_secret_good1() {
        //Just make sure it doesn't panic
        choose_secret();
    }
}
        
