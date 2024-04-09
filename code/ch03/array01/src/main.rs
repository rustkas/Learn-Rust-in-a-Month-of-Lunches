mod util;

pub fn ex01() {
    let array1 = ["One", "Two"];
    let array2 = ["One", "Two"];
    let array3 = ["One", "Two", "Five"];

    println!("{array1:?}");
    println!("{:?}", array2);
    println!("{:#?}", array3);

    println!("{}", util::get_type(&array1));
}

pub fn ex02() {
    let array1 = ["One", "Two"];
    let array2 = ["One", "Two"];
    let array3 = ["One", "Two", "Five"];

    println!("{}", util::get_type(&array1));
    println!("{}", util::get_type(&array2));
    println!("{}", util::get_type(&array3));
}

pub fn ex03() {
    let my_array = ["a"; 5];
    println!("{:?}", my_array);
}

pub fn ex04() {
    let mut buffer = [0u8; 640];
    buffer[0] = 1;
    println!("buffer's type = {}", util::get_type(&buffer));
    println!("{buffer:?}");
}

pub fn ex05() {
    // println!("{}", b"Hello there");
    println!("{:?}", b"Hello there");
}

pub fn ex06() {
    let my_numbers = [0, 10, -20];
    println!("{}", my_numbers[1]);
}

pub fn ex07() {
    let array_of_ten = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let two_to_five = &array_of_ten[2..5];
    let start_at_one = &array_of_ten[1..];
    let end_at_five = &array_of_ten[..5];
    let everything = &array_of_ten[..];
    println!(
        "Two to five: {two_to_five:?},
Start at one: {start_at_one:?},
End at five: {end_at_five:?},
Everything: {everything:?}"
    );
}

fn main() {
    ex07();
}
