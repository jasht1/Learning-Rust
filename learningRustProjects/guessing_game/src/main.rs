use rand::Rng;
use std::io;

fn get_guess() -> i32 {
    loop {
        println!("Make a guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        match guess.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => println!("Well it's defenitly not that"),
        }
    }
}
fn check_guess(target: i32, guess: i32) -> bool {
    if guess == target {
        println!("I don't want to play anymore");
        true
    } else {
        println!("Ha rubbish, your like {} off!", target - guess);
        false
    }
}

// as before the main function is declared, which contains all the code we expect to run at run time
fn main() {
    println!("Welcome to The Guessing Game!\n\n   I (the game) will generate a random number between 1 and and a billion,\nthats a one with 9 zeros mind you not 12, to win the game you must guess this number.\n\n Ready\n\n Set!\n\n GO!\n");
    let target = rand::thread_rng().gen_range(1..=1000000000);
    loop {
        let guess = get_guess();
        if check_guess(target, guess) {
            break;
        }
    }
}
