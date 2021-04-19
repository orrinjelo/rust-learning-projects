use text_io::read;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let choice = rng.gen_range(1..101);

    println!("I'm thinking of a number between 1 and 100...");

    let mut i = 0;

    while i != choice {
        i = read!();

        if i > choice {
            println!("Lower!");
        } else if i < choice {
            println!("Higher!");
        } else {
            println!("That's it!");
        }
    }

}
