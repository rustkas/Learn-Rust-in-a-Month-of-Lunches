#[allow(unused_variables)]
pub fn ex01() {
    let first_letter = 'A';
    let space = ' ';
    let other_language_char = 'Ꮔ';
    let cat_face = '🐈';

    let my_number = 100 as u8;
    println!("{}", my_number as char);
    println!("{}", 101 as u8 as char);

    // println!("{}", 256 as u8 as char);
}

pub fn ex02() {
    println!("Size of a char: {}", std::mem::size_of::<char>());
    println!("Size of a: {}", "a".len());
    println!("Size of ß: {}", "ß".len());
    println!("Size of 国: {}", "国".len());
    println!("Size of 𓅱: {}", "𓅱".len());
}

pub fn ex03_01() {
    let str1 = "Hello!";
    println!("str1 is {} bytes.", str1.len());
    let str2 = "안녕!";
    println!("str2 is {} bytes.", str2.len());
}

pub fn ex02_02() {
    println!("{:?}", "a".as_bytes());
    println!("{:?}", "ß".as_bytes());
    println!("{:?}", "国".as_bytes());
    println!("{:?}", "𓅱".as_bytes());
}

pub fn ex03_02() {
    let str1 = "Hello!";
    println!(
        "str1 is {} bytes and also {} characters.",
        str1.len(),
        str1.chars().count()
    );
    let str2 = "안녕!";
    println!(
        "str2 is {} bytes but only {} characters.",
        str2.len(),
        str2.chars().count()
    );
}

#[allow(unused_variables)]
pub fn ex04() {
    let small_number: u8 = 10;

    let small_number = 10_u8;
    let big_number = 100_000_000_i32;

    let number = 0________u8;
    let number2 = 1___6______2____4______i32;
    println!("{}, {}", number, number2);
}

#[allow(unused_variables)]
pub fn ex05() {
    let my_float = 5.;

    let my_float: f64 = 5.0;
    let my_other_float: f32 = 8.5;
    let third_float = my_float + my_other_float as f64;

    let my_float = 5.0;
    let my_other_float = 8.5;
    let third_float = my_float + my_other_float;
}

pub fn ex06() {
    println!("Hello, world number {}!", 8);
}

pub fn ex07() {
    println!("Hello, worlds number {} and {}!", 8, 9);
}

fn give_number() -> i32 {
    8
}
pub fn ex08() {
    println!("Hello, world number {}!", give_number());
}

#[allow(unreachable_code)]
fn give_number02() -> i32 {
    return 8;
    10;
}

pub fn ex09() {
    println!("Hello, world number {}", give_number02());
}

fn multiply(number_one: i32, number_two: i32) {
    let result = number_one * number_two;
    println!("{} times {} is {}", number_one, number_two, result);
}

pub fn ex10() {
    multiply(8, 9);
    let some_number = 10;
    let some_other_number = 2;
    multiply(some_number, some_other_number);
}

fn multiply02(number_one: i32, number_two: i32) -> i32 {
    let result = number_one * number_two;
    result
}

pub fn ex11() {
    let multiply_result = multiply02(8, 9);
    println!(
        "The two numbers multiplied are: {multiply_result}",
        multiply_result = multiply_result
    );
}

fn multiply03(number_one: i32, number_two: i32) -> i32 {
    number_one * number_two
}

pub fn ex12() {
    let multiply_result = multiply03(8, 9);
    println!("The two numbers multiplied are: {}", multiply_result);
}

pub fn ex13() {
    let naver_base_url = "naver";
    let google_base_url = "google";
    let microsoft_base_url = "microsoft";
    println!("The url is www.{naver_base_url}.com");
    println!("The url is www.{google_base_url}.com");
    println!("The url is www.{microsoft_base_url}.com");
    println!("The url is www.{}.com", naver_base_url);
    println!("The url is www.{}.com", google_base_url);
    println!("The url is www.{}.com", microsoft_base_url);
}

pub fn ex14() {
    let my_number = {
        let second_number = 8;
        second_number + 9
    };
    println!("My number is: {}", my_number);
}

// pub fn ex15() {
//     let my_number = {
//         let second_number = 8;
//         second_number + 9;
//         };
//         println!("My number is: {}", my_number);
// }

// pub fn ex16() {
//     let doesnt_print = ();
//     println!("This will not print: {}", doesnt_print);
// }

pub fn ex17() {
    print!("This will not print a new line");
    println!(" so this will be on the same line");
}

pub fn ex18() {
    println!("The smallest i8: {} The biggest i8: {}", i8::MIN, i8::MAX);
    println!("The smallest u8: {} The biggest u8: {}", u8::MIN, u8::MAX);
    println!(
        "The smallest i16: {} The biggest i16: {}",
        i16::MIN,
        i16::MAX
    );
    println!(
        "The smallest u16: {} and the biggest u16: {}",
        u16::MIN,
        u16::MAX
    );
    println!(
        "The smallest i32: {} The biggest i32: {}",
        i32::MIN,
        i32::MAX
    );
    println!(
        "The smallest u32: {} The biggest u32: {}",
        u32::MIN,
        u32::MAX
    );
    println!(
        "The smallest i64: {} The biggest i64: {}",
        i64::MIN,
        i64::MAX
    );
    println!(
        "The smallest u64: {} The biggest u64: {}",
        u64::MIN,
        u64::MAX
    );
    println!(
        "The smallest i128: {} The biggest i128: {}",
        i128::MIN,
        i128::MAX
    );
    println!(
        "The smallest u128: {} The biggest u128: {}",
        u128::MIN,
        u128::MAX
    );
}

fn times_two(number: i32) -> i32 {
    number * 2
}

fn ex19() {
    let final_number = {
        let y = 10;
        let x = 9;
        let x_twice = times_two(x);
        let x_twice_and_y = x_twice + y;
        x_twice_and_y
    };
    println!("The number is now: {}", final_number)
}

#[allow(unused_variables)]
fn main() {
    ex19();
}
