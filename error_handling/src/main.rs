use std::error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
struct NegativeNumberError;

impl Display for NegativeNumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Negative Error.")
    }
}

impl error::Error for NegativeNumberError {}

#[derive(Debug)]
struct ZeroDivideError;

impl Display for ZeroDivideError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ZeroDivide Error.")
    }
}

impl error::Error for ZeroDivideError {}

fn divide(a: i32, b: i32) -> Result<u32, Box<dyn error::Error>> {
    if a < 0 || b < 0 {
        return Err(NegativeNumberError.into());
    }
    if b == 0 {
        return Err(ZeroDivideError.into());
    }
    Ok((a / b) as u32)
}

fn main() {
    let a = -1;
    let b = 10;
    divide(a, b)
        .map(|n| println!("{}", n))
        .map_err(|e| println!("{:?}", e));
    let a = 1;
    let b = 10;
    divide(a, b)
        .map(|n| println!("{}", n))
        .map_err(|e| println!("{:?}", e));
    let a = 1;
    let b = 0;
    divide(a, b)
        .map(|n| println!("{}", n))
        .map_err(|e| println!("{:?}", e));
}
