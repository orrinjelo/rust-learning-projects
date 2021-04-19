use rand::Rng;

fn first_choice() {
    println!("You have been given the first choice: lasagna.");
}

fn second_choice() {
    println!("You have been given the second choice: fruitcake.");
}

fn third_choice() {
    println!("You have been given the third choice: worms.");
}

fn main() {
    let mut rng = rand::thread_rng();

    let choice = rng.gen_range(1..4);

    if choice == 1 { first_choice(); }
    else if choice == 2 { second_choice(); }
    else { third_choice(); }
}
