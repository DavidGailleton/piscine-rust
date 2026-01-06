use ftkit::ARGS;
use ex07::strpcmp;

fn main() {
    if ARGS.len() == 3 {
        println!("{}", strpcmp(&ARGS[1].as_bytes(), &ARGS[2].as_bytes()));
    }
}
