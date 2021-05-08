use rand::seq::SliceRandom;
use std::io;

fn main() {
    let mut choices: Vec<String> = Vec::new();

    loop {
        println!("\nAdd your choice to the list!");
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let choice: String = match choice.trim().parse() {
            Ok(choice) => choice,
            Err(_) => continue,
        };

        if !choice.is_empty() {
            choices.push(choice);
        }

        if !choices.is_empty() {
            println!("\nYour choices: {:#?}", choices);
        }

        println!("\nDo you want to add another entry? y/n");

        let mut reply = String::new();

        io::stdin()
            .read_line(&mut reply)
            .expect("Failed to read input");

        let parsed_reply = reply.trim().parse::<char>();

        match parsed_reply {
            Ok(reply) => {
                match reply {
                    'y' => continue,
                    'n' => break,
                    _ => println!("Invalid input! Type y or n!"),
                };
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    let random_choice = choices
        .choose(&mut rand::thread_rng())
        .expect("Could not get random element from choices vec!");

    println!("\n\nYour random choice is: {}", random_choice);
}
