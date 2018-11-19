//extern crate rand;

//mod cli;
//mod state_machine;
pub mod candidate;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_public1() {
        let expected = candidate::Letter::from_byte(0x69);
        assert_eq!("i", format!("{}", expected));
    }       
}
