fn prints_number(number: i32) {
    println!("{}", number);
}

pub fn ex01() {
    let my_number = 8;
    prints_number(my_number);
    prints_number(my_number);
}

fn prints_country(country_name: String) {
    println!("{country_name}");
}
pub fn ex02() {
    let country = String::from("Kiribati");
    prints_country(country);
    // prints_country(country);
}

fn prints_country02(country_name: String) {
    println!("{}", country_name);
}
pub fn ex03() {
    let country = String::from("Kiribati");
    prints_country02(country.clone());
    prints_country02(country.clone());
}

fn get_length(input: String) {
    println!("It's {} words long.", input.split_whitespace().count());
}

pub fn ex04() {
    let mut my_string = String::new();
    for _ in 0..50 {
        my_string.push_str("Here are some more words ");
        get_length(my_string.clone());
    }
}

fn get_length02(input: &String) {
    println!("It's {} words long.", input.split_whitespace().count());
}

pub fn ex05() {
    let mut my_string = String::new();
    for _ in 0..50 {
        my_string.push_str("Here are some more words ");
        get_length02(&my_string);
    }
}

fn main() {
    ex05();
}
