pub fn ex01() {
    let my_number = 9;
    let reference = &my_number;
    // println!("{}", my_number == reference);
    println!("{name} = {value}. ref = {ref:p}. Dereferencing = {dereferencing}", 
        name = stringify!(my_number), value = my_number, ref = reference, dereferencing = my_number == *reference);
    println!("{}", my_number == *reference);
}

pub fn ex02() {
    let my_name = "Billy".to_string();
    println!("{}", my_name.is_empty());
}

pub fn ex03() {
    let my_name = "Billy".to_string();
    let other_name = "Billy".to_string();
    // println!("{}", my_name == &other_name);
    println!("{}", &my_name == &other_name);
}

pub fn ex04() {
    let my_name = "Billy".to_string();
    let double_ref = &&my_name;
    println!("{}", double_ref.is_empty());
}

pub fn ex05() {
    let my_name = "Billy".to_string();
    let double_ref = &&my_name;
    let result = &**double_ref;
    println!("{}", result.is_empty());
}

pub fn ex06() {
    let my_name = "Billy".to_string();
    let my_ref = &my_name;
    let result = &&&&&my_ref;
    println!("{}", result.is_empty());
}

fn main() {
    ex06();
}
