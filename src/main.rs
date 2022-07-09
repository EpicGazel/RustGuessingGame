use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::io::Write;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..101);

    //println!("Secret number is {}", secret_number);

    loop {
        print!("Guess a number: ");
        io::stdout().flush().unwrap();
        
        let mut guess = String::new();
    
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            },
        };
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Just Right!");
                break;
            },
        }
    }

}
