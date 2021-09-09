fn random_int() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let mut seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    seed ^= seed << 21;
    seed ^= seed >> 35;
    seed ^= seed << 4;
    seed
}

fn main() {
    println!("Hi! You'll need to guess the number from 1 to 100.");

    let guessed = random_int() % 100 + 1;
    let mut attempts = 0;

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: u64 = input.trim().parse().unwrap();
        attempts += 1;

        if input < guessed {
            println!("Your guess is way too low!");
        } else if input > guessed {
            println!("Your guess is way too high!");
        } else {
            println!("Correct! It's {}. It took you {} attempts to guess.", guessed, attempts);
            break;
        }
    }
}
