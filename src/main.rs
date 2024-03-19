use std::io::{self, Read};

use rand::Rng;

fn main() {
    
    let mut rng = rand::thread_rng();

    let number: i32 = rng.gen_range(1..=100);

    while true {
        print!("Insert a number between 1 and 100: ");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
                            .expect("Fail to read input");

    }

}
