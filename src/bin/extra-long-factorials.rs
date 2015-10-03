extern crate num;

use std::io;
use num::One;
use num::bigint::BigUint;
use num::traits::FromPrimitive;

fn main() {
    let n: u8 = from_stdin();
    let mut result: BigUint = One::one();

    for i in 2..n+1 {
        let factor: BigUint = FromPrimitive::from_u8(i).unwrap();
        result = result * factor
    }

    println!("{}", result)
}

fn from_stdin<T>() -> T where T : std::str::FromStr, T::Err : std::fmt::Debug {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    std::str::FromStr::from_str(buffer.trim()).unwrap()
}
