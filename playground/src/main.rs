mod my_functions;

// If we want to use all the functions in my_functions module
// use crate::my_functions::*;

// Everthng in Rust is immutable by default
// To make a variable mutable, we must use the mut keyword

fn main() {
    // immutable variable
    let num: u32 = 10;
    println!("{} + 5 = {}", num, my_functions::add_five(num));

    // mutable variable
    let mut x: i32 = 10;
    println!("x = {}", x);
    my_functions::display_address(&x);
    x = 20;
    println!("x = {}", x);    
}
