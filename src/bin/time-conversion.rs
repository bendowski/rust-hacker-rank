use std::io;

fn main() {
    let input: String = from_stdin();
    let hour12: u8 = input[0..2].parse().unwrap();
    let minute: u8 = input[3..5].parse().unwrap();
    let second: u8 = input[6..8].parse().unwrap();
    let period: &str = &input[8..10];

    let hour24 = match (period, hour12) {
        ("AM", 12) => 0,
        ("AM", _) => hour12,
        ("PM", 12) => 12,
        ("PM", _) => hour12 + 12,
        _ => unreachable!()
    };

    println!("{:02}:{:02}:{:02}", hour24, minute, second)
}

fn from_stdin<T>() -> T where T : std::str::FromStr, T::Err : std::fmt::Debug {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse().unwrap()
}
