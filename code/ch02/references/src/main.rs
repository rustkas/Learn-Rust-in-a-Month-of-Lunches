pub fn ex01() {
    let country = String::from("Austria");
    let ref_one = &country;
    let ref_two = &country;
    println!("{}", ref_one);
    println!("{}", ref_two);
}

pub fn ex02() {
    let country = String::from("Austria");
    for index in 0..u64::MAX {
        let ref_one = &country;
        println!("{}) {}", index, ref_one);
    }
}

// fn return_str() -> &String {
//     let country = String::from("Austria");
//     let country_ref = &country;
//     country_ref
// }
// pub fn ex03() {
//     _ = return_str();
// }

fn main() {
    ex02();
}
