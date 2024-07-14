use std::{char, fs, io::Read};

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

#[derive(Debug)]
enum Op {
    GoBack(usize),
    GoForward(usize),
    Add(usize),
    Sub(usize),
    Write(usize),
    Read(usize),
    OpenBracket(usize),
    CloseBracket(usize),
}

impl Op {
    fn from_byte(c: u8, value: usize) -> Option<Op> {
        match c {
            b'<' => Some(Op::GoBack(value)),
            b'>' => Some(Op::GoForward(value)),
            b'+' => Some(Op::Add(value)),
            b'-' => Some(Op::Sub(value)),
            b'.' => Some(Op::Write(value)),
            b',' => Some(Op::Read(value)),
            b'[' => Some(Op::OpenBracket(value)),
            b']' => Some(Op::CloseBracket(value)),
            _ => None
        }
    }
}

fn extract_operators(data: &mut Vec<u8>) -> Vec<Op> {
    let mut operators = Vec::<Op>::new();
    let mut open_brackets = Vec::<usize>::new();

    let mut bytes_iter = data.iter().peekable();
    while bytes_iter.size_hint().0 != 0 {
        let curr_byte = *bytes_iter.next().unwrap();

        match curr_byte {
            b'<' | b'>' | b'+' | b'-' | b'.' | b',' => {
                let mut op_counter = 1;
                while bytes_iter.size_hint().0 != 0 && **bytes_iter.peek().unwrap() == curr_byte {
                    bytes_iter.next().iter();
                    op_counter += 1;
                }
                operators.push(Op::from_byte(curr_byte, op_counter).unwrap());
            },
            b'[' => {
                open_brackets.push(operators.len());
                operators.push(Op::OpenBracket(0))
            },
            b']' => {
                if open_brackets.is_empty() { throw_error!("Unbalanced brackets"); }
                let opening_index = open_brackets.pop().unwrap();

                operators[opening_index] = Op::OpenBracket(operators.len());
                operators.push(Op::CloseBracket(opening_index));
            },
            _ => {}
        }
    }

    if !open_brackets.is_empty() { throw_error!("Unbalanced brackets"); }
    operators
}

fn execute_program(operators: &Vec<Op>) {
    let mut cells = Vec::<i32>::new();
    let mut cell_offset = 0;
    cells.push(0);

    let mut i = 0;
    while i < operators.len() {
        let curr_operator = operators.get(i).unwrap();

        match curr_operator {
            Op::GoBack(counter) => {
                if cell_offset == 0 { throw_error!("Range error"); }
                cell_offset -= counter;
            },
            Op::GoForward(counter) => {
                cell_offset += counter;
                while cell_offset >= cells.len() { cells.push(0); }
            },
            Op::Add(counter) => { cells[cell_offset] += *counter as i32; },
            Op::Sub(counter) => { cells[cell_offset] -= *counter as i32; },
            Op::Write(counter) => for _ in 0..*counter {
                if let Some(c) = char::from_u32(cells[cell_offset] as u32) {
                    print!("{c}")
                }
            },
            Op::Read(counter) => {
                let mut byte = [0_u8];
                for _ in 0..*counter {
                    if let Err(_) = std::io::stdin().read_exact(&mut byte) {
                        throw_error!("Unexpected error when reading character");
                    }
                    cells[cell_offset] = byte[0] as i32;
                }
            },
            Op::OpenBracket(pair) => if cells[cell_offset] == 0 { i = *pair; },
            Op::CloseBracket(pair) => if cells[cell_offset] != 0 { i = *pair; },
        }
        i += 1;
    }
}

fn main() {
    let mut args: Vec<String> = std::env::args().rev().collect();

    let _ = read_arg(&mut args, "Unable to read program arguments");
    let file_path = read_arg(&mut args, "File path not given");

    let mut data = read_file(file_path.as_str());
    let operators = extract_operators(&mut data);
    execute_program(&operators);
}
