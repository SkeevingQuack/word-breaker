use std::fmt;

#[derive(Debug)]
pub struct Letter {
    value: u8
}
impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl Letter {
    pub fn from_byte(input: u8) -> Self {
        if !input.is_ascii_lowercase() {
            panic!("{} not in ascii_lowercase", input);
        }
        Letter { value: input }
    }
    fn get_alphabet_index(&self) -> i32 {
        i32::from(self.0 - 97)
    }
}

#[derive(Debug)]
pub struct Candidate {
    list: [Letter; 5]
}
impl Candidate {
    pub fn from_str(input: &str) -> Self {
        let word = input.as_bytes();
        if word.len() != 5 {
            panic!("Candidate input str too long");
        }
        Self {
            [Letter::from_byte(word[0]),
             Letter::from_byte(word[1]),
             Letter::from_byte(word[2]),
             Letter::from_byte(word[3]),
             Letter::from_byte(word[4])]
        }       
    }
}
impl fmt::Display for Candidate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //TODO: find a cleaner way
        write!(f, "{}{}{}{}{}", self[0], self[1], self[2], self[3], self[4]);
    }
}
