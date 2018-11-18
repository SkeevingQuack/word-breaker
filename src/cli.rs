use std::io::stdin;
use ::state_machine;

pub fn main_loop() {
    let secret = state_machine::choose_secret();

    loop {
        println!("Please input your guess.");
        //following line can't be outside loop. Why?
        let mut guess = String::new();
        stdin().read_line(&mut guess)
            .expect("Failed to read line");

        println!("Is {} the same as {}?", secret.0, guess);
    }

}
