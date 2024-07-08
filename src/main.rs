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

fn read_file(file_path: &str) -> Vec<u8> {
    if !file_path.ends_with(".bf") { throw_error!("Invalid file extension"); }

    let file_metadata = fs::metadata(file_path);
    if file_metadata.is_err() { throw_error!("No such file '{}'", file_path); }

    let file_type = file_metadata.unwrap().file_type();
    if !file_type.is_file() { throw_error!("This path is not to a file"); }

    if let Ok(data) = fs::read(file_path) { data }
    else { throw_error!("Unable to read file '{}'", file_path); }
}

type Operator = (char, usize);

fn extract_operators(data: &mut Vec<u8>) -> Vec<Operator> {
    let mut operators = Vec::<Operator>::new();
    let mut open_brackets = Vec::<usize>::new();

    for byte in data.iter() {
        let curr_char = *byte as char;

        match curr_char {
            '<' | '>' | '+' | '-' | '.' | ',' => {
                if let Some(last_operator) = operators.last_mut() {
                    if last_operator.0 == curr_char { last_operator.1 += 1; }
                    else { operators.push((curr_char, 1)); }
                } else { operators.push((curr_char, 1)); }
            },
            '[' => {
                open_brackets.push(operators.len());
                operators.push((curr_char, 0));
            },
            ']' => {
                if open_brackets.is_empty() { throw_error!("Unbalanced brackets"); }
                let opening_index = open_brackets.pop().unwrap();

                operators[opening_index].1 = operators.len();
                operators.push((curr_char, opening_index));
            },
            _ => {}
        }
    }

    if !open_brackets.is_empty() { throw_error!("Unbalanced brackets"); }
    operators
}

fn main() {
    let mut args: Vec<String> = std::env::args().rev().collect();

    let _ = read_arg(&mut args, "Unable to read program arguments");
    let file_path = read_arg(&mut args, "File path not given");

    let mut data = read_file(file_path.as_str());
    let operators = extract_operators(&mut data);
    println!("{:?}", operators);
}
