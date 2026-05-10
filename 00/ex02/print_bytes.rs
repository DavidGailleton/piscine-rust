fn print_bytes(s: &str) {
    for c in s {
        std::println!("{}", c);
    }
}

fn main() {
    print_bytes("Déjà Vu\n")
}
