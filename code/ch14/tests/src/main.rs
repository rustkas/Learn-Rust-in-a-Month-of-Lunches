#![allow(dead_code)]

#[test]
fn two_is_two() {
    assert_eq!(2, 2);
}
// #[test]
// fn test_that_wont_work(input: i32) {}

#[test]
fn two_is_three() {
    assert_eq!(2, 3);
}

#[test]
fn two_is_two2() {
    std::env::set_var("RUST_BACKTRACE", "full");
    assert!(2 == 3);
}

pub fn ex01() {
    println!("{:?}", std::env::var("RUST_BACKTRACE"));
}

pub fn ex02() {
    println!(
        "{:?}",
        std::env::set_var("RUST_BACKTRACE", 1.to_string().as_str())
    );
    println!("{:?}", std::env::var("RUST_BACKTRACE"));
}

fn return_two() -> i8 {
    2
}
#[test]
fn it_returns_two() {
    assert_eq!(return_two(), 2);
}
fn return_six() -> i8 {
    4 + return_two()
}
#[test]
fn it_returns_six() {
    assert_eq!(return_six(), 6)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_returns_six() {
        assert_eq!(return_six(), 6)
    }
    #[test]
    fn it_returns_two() {
        assert_eq!(return_two(), 2);
    }
}



fn main() {
    ex02();
}
