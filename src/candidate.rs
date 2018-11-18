pub struct Letter(u8)

impl Letter {
    fn from(character: &str) -> Letter {
        //TODO: Check length, check ASCII
    }
    fn get_alphabet_index(&self) -> i32 {
        //TODO: probably just ascii math
    }
}


pub struct Candidate(Letter, Letter, Letter, Letter, Letter)

impl Candidate {
    fn from(word: &str) -> Candidate {
        //TODO: Check length, check ASCII
    }
}
