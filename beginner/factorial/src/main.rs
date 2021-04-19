use factorial::Factorial;
use rustop::opts;
use std::process;


fn main() {
    let (args, _rest) = opts! {
        synopsis "Computes the factorial of a number.";
        param number:u64, desc:"N!";
    }.parse_or_exit();

    process::exit(
        match args.number.checked_factorial() {
            Some(x) => {
                println!("{}", x);
                0
            },
            None => {
                println!("Invalid input, or input number too large.");
                1
            }
        }
    );
}
