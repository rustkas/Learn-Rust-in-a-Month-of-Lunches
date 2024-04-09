fn print_country(country_name: &String) {
    println!("{country_name}");
}

pub fn ex01() {
    let country = String::from("Austria");
    print_country(&country);
    print_country(&country);
}

fn add_hungary(country_name: &mut String) {
    country_name.push_str("-Hungary");
    println!("Now it says: {country_name}");
}
pub fn ex02() {
    let mut country = String::from("Austria");
    add_hungary(&mut country);
    println!("{country}");
}

fn adds_hungary(mut string_to_add_hungary_to: String) {
    string_to_add_hungary_to.push_str("-Hungary");
    println!("{}", string_to_add_hungary_to);
}

pub fn ex03() {
    let country = String::from("Austria");
    adds_hungary(country);
}

fn main() {
    ex03();
}
