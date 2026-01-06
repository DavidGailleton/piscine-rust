use std::cmp::Ordering;

fn main() {
    let to_guess: i32 = ftkit::random_number(0..50);

    println!("Me and my infinite wisdom have found an appropriate secret you shall yearn for.");
    loop {
        let input: i32 = ftkit::read_number();
        match to_guess.cmp(&input) {
            Ordering::Greater => println!("This student might not be as smart as I was told. This answer is obviously too weak."),
            Ordering::Less => println!("Sometimes I wonder whether I should retire. I would have guessed higher."),
            Ordering::Equal => break ,
        }
    }
    println!("That is right! The secret was indeed the number 19, which you have brilliantly discovered!");
}
