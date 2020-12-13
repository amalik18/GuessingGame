// fn main() {
//     println!("Hello, world!");
//     playing_with_nums();
// }
//
// fn playing_with_nums() {
//     let x = 12; //by default rust resorts to i32
//     let y: u8 = 12; // strictly defining type
//     let z = 12u8; //can also be written let z = 12_u8
//     println!("x = {}, y = {}, z = {}", x, y, z);
//     let a: i8 = -12;
//     println!("A negative int {}", a);
//
//     // Arithmetic
//     // println!("{}", x + y); //error differenty types x(i32) y(u8)  ^ expected `i32`, found `u8`
//     println!("{}", z + y); // this works;
// }

use std::io;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

fn main() {
    println!("---------- Guess the Number ----------");
    let secret = generate_secret_number();

    // Allow for multiple guesses
    loop {
        // Convert the string returned by get_input to a number, and run match to handle errors
        let input_val: u32 = match get_input().trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your guess is: {}\n", input_val);

        match input_val.cmp(&secret) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You win!!!!\n");
                println!("You guessed {} correctly!", secret);
                break;
            },
        }
    }
}

/// Generate a random number using rand::thread_rng
/// thread_rng: Retrieve the lazily-initialized thread-local random number generator, seeded by the system.
///
/// # Return
/// an u32 int between 1 (inclusive) - 101 (exclusive)
fn generate_secret_number() -> u32 {
    let secret_number = thread_rng().gen_range(1, 101);
    return secret_number;
}


/// Retrieve user input
/// Grabs the user input as a String and return it as a u32 int
///
/// # Return
/// an u32 int of the user input.
fn get_input() -> String {
    println!("Please input a number.");

    let mut input_num = String::new();
    //create a new string variable (empty string) and bind it to input_num
    // mut allows the variable to change, by default all variables are immutable


    io::stdin()
        .read_line(&mut input_num)
        .expect("Failed to read line");
    //.read_line returns a value of the type Result, an enum, has values Ok and Err, need to handle in case of Err

    return input_num;
}