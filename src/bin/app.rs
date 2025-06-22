use rust_arch_demo::{add as add_numbers, multiply, say_hello};
use utils::print_message;

fn main() {
    let sum = add_numbers(5, 7);
    let product = multiply(3, 4);
    let greeting = say_hello("User");

    println!("Sum: {}", sum);
    println!("Product: {}", product);
    println!("{}", greeting);

    print_message("This is a message from the utils crate!");
}

