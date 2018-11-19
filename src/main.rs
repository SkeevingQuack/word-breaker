extern crate rand;

//mod cli;
//mod state_machine;
mod candidate;

fn main() {
    let c = candidate::Letter::from_byte(100u8);

    println!("{}", c);
}
