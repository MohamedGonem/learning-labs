//! This is simple guessing game with the following:
//! - replaying with tracked stats
//! - guessing a number between `0..=u16::MAX`
//!
//! 2026 (c) Mohamed Gonem
fn main() {
    println!("welcome to the humble guessing game");

    // we initialize the game stats
    // used for multiple games
    let mut streak = 0;
    let mut game_count = 1;
    let mut lose_counter = 0;
    let mut win_counter = 0;
    let mut highest_streak = 0;

    'game_loop: loop {
        // first we initialize the secret number of this round
        let target_number = rand::random::<u16>();
        // ...and how many times user can enter
        const TRIES: u8 = 5;
        const CORRECTION_TRIES: u8 = 3;
        // and create variable if the user guessed it or not
        let mut guessed = false;

        // we start the round logic by iterating [1, ..., TRIES]
        for round in 1..=TRIES {
            println!("enter your guess (0 - 65535)");

            // we loop `CORRECTION_TRIES` times for getting correct user guess/input
            // so game doesn't exit immedietly
            let user_guess = 'guess_input: {
                for attempt in 1..=CORRECTION_TRIES {
                    // we initialize empty `String` into `guess`
                    let mut guess = String::new();
                    // the following constructs stdin
                    std::io::stdin()
                        // then reads user input and put it in `guess` variable
                        .read_line(&mut guess)
                        // there is two ways to handle the `Result` from `read_line`
                        // use of `expect()` and `unwrap`-like functions
                        .unwrap_or_else(
                            // this closure made to pass the `Error`
                            |e| {
                                // into `eprintln!` macro which will print to stderr
                                eprintln!("error: {e:?}");
                                // then exit the program with exit code '1'
                                std::process::exit(1);
                            },
                        );

                    // now to compare the user guess to our random secret number...
                    match guess
                        // ...we start by trimming the extra whitespace using `trim()`...
                        .trim()
                        // ...then we need to 'parse' (convert types) the `String` to `u16`
                        .parse::<u16>()
                    {
                        // if the user guess got parsed correctly into `u16`
                        // then break the loop of input
                        Ok(num) => break 'guess_input num,
                        // if not then for `CORRECTION_TRIES` times warn the user
                        // then exit with exit code '2'
                        _ => {
                            if attempt == CORRECTION_TRIES {
                                eprintln!("exiting after multiple failed attempts");
                                std::process::exit(2);
                            }

                            eprintln!("your guess is not a valid number");
                            eprintln!("what considered valid are numbers between 0 and 65535");
                        }
                    }
                }
                // this is unreachable part due previous `match` block handles them in the loop,
                // but this is needed due `exit` returns `()`
                unreachable!();
            };

            // if all good print the user guess
            print!("you guessed: {user_guess}");

            // then we compare if it matches the secret number
            match user_guess.cmp(&target_number) {
                // we see if it is less...
                std::cmp::Ordering::Less => {
                    println!(" ...but it less than the secret number");
                    println!("try again you got {} tries left", TRIES - round);
                }
                // ...or is the secret number itself...
                std::cmp::Ordering::Equal => {
                    println!(" ...and it is the secret number!");
                    println!("you guessed it in {} tries", round);

                    guessed = true;
                    // we break the game loop
                    break;
                }
                // ...or greater than it
                std::cmp::Ordering::Greater => {
                    println!(" ...but it greater than the secret number");
                    println!("try again you got {} tries left", TRIES - round);
                }
            }
        }

        // if the loop finished and the user didn't guess
        if !guessed {
            println!("the number was {target_number}");
            lose_counter += 1;

            // backup if needed
            if streak > highest_streak {
                highest_streak = streak;
            }
            streak = 0;
        } else {
            win_counter += 1;
            streak += 1;

            // we set the `highest_streak` to `streak` if needed
            if streak > highest_streak {
                highest_streak = streak;
            }
        }

        println!("Play again? (y/n)");
        let mut continue_choice = String::new();
        // we again construct `stdin`
        std::io::stdin()
            // then read user input
            .read_line(&mut continue_choice)
            // then handle the `Result`
            .unwrap_or_else(
                // this closure made to pass the `Error`
                |e| {
                    // into `eprintln!` macro which will print to stderr
                    eprintln!("error: {e:?}");
                    // then exit the program with exit code '3'
                    std::process::exit(3);
                },
            );

        // we check if the user wants to continue or not
        // by checking the first letter of the input
        // if it is 'y' then continue the game otherwise exit the loop
        if continue_choice.trim().to_lowercase().starts_with('y') {
            game_count += 1;
            continue 'game_loop;
        } else {
            break 'game_loop;
        }
    }

    println!(
        "thanks for playing!\n
    for total of {game_count} games you got
    total wins: {win_counter}
    total loses: {lose_counter}
    highest win streak: {highest_streak}"
    );
}
