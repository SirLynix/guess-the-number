use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut remaining_tries = 7;
    let mut guess = String::new();

    loop {
        let guess = get_number(remaining_tries, &mut guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Secret number is bigger!"),
            Ordering::Greater => println!("Secret number is lower!"),
            Ordering::Equal => {
                println!("Congratulations, you found the number!");
                break;
            }
        }

        remaining_tries = remaining_tries - 1;
        if remaining_tries <= 0 {
            println!("You failed, secret number was {}!", secret_number);
            return;
        }
    }
}

fn get_number(remaining_tries: i32, buffer: &mut String) -> i32 {
    loop {
        print!(
            "Guess the number ({} {} remaining): ",
            remaining_tries,
            if remaining_tries > 1 { "tries" } else { "try" }
        );
        io::stdout().flush().unwrap();

        buffer.clear();

        io::stdin().read_line(buffer).expect("Failed to read line");

        match buffer.trim().parse::<i32>() {
            Ok(t) => return t,
            Err(e) => println!("Failed to parse number: {}", e),
        };
    }
}
