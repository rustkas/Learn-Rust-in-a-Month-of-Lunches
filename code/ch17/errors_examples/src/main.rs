#![allow(dead_code)]

#[allow(unused_must_use)]
pub fn ex01() {
    use std::io::{Error, ErrorKind};
    fn parse_then_send(input: &[u8]) {
        let some_str = std::str::from_utf8(input).unwrap();
        let number = some_str.parse::<i32>().unwrap();
        send_number(number);
    }
    fn send_number(number: i32) -> Result<(), Error> {
        if number < 1_000_000 {
            println!("Number sent!");
            Ok(())
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Invalid data"))
        }
    }

    println!("{:?}", parse_then_send(b"nine"));
    println!("{:?}", parse_then_send(b"10"));
}

#[allow(unused_must_use)]
pub fn ex02() {
    use anyhow::{anyhow, Error};

    fn parse_then_send(input: &[u8]) -> Result<(), Error> {
        let some_str = std::str::from_utf8(input)?;
        let number = some_str.parse::<i32>()?;
        send_number(number)?;
        Ok(())
    }
    fn send_number(number: i32) -> Result<(), Error> {
        if number < 1_000_000 {
            println!("Number sent!");
            Ok(())
        } else {
            println!("Too large!");
            Err(anyhow!("Number is too large"))
        }
    }

    println!("{:?}", parse_then_send(b"nine"));
    println!("{:?}", parse_then_send(b"10"));
}

pub fn ex03() {
    use anyhow::{anyhow, Context, Error};
    fn parse_then_send(input: &[u8]) -> Result<(), Error> {
        let some_str = std::str::from_utf8(input).with_context(|| "Couldn't parse into a str")?;
        let number = some_str
            .parse::<i32>()
            .with_context(|| format!("Got a weird str to parse: {some_str}"))?;
        send_number(number)?;
        Ok(())
    }
    fn send_number(number: i32) -> Result<(), Error> {
        if number < 1_000_000 {
            println!("Number sent!");
            Ok(())
        } else {
            println!("Too large!");
            Err(anyhow!("Number is too large"))
        }
    }

    println!("{:?}", parse_then_send(b"nine"));
    println!("{:?}", parse_then_send(b"10"));
}

pub fn ex04() {
    use std::{num::ParseIntError, str::Utf8Error};
    use thiserror::Error;
    #[derive(Error, Debug)]
    enum SystemError {
        #[error("Couldn't send: {0}")]
        SendError(String),
        #[error("Couldn't parse into a str: {0}")]
        StringFromUtf8Error(#[from] Utf8Error),
        #[error("Couldn't turn into an i32: {0}")]
        ParseI32Error(#[from] ParseIntError),
        #[error("Wrong color: Red {0} Green {1} Blue {2}")]
        ColorError(u8, u8, u8),
        #[error("Something happened")]
        OtherError,
    }
    fn parse_then_send(input: &[u8]) -> Result<(), SystemError> {
        let some_str = std::str::from_utf8(input)?;
        let number = some_str.parse::<i32>()?;
        send_number(number)?;
        Ok(())
    }
    fn send_number(number: i32) -> Result<(), SystemError> {
        match number {
            num if num == 500 => Err(SystemError::OtherError),
            num if num > 1_000_000 => Err(SystemError::SendError(format!(
                "{num} is too large, can't send!"
            ))),
            _ => {
                println!("Number sent!");
                Ok(())
            }
        }
    }

    println!("{}", parse_then_send(b"nine").unwrap_err());
    println!("{}", parse_then_send(&[8, 9, 0, 200]).unwrap_err());
    println!("{}", parse_then_send(b"109080098").unwrap_err());
    println!("{}", SystemError::ColorError(8, 10, 200));
    parse_then_send(b"10098").unwrap();
}

fn main() {
    ex04();
}
