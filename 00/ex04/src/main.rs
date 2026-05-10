fn main() {
    println!("Hello, Cargo!");
    #[cfg(not(debug_assertions))]
    println!("I'm in release mode!");
}
