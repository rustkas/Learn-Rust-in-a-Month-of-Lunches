pub fn ex01() {
    let country = String::from("Austria");
    let country_ref = &country;
    let country = 8;
    println!("{country_ref} {country}");
}

pub fn ex02() {
    let country = String::from("Austria");
    let country_ref = &country;
    let country = 8;
    println!("{country_ref}, {country}");
}
fn main() {
    ex02();
}
