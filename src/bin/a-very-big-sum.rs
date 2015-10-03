use std::io;

fn main() {
    let _: i64 = from_stdin();
    let nums = read_numbers();
    let sum = nums.iter().fold(0, |a,b| a+b);
    println!("{}", sum)
}

fn from_stdin<T>() -> T where T : std::str::FromStr, T::Err : std::fmt::Debug {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    std::str::FromStr::from_str(buffer.trim()).unwrap()
}

fn read_numbers() -> Vec<i64> {
    let line: String = from_stdin();
    line.split(' ')
        .map(std::str::FromStr::from_str)
        .map(Result::unwrap)
        .collect()
}
