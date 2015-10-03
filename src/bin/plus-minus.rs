use std::io;

fn main() {
    let all: f64 = from_stdin();
    let nums = read_numbers();
    let (pos, neg, zero) = nums.iter().fold((0,0,0), |(p,n,z), x| {
        match x.signum() {
            1 => (p+1, n, z),
            -1 => (p, n+1, z),
            0 => (p, n, z + 1),
            _ => unreachable!()
        }
    });
    println!("{:.3}", pos as f64 / all);
    println!("{:.3}", neg as f64 / all);
    println!("{:.3}", zero as f64 / all);
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
