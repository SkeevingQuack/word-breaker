use ::candidate::{Letter, Candidate};
use ::wordlist::Wordlist;

use std::io;

pub fn play() {
    let mut list = Wordlist::new();
    println!("h for help");
    
    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command)
            .expect("Failed to read line");
        let mut command: Vec<&str> = command.as_str().split(" ").collect();
        match command.remove(0).trim() {
            "n" => {
                println!("{}", list.get_word().expect("list empty"));
            }
            "i" => {
                for arg in command {
                    let arg = arg.trim();
                    if arg.len() > 1 { panic!("'{}' only 1 char", arg); }
                    list.include(arg);
                }
                println!("{}", list.get_word().expect("list empty"));
            }
            "e" => {
                for arg in command {
                    let arg = arg.trim();
                    if arg.len() > 1 { panic!("'{}' only 1 char", arg); }       
                    list.exclude(arg);
                }
                println!("{}", list.get_word().expect("list empty"));
            }
            "r" => {
                list.reset_list();
                println!("{}", list.get_word().expect("list empty"));
            }
            "h" => {
                println!("n: get New word without changing list");
                println!("i: limit list to words that Include letters");
                println!("e: limit list to words that Exclude letters");
                println!("r: Reset list");
                println!("q: Quit");
            }
            "q" => {
                return;
            }
            _ => {
                println!("Command not understood: nierq");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;


}
