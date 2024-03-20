use std::io::{self, Write};

use rand::Rng;

fn main() -> io::Result<()>{
    
    let mut rng = rand::thread_rng();

    let number: i32 = rng.gen_range(1..=100);

    let mut count_tries: i32 = 0;
    while number >= 0 {
        print!("Insert a number between 1 and 100: ");
        io::stdout().flush()?;
        let mut guess = String::new();

        count_tries += 1;
        io::stdin().read_line(&mut guess)
                            .expect("Fail to read input");


        match guess.trim().parse::<i32>() {
            Ok(choice) => {
                if choice < 0 || choice > 100 {
                    println!("The number must be between 0 and 100! Try again.")
                }
                else if choice > number {
                    println!("The number is smaller. Try again!");
                }
                else if choice < number {
                    println!("The number is greater. Try again!");
                }
                else {
                    println!("Congratulations, you got it in {count_tries} tries!");
                    
                    break;
                }
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid number.\n");
                continue;
            }
        }
    }
    
    Ok(())
}