fn add_five(num:u32) -> u32 {
    num + 5
}

// Everthng in Rust is immutable by default
// To make a variable mutable, we must use the mut keyword

fn main() {
    // immutable variable
    let num = 10;
    println!("{} + 5 = {}", num, add_five(num));

    // mutable variable
    let mut x = 10;
    println!("x = {}", x);
    x = 20;
    println!("x = {}", x);
    
}
