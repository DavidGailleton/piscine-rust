use std::cmp::Ordering;

fn main() {
    let to_find: i32 = ftkit::random_number(0..100);
    let mut to_cmp: i32;
    println!("Me and my infinite wisdom have found an appropriate secret you shall yearn for.");
    loop {
        to_cmp = ftkit::read_number();
        match to_find.cmp(&to_cmp) {
            Ordering::Less => println!(
                "This student might not be as smart as I was told. This answer is obviously too weak."
            ),
            Ordering::Greater => {
                println!("Sometimes I wonder whether I should retire. I would have guessed higher.")
            }
            Ordering::Equal => break,
        }
    }
    println!(
        "That is right! The secret was indeed the number {}, which you have brilliantly discovered!",
        to_find
    )
}
