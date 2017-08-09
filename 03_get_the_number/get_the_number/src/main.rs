extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // you can write let here beacuse rust is able to get the type on compile time <3
    let secret = rand::thread_rng().gen_range(1, 101);
    println!("secret: {}", secret);


    // mut = mutable. props are immutable by default <3>
    let mut input = String::new();
    println!("Type a number:");

    io::stdin().read_line(&mut input)
        .ok()
        .expect("Error while reading the input");

    // we don't want to cast here so we try to parse the input. on sucess, the new input var
    // will shadow the old one but with a new type = u32. sweeet
    let input: u32 = input.trim().parse()
        .ok()
        .expect("please type a number!");

    println!("input: {}", input);


    match input.cmp(&secret) {
        Ordering::Less => println!("to small"),
        Ordering::Greater => println!("to big"),
        Ordering::Equal => println!("bam!")
    }
}
