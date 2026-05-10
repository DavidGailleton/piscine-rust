fn min(a: i32, b: i32) -> i32 {
    if a > b {
        b
    } else {
        a
    }
}

fn main() {
    std::println!("10 vs 20");
    std::println!("{}", min(10, 20));
    std::println!("20 vs 20");
    std::println!("{}", min(20, 20));
}
