use ex07::strpcmp;
use ftkit::ARGS;

fn main() {
    if ARGS.len() != 3 {
        println!("exactly 2 args need to be provide");
    } else {
        println!("{}", strpcmp(ARGS[1].as_bytes(), ARGS[2].as_bytes()))
    }
}

