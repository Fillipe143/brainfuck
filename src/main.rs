macro_rules! throw_error {
    ($($arg: tt)*) => {
        println!($($arg)*);
        std::process::exit(1);
    };
}

fn read_arg(args: &mut Vec<String>, msg: &str) -> String {
    if args.len() == 0 { throw_error!("cmd line error: {}", msg); }
    args.pop().unwrap()
}

fn main() {
    let mut args: Vec<String> = std::env::args().rev().collect();

    let _ = read_arg(&mut args, "unable to read program arguments");
    let _file_path = read_arg(&mut args, "file path not given");
}
