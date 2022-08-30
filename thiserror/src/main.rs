use std::error::{self as stdError, Error};
use std::num::ParseIntError;
use std::vec;
use thiserror::Error;

type Result<T> = std::result::Result<T, CustomError>;

#[derive(Debug, Error)]
enum CustomError {
    #[error("list not empty.")]
    EmptyVec,
    #[error("invalid list element.")]
    Parse(#[from] ParseIntError),
}

fn first_element(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(CustomError::EmptyVec)?;
    let element = first.parse::<i32>()?;
    Ok(element)
}

fn print(result: Result<i32>) {
    result.map_or_else(
        |e| {
            println!("Error: {}", e);
            if let Some(source) = e.source() {
                println!("Caused by: {}", source);
            }
        },
        |r| println!("The first element is {}", r),
    );
}

fn main() {
    let numbers = vec!["1", "2", "3"];
    let empty: Vec<&str> = vec![];
    let strings = vec!["string", "2", "3"];

    print(first_element(numbers));
    print(first_element(empty));
    print(first_element(strings));
}
