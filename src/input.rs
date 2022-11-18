use std::io;

pub fn read_int() -> i32{
    let mut buffer = String::new();
    let stdin = io::stdin();
    let _input = stdin.read_line(&mut buffer).unwrap();
    let as_int: i32 = match buffer.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("invalid input, try again!");
            read_int()
        }
    };
    return as_int;
}