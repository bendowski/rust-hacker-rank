use std::io;

fn main() {
    let t: i64 = from_stdin();
    for _ in 0..t {
        let nums = read_numbers();
        println!("{}", nums[0] + nums[1])
    }
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
