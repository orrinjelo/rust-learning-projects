use rustop::opts;

fn main() {
    let (args, _rest) = opts! {
        synopsis "Generate a fibonacci sequence until N.";
        param n:u32, desc:"Max N number.";
    }.parse_or_exit();

    let mut a = 1;
    let mut b = 1;
    let mut c = 2;

    while a < args.n {
        print!("{} ", a);
        a = b;
        b = c;
        c = a + b;
    }
    println!();
}
