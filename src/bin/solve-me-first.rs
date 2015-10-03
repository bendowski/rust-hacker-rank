use std::io;

fn main() {
    let num1: i32 = from_stdin();
    let num2: i32 = from_stdin();
    println!("{}", num1 + num2);
}

fn from_stdin<T>() -> T where T : std::str::FromStr, T::Err : std::fmt::Debug {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    std::str::FromStr::from_str(buffer.trim()).unwrap()
}
