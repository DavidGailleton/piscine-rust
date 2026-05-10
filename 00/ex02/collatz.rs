fn collatz(start: u32) {
    let mut n: u32 = start;
    std::println!("{}", n);
    while n != 1 {
        match n % 2 {
            0 => n /= 2,
            1 => n = n * 3 + 1,
            _ => n += 1,
        }
        std::println!("{}", n);
    }
}

fn main() {
    collatz(3)
}
