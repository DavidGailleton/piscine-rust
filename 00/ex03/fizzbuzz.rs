fn main() {
    let mut i: u32 = 1;
    while i <= 100 {
        if i % 3 == 0 && i % 5 == 0 {
            std::println!("fizzbuzz");
        } else if i % 3 == 0 {
            std::println!("fizz");
        } else if i % 5 == 0 {
            std::println!("buzz");
        } else if i % 11 == 3 {
            std::println!("FIZZ");
        } else if i % 11 == 5 {
            std::println!("BUZZ");
        } else {
            std::println!("{}", i);
        }
        i += 1;
    }
}
