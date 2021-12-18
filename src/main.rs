use rand::seq::SliceRandom;
use std::io;

fn main() {
    let mut choice_list: Vec<String> = Vec::new();

    loop {
        println!("\nAdd your choice to the list!");
        let mut new_choice_input = String::new();

        io::stdin()
            .read_line(&mut new_choice_input)
            .expect("Failed to read input");

        let new_choice: String = match new_choice_input.trim().parse() {
            Ok(choice) => choice,
            Err(_) => continue,
        };

        if !new_choice.is_empty() {
            choice_list.push(new_choice);
        }

        if !choice_list.is_empty() {
            println!("\nYour choices: {:#?}", choice_list);
        }

        println!("\nDo you want to add another entry? y/n");

        let mut user_reply = String::new();

        io::stdin()
            .read_line(&mut user_reply)
            .expect("Failed to read input");

        let parsed_user_reply = user_reply.trim().parse::<char>();

        match parsed_user_reply {
            Ok(reply) => {
                let reply = reply.to_ascii_lowercase();
                match reply {
                    'y' => continue,
                    'n' => break,
                    _ => println!("Invalid input! Type y or n!"),
                };
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    let random_choice = choice_list
        .choose(&mut rand::thread_rng())
        .expect("Could not get random element from choices vec!");

    println!("\n\nYour random choice is: {}", random_choice);
}
