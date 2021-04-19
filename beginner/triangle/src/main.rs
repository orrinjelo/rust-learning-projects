use rustop::opts;

fn main() {
    let (args, _rest) = opts! {
        synopsis "Print a starry triangle.";
        opt skip:bool, desc:"Skip lines.";
        opt reverse:bool, desc:"Reverse the triangle";
        param num:u32, desc:"Max stars in large side of triangle.";
    }.parse_or_exit();

    for i in 1..args.num+1 {
        if args.skip && i % 2 == 0 {
            continue;
        }
        if args.reverse {
            for _ in 0..args.num-i+1 {
                print!("*");
            }
        } else {
            for _ in 0..i {
                print!("*");
            }
        }
        println!();
    }
}
