use std::fmt;

#[derive(Debug)]
pub struct LetterError(&str)
impl fmt::Display for CastError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} can't be Letter", self.0)
    }
}

#[derive(Debug)]
pub struct Letter(u8)
impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Letter {
    pub fn from(input: &str) -> Result<Self, LetterError> {
        let characters = input.as_bytes();
        if characters.len() != 1 && 96 < characters.0 < 123 {
            Ok(Self{characters.0})
        } else {
            Err(LetterError(input))
        }     
    }
    fn get_alphabet_index(&self) -> i32 {
        i32::from(self.0 - 97)
    }
}

#[derive(Debug)]
struct Candidate(Letter, Letter, Letter, Letter, Letter)

impl Candidate {
    pub fn from(input: &str) -> Result<Self, LetterError> {
        let word = input.as_bytes();
        if word.len() != 5 {
            Err(LetterError(input))
        } else {
            for byte in word {
                
    }
}
