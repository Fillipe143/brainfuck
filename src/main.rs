use std::fs;

macro_rules! throw_error {
    ($($arg: tt)*) => {
        println!($($arg)*);
        std::process::exit(1);
    };
}

fn read_arg(args: &mut Vec<String>, msg: &str) -> String {
    if args.len() == 0 { throw_error!("Command line error: {}", msg); }
    args.pop().unwrap()
}

fn check_file_path(file_path: &str) {
    if !file_path.ends_with(".bf") { throw_error!("Invalid file extension"); }

    let file_metadata = fs::metadata(file_path);
    if file_metadata.is_err() { throw_error!("No such file '{}'", file_path); }

    let file_type = file_metadata.unwrap().file_type();
    if !file_type.is_file() { throw_error!("This path is not to a file"); }
}

fn main() {
    let mut args: Vec<String> = std::env::args().rev().collect();

    let _ = read_arg(&mut args, "Unable to read program arguments");
    let file_path = read_arg(&mut args, "File path not given");
    check_file_path(file_path.as_str());
}
