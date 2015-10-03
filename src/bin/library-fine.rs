use std::io;

#[derive(PartialEq, PartialOrd, Eq, Ord)]
struct Date {
    year: u64,
    month: u64,
    day: u64,
}

fn main() {
    let input = read_numbers();
    let actual = Date {year: input[2], month: input[1], day: input[0]};
    let input = read_numbers();
    let expected = Date {year: input[2], month: input[1], day: input[0]};

    let fine = if actual <= expected {
        0
    } else if (actual.year, actual.month) == (expected.year, expected.month) {
        15 * (actual.day - expected.day)
    }  else if actual.year == expected.year {
        500 * (actual.month - expected.month)
    } else {
        10_000
    };

    println!("{}", fine)
}

fn from_stdin<T>() -> T where T : std::str::FromStr, T::Err : std::fmt::Debug {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    std::str::FromStr::from_str(buffer.trim()).unwrap()
}

fn read_numbers() -> Vec<u64> {
    let line: String = from_stdin();
    line.split(' ')
        .map(std::str::FromStr::from_str)
        .map(Result::unwrap)
        .collect()
}
