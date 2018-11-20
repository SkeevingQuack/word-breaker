use std::fmt;
use std::str;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Letter {
    value: u8
}
impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", str::from_utf8(&[self.value])
               .expect("Letter seems to not be a letter"))
    }
}
impl Letter {
    pub fn from_byte(input: u8) -> Self {
        if !input.is_ascii_lowercase() {
            panic!("{} not in ascii_lowercase", input);
        }
        Self { value: input }
    }
    pub fn get_alphabet_index(&self) -> usize {
        usize::from(self.value - 97)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Candidate {
    list: [Letter; 5]
}
impl Candidate {
    pub fn from_str(input: &str) -> Self {
        let word = input.as_bytes();
        if word.len() != 5 {
            panic!("Candidate input str not length 5");
        }
        Self {
            list: [Letter::from_byte(word[0]),
                   Letter::from_byte(word[1]),
                   Letter::from_byte(word[2]),
                   Letter::from_byte(word[3]),
                   Letter::from_byte(word[4])]
        }       
    }
    pub fn compare(&self, other: Candidate) -> u8 {
        let mut score = 0;
        let zipped = self.list.iter().zip(other.list.iter());
        for (this_letter, other_letter) in zipped {
            if this_letter == other_letter { score += 1 };
        }
        score
    }
}
impl fmt::Display for Candidate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //TODO: find a cleaner way
        write!(f, "{}{}{}{}{}",
               self.list[0],
               self.list[1],
               self.list[2],
               self.list[3],
               self.list[4])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn letter_byte_good1() {
        let letter = Letter::from_byte(0x68);
        assert_eq!(0x68u8, letter.value);
    }
    #[test]
    #[should_panic(expected = "not in ascii_lowercase")]
    fn letter_byte_bad1() {
        Letter::from_byte(0x43);
    }
    #[test]
    fn letter_display_good1() {
        let letter = Letter::from_byte(0x63);
        assert_eq!("c", format!("{}", letter));
    }

    #[test]
    fn letter_index_good1() {
        let letter = Letter::from_byte(0x61);
        assert_eq!(0, letter.get_alphabet_index());
    }
    
    #[test]
    fn candidate_str_good1() {
        let expected = Candidate {
            list: [Letter::from_byte(0x61),
                   Letter::from_byte(0x62),
                   Letter::from_byte(0x63),
                   Letter::from_byte(0x64),
                   Letter::from_byte(0x65)]
        };
        let test_value = Candidate::from_str("abcde");
        assert_eq!(expected, test_value);
    }
    #[test]
    #[should_panic(expected = "str not length")]
    fn candidate_str_bad1() {
        Candidate::from_str("asdf");
    }
    #[test]
    #[should_panic(expected = "str not length")]
    fn candidate_str_bad2() {
        Candidate::from_str("abcdef");
    }
    #[test]
    #[should_panic(expected = "str not length")]
    fn candidate_str_bad3() {
        Candidate::from_str("すみません");
    }

    #[test]
    fn candidate_display_good1() {
        let test_value = Candidate {
            list: [Letter::from_byte(0x61),
                   Letter::from_byte(0x62),
                   Letter::from_byte(0x63),
                   Letter::from_byte(0x64),
                   Letter::from_byte(0x65)]
        };
        assert_eq!("abcde", test_value.to_string());
    }
    #[test]
    fn candidate_display_good2() {
        let test_value = Candidate::from_str("zygot");
        assert_eq!("zygot", test_value.to_string());
    }
}
