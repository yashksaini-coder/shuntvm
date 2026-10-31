use std::io;

fn main() {
    let mut input = String::new();
    // Create a new mutable string for user input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input"); // Read input
    println!("{}", input);
}
