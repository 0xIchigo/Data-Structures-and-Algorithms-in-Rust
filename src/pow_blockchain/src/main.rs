#[macro_use]
extern crate serde_derive;

use std::io;
use std::process;
use std::io::{ Write };

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("Input a miner address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_addr);
    print!("Input a difficulty: ");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);

    match difficulty.trim().parse::<u32>() {
        Ok(_) => {
            println!("Generating genesis block...");
        },
        Err(_) => {
            println!("Invalid input! Difficulty must be a whole number");
            println!("Failed to generate genesis block...");
            println!("Exiting...");
            process::exit(0);
        }
    }

    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), difficulty.trim().parse::<u32>().unwrap());

    loop {
        println!("~~~");
        println!("Menu: ");
        println!("1. New transaction");
        println!("2. Mine block");
        println!("3. Change difficulty");
        println!("4. Change reward");
        println!("0. Exit");
        println!("~~~");

        print!("Please enter a number: ");
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);

        match choice.trim().parse::<i32>() {
            Ok(choice) => {
                match choice {
                    0 => {
                        print!("Exiting...");
                        process::exit(0);
                    },
                    1 => {
                        let mut sender = String::new();
                        let mut receiver = String::new();
                        let mut amount = String::new();
        
                        print!("Enter sender address: ");
                        io::stdout().flush();
                        io::stdin().read_line(&mut sender).expect("Error reading user input");
                        print!("Enter receiver address: ");
                        io::stdout().flush();
                        io::stdin().read_line(&mut receiver).expect("Error reading user input");
                        print!("Enter amount: ");
                        io::stdout().flush();
                        io::stdin().read_line(&mut amount).expect("Error reading user input");
        
                        match amount.trim().parse::<f32>() {
                            Ok(amount) => {
                                let res = chain.new_transaction(sender.trim().to_string(), receiver.trim().to_string(), amount);
        
                                match res {
                                    true => println!("Transaction added"),
                                    false => println!("Transaction failed")
                                }
                            },
                            Err(_) => println!("Invalid! Please enter a valid, numeric amount")
                        }
                    },
                    2 => {
                        print!("Generating block...");
                        let res = chain.generate_new_block();
                        match res {
                            true => println!("Block generated successfully"),
                            false => println!("Block generation failed"),
                        }
                    },
                    3 => {
                        let mut new_diff = String::new();
                        print!("Please enter new difficulty: ");
                        io::stdout().flush();
                        io::stdin().read_line(&mut new_diff);
                        
                        let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                        match res {
                            true => println!("Updated difficulty"),
                            false => println!("Failed update"),
                        }
                        println!("~~~");
                    },
                    4 => {
                        let mut new_reward = String::new();
                        println!("Please enter new reward: "); 
                        io::stdout().flush();
                        io::stdin().read_line(&mut new_reward);
                        
                        let res = chain.update_reward(new_reward.trim().parse().unwrap());
                        match res {
                            true => println!("Updated reward"),
                            false => println!("Failed update"),
                        }
                        println!("~~~");
                    },
                    _ => print!("Invalid choice. Please try again"),
                }
            },
            Err(_) => {
                println!("Invalid input! Please enter a numeric value from 0 to 4");
            }
        };
    }
}
