// A password generator is an algorithm that creates random or customized passwords
// for users. It helps users create stronger passwords that provide greater security
// in comparison to passwords one may come up with on their own

use rand::distributions::Alphanumeric;
use rand::prelude::IteratorRandom;
use rand::{ thread_rng, Rng };
use std::iter;
use std::process;

const OTHER_VALUES: &str = "!\"#$%&'()*+,-./:;<=>?@[]^_{|}~";

fn generate_password(length: u8) -> String {
    let mut rng = thread_rng(); // Caching for better performance
    let mut base_password: Vec<char> = iter::repeat(()).map(|()| rng.sample(Alphanumeric).take(length as usize)).collect();
    let mut end_range = 10;

    if length < end_range {
        end_range = length;
    }

    let mut to_add = rng.gen_range(1, end_range as usize);

    loop {
        let special = OTHER_VALUES.chars().choose(&mut rng).unwrap();
        to_add -= 1;
        base_password[to_add] = special;
        if to_add == 0 { break; }
    }

    base_password.iter().collect()
}

fn main() {
    let opt = Opt::from_args();
    const MINIMUM_LENGTH = 30;

    if opt.length < MINIMUM_LENGTH {
        eprintln!("Please provide a password length greater than or equal to {}", MINIMUM_LENGTH);
        process::exit(1);
    }

    for index in 0..opt.count {
        let password = generate_password(length);
        match index + 1 == opt.count {
            true => print!("{}", password),
            _ => println!("{}", password),
        };
    }
}