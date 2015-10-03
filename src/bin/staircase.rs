use std::io;
use std::iter::repeat;

fn main() {
    let n: usize = from_stdin();
    for i in 1..n+1 {
        let spaces = repeat(' ').take(n - i);
        let hashes = repeat('#').take(i);
        let line: String = spaces.chain(hashes).collect();
        println!("{}", line)
    }
}

fn from_stdin<T>() -> T where T : std::str::FromStr, T::Err : std::fmt::Debug {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    std::str::FromStr::from_str(buffer.trim()).unwrap()
}
