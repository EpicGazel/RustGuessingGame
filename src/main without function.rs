use std::io;
use rand::Rng;
use std::cmp::Ordering;
//use std::io::Write;
//use std::{thread, time};

fn main() {
    let range_min:u128 = 0;
    let mut token = String::new();

    //Below code curtosey of https://www.reddit.com/r/rust/comments/obnlv8/some_neat_rust_syntax_loop_break_match/
    let range_max:u128 = loop {
        println!("Input range_max number:");

        token.clear();
        io::stdin()
            .read_line(&mut token)
            .expect("Failed to read line");

        break match token.trim().parse() {
            Ok(number) if number > range_min => number,
            Ok(_) => {
                println!("Number too small.");
                continue;
            }
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
            }
        };

    let secret_number = rand::thread_rng().gen_range(range_min..range_max);

    println!("Secret number is {}", secret_number);

    //let sleep_time = time::Duration::from_millis(500);
    //let now = time::Instant::now();

    let mut guess_min = range_min;
    let mut guess_max = range_max;
    let mut guess;

    let mut num_guesses = 1;
    loop {
        guess = ((guess_max - guess_min) / 2) + guess_min;
        println!("Min: {}, Max: {}, Guess: {}", guess_min, guess_max, guess);
        //println!("Guessing: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small");
                guess_min = guess;
            },
            Ordering::Greater => {
                println!("Too big");
                guess_max = guess;
            },
            Ordering::Equal => {
                println!("Just Right! The soltuion was {}! It took {} guesses.", secret_number, num_guesses);
                break;
            },
        }

        num_guesses += 1;

        //thread::sleep(sleep_time);
        //assert!(now.elapsed() >= sleep_time);
    }

}
