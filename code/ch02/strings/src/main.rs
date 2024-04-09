pub fn ex01() {
    let name = "자우림";
    let other_name = String::from("Adrian Fahrenheit Țepeș");

    println!("{name}", name = name);
    println!("{other_name}", other_name = other_name);

    let name = "😀";
    println!("My name is actually {}", name);
}

use std::mem::size_of;
pub fn ex02() {
    let size_of_string = size_of::<String>();
    let size_of_i8 = size_of::<i8>();
    let size_of_f64 = size_of::<f64>();
    let size_of_jaurim = std::mem::size_of_val("자우림");
    let size_of_adrian = std::mem::size_of_val("Adrian Fahrenheit Țepeș");
    println!(
        "A String is Sized and always {size_of_string} bytes.",
        size_of_string = size_of_string
    );
    println!(
        "An i8 is Sized and always {size_of_i8} bytes.",
        size_of_i8 = size_of_i8
    );
    println!(
        "An f64 is always Sized and {size_of_f64} bytes.",
        size_of_f64 = size_of_f64
    );
    println!(
        "But a &str is not Sized: '자우림' is {size_of_jaurim} bytes.",
        size_of_jaurim = size_of_jaurim
    );
    println!(
        "And 'Adrian Fahrenheit Țepeș' is {size_of_adrian} bytes - not Sized.",
        size_of_adrian = size_of_adrian
    );
}

pub fn ex03() {
    let my_name = "My name";
    println!("{}", my_name);
    // println!("{}", *my_name);
}
pub fn ex04() {
    let _string = String::from("This is the string text");
    let _string = "This is the string text".to_string();

    {
        let name = "Billy Brobby";

        let country = "USA";
        let home = "Korea";
        let together = format!("I am {name} from {country} but I live in {home}.");
        println!("{}", together);
    }
}

#[allow(unused_variables)]
pub fn ex05() {
    let my_string:String = "Try to make this a String".into();
}

fn main() {
    ex04();
}
