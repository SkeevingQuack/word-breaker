use ::candidate::{Letter, Candidate};
use self::Message::*;
use self::Response::*;
use ::wordlist::choose_secret;


pub enum Message {
    //Make a guess
    Guess(Candidate),
    //(un)Mark letter as confirmed
    MarkInclude(Letter),
    //(un)Mark letter as unneeded
    MarkExclude(Letter),
    //Save/load/delete state of current marks
    //MarkBranch(Function)
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
    //Always display score*10?
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
                //let pushed = self.push_guess(word.clone());
                let pushed = self.push_guess(word);
                if !pushed {
                    Invalid(self, "Duplicate guess")
                } else if word == self.secret {
                    LevelUp(self.level_up())
                    //TODO: win if any 5 result
                } else if self.guesses == self.guess_limit() {
                    GameOver(self)
                } else {
                    Step(self)
                }
            }
            MarkInclude(letter) => {
                let index = letter.get_alphabet_index();
                let mut count = 0;
                for mark in self.marked_include.iter() {
                    if *mark { count += 1; }
                }
                let count = count; //no mut
                
                if count == 5 {
                    return Invalid(self, "Already 5 marks")
                }
                self.marked_exclude[index] = false;
                self.marked_include[index] = !self.marked_include[index];
                Step(self)
            }
            MarkExclude(letter) => {
                let index = letter.get_alphabet_index();

                self.marked_include[index] = false;
                self.marked_exclude[index] = !self.marked_include[index];
                Step(self)
            }
        }        
    }
    
    fn level_up(self) -> GameState {
        //TODO: overflow for score, level
        GameState {
            //score must be (nearly) first
            score: self.score + self.score_points(),
            secret: choose_secret(),
            level: self.level + 1,
            guesses: 0,
            guess_array: [None; 30],
            hint_array: [0; 30],
            marked_include: [false; 26],
            marked_exclude: [false; 26]
        }
    }
    fn score_points(&self) -> u16 {
        //Scoring algorithm based on absolutely nothing
        let extra_guesses = u16::from(self.guess_limit() - self.guesses);
        let level_bonus = u16::from(self.level.pow(2)) * 100;
        (extra_guesses * self.level as u16 * 20) + level_bonus
    }
    fn push_guess(&mut self, guess: Candidate) -> bool {
        //Look through guess_array
        //  If `guess` already exists, return false. Place `guess` in
        //  first open slot and update `guesses` and `hint_array`
        let mut i = 0;
        for opt in self.guess_array.iter() {
            match *opt {
                Some(word) => { if word == guess {return false;} }
                None => { break; }
            }
            i += 1;
        }
        let i = i; //no mut
        self.guess_array[i] = Some(guess);
        self.guesses += 1;
        self.hint_array[i] = self.secret.compare(guess);
        true
    }
    fn guess_limit(&self) -> u8 {
        //TODO: `use` in arbitrary scope?
        use std::cmp::min;
        let limit = min(self.level, 6) - 1;
        30 - limit * 4
    }
    ////Underived to keep it private(?)
    // fn clone(&self) -> GameState {
    //     GameState {
    //         secret: self.secret.clone(),
    //         score: self.score,
    //         level: self.level,
    //         guesses: self.guesses,
    //         guess_array: self.guess_array.clone(),
    //         hint_array: self.hint_array.clone(),
    //         marked_include: self.marked_include.clone(),
    //         marked_exclude: self.marked_exclude.clone()
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn cfs(input: &str) -> Candidate {
        Candidate::from_str(input)
    }
    fn fill_guesses(cands: Vec<&str>) -> [Option<Candidate>; 30] {
        let mut array = [None; 30];
        let mut index = 0;
        for cand in cands {
            array[index] = Some(cfs(cand));
            index += 1;
        }
        for i in index..30 {
            array[i] = None;
        }
        array
    }
    fn create_test_game(option: u8) -> GameState {
        match option {
            0 => GameState::init(),
            1 => {
                let candidates = vec!["zonal", "zoned", "zoner", "zones",
                                      "abets", "abide", "abhor", "abode"];
                GameState {
                    secret: cfs("quips"),
                    score: 6600,
                    level: 6,
                    guesses: 8,
                    guess_array: fill_guesses(candidates),
                    hint_array: [0, 0, 0, 1, 1, 1, 0, 0,
                                 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    marked_include: [false, false, false, false,
                                     false, false, false, false,
                                     true,  false, false, false,
                                     false, false, false, false,
                                     false, false, true,  false,
                                     false, false, false, false,
                                     false, false],
                    marked_exclude: [true,  true,  false, true,
                                     true,  false, false, true,
                                     false, false, false, true,
                                     false, true,  true,  false,
                                     false, true,  false, false,
                                     false, false, false, false,
                                     false, true]
                }
            },
            _ => panic!("test_game option {} doesn't exist", option)
        }
    }    
    #[test]
    fn process_guess_good1() {
        let game = create_test_game(1);
        match game.process(Guess(cfs("quips"))) {
            LevelUp(_state) => assert!(true),
            _ => assert!(false)
        }
    }
    #[test]
    fn process_guess_good2() {
        let game = create_test_game(1);
        match game.process(Guess(cfs("quote"))) {
            Step(state) => {
                                
                assert_eq!( 9, state.guesses );
                assert_eq!( Some(cfs("quote")), state.guess_array[8] );
                assert_eq!( 2, state.hint_array[8] );
            },
            _ => assert!(false)
        }
    }
    #[test]
    fn process_guess_good3() {
        let mut game = create_test_game(1);
        game = match game.process(Guess(cfs("quote"))) {
            Step(state) => state,
            _ => {
                assert!(false);
                create_test_game(1)
            }
        };
        match game.process(Guess(cfs("quits"))) {
            GameOver(_state) => assert!(true),
            _ => assert!(false)
        }
    }
}
        
