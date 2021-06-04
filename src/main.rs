use std::io;
use std::fmt; // String formatting
use io::Write; // flushing to show text
use rand::Rng;

fn trim_newline(s: &mut String) -> String {
    if s.ends_with('\n') || s.ends_with('\r') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
    return s.to_string();
}


fn input(prefix: &str) -> String {
    print!("{}", prefix.to_string());
    io::stdout()
        .flush()
        .expect("Failed to flush");

    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Failed to read line from stdin");

    return trim_newline(&mut inp);
}

fn main() {
    // const
    let min = 1;
    let max = 100;

    // Get number
    let number = rand::thread_rng().gen_range(min..max+1);
    let mut guesses = 0;

    let mut guess_num = 0;

    println!("Guess a number between {} and {}", min, max);

    while guess_num != number {
        // Get user input
        let guess = input(&format!("Guess {}: ", (guesses + 1).to_string()));
        guesses += 1;

        match guess.parse::<i32>() {
            Ok(val) => {guess_num = val;},
            Err(err) => println!("Error {}", err),
        };

        if guess_num > number {
            println!("Too high, guess lower");
        } else if guess_num < number {
            println!("Too low, guess higher");
        }
    }

    println!("You guessed correctly in {} guesses!", guesses.to_string());

}
