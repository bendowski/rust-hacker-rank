use std::io;

fn main() {
    let n = from_stdin();
    let mut diag1 = 0;
    let mut diag2 = 0;

    for i in 0..n {
        let row = read_numbers();
        diag1 += row[i];
        diag2 += row[n - i - 1]
    }

    println!("{}", (diag1 - diag2).abs())
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
