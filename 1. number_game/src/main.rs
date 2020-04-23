extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::io::Read;
use rand::distributions::Alphanumeric;

fn main() {

    let random = rand::thread_rng().sample_iter(&Alphanumeric).next().unwrap();

    loop {

        let mut input:[u8; 1] = [0; 1];

        io::stdin().read(&mut input).unwrap();

        let input = input[0] as char;

        if !input.is_alphanumeric() {
            continue;
        }

        match random.cmp(&input) {
            Ordering::Less => println!("low"),
            Ordering::Greater => println!("high"),
            Ordering::Equal => {
                println!("Good job");
                break;
            }
        }

        println!("random is {:?}", random);
        println!("input is {:?}", input);
    }

}
