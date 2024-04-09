pub fn ex01() {
    let mut my_number = 8;
    let num_ref = &mut my_number;
    *num_ref = 12;
    println!("my_number = {}", my_number);
}

pub fn ex02() {
    let mut my_number = 8;
    let num_ref = &mut my_number;
    *num_ref += 100;
    println!("{}", my_number);

    let second_number = 800;
    let triple_reference = &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&second_number;
    println!(
        "Are they equal? {}",
        second_number == *********************************triple_reference
    );
}

// pub fn ex03() {
//     let mut number = 10;
//     let number_ref = &number;
//     let number_change = &mut number;
//     *number_change += 10;
//     println!("{}", number_ref);
// }

pub fn ex04() {
    let mut number = 10;

    let number_change = &mut number;
    *number_change += 10;
    let number_ref = &number;
    println!("{}", number_ref);
}

fn main() {
    ex04();
}
