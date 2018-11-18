
use std::fs::File;

//use rand::seq::SliceRandom;
use ::rand::Rng;
use ::rand;

use std::io;
use std::io::Read;
use std::io::Seek;


pub enum Message {
    //Set up initial game state
    //Init,

    //Make a guess
    Guess(Candidate),
    //Mark letter as confirmed
    Mark_include(Letter),
    //Mark letter as unneeded
    Mark_exclude(Letter),
    //Save state of current marks
    Mark_branch
}

enum Response {
    //Word guessed; advance level
    Success(Game_state),
    //Word not guessed in time; game over
    Failure(Game_state),
    //Duplicate guess, impossible hint, etc.
    Invalid(String),
    //Message processed; game continues apace
    Step(Game_state)
}

struct Game_state {
    Secret: Candidate,
    Score: u16,

    Level: u8,
    Guesses_made: u8,
    Guess_array: [Candidate; 34],

    Marked_include: [bool; 26],
    Marked_exclude: [bool; 26]
}

impl Game_state {
    pub fn init() -> Game_state {
        Game_state {
            Secret: choose_secret(),
            Score: 0,
            Level: 1,
            Guesses_made: 0,
            //Guess_array:
            Marked_include: [bool; false],
            Marked_exclude: [bool; false]
    } 

    pub fn process(&self, message: Message) -> Response {

    }
}

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

    Candidate(secret)
}
