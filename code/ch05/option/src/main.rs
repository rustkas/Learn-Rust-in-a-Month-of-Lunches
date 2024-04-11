mod util;

pub fn ex01() {
    fn take_fifth_item(value: Vec<i32>) -> i32 {
        value[4]
    }

    let new_vec = vec![1, 2];
    let index = take_fifth_item(new_vec);
    println!("{name} = {value}", name = stringify!(index), value = index);
}

pub fn ex02() {
    fn try_take_fifth(value: Vec<i32>) -> Option<i32> {
        if value.len() < 5 {
            None
        } else {
            Some(value[4])
        }
    }

    let small = vec![1, 2];
    let big = vec![1, 2, 3, 4, 5];
    println!(
        "Name = {name:?}, size = {size},  result = {result:?}",
        name = stringify!(small),
        size = small.len(),
        result = try_take_fifth(small)
    );
    println!(
        "Name = {name:?}, size = {size},  result = {result:?}",
        name = stringify!(big),
        size = big.len(),
        result = try_take_fifth(big)
    );
}

pub fn ex03() {
    fn try_take_fifth(value: Vec<i32>) -> Option<i32> {
        if value.len() < 5 {
            None
        } else {
            Some(value[4])
        }
    }

    let small = vec![1, 2];
    let big = vec![1, 2, 3, 4, 5];

    println!(
        "Name = {name:?}, size = {size},  result = {result:?}",
        name = stringify!(small),
        size = small.len(),
        result = try_take_fifth(small).unwrap()
    );
    println!(
        "Name = {name:?}, size = {size},  result = {result:?}",
        name = stringify!(big),
        size = big.len(),
        result = try_take_fifth(big).unwrap()
    );
}

pub fn ex04() {
    fn try_take_fifth(value: Vec<i32>) -> Option<i32> {
        if value.len() < 5 {
            None
        } else {
            Some(value[4])
        }
    }
    fn handle_options(my_option: &Vec<Option<i32>>) {
        for item in my_option {
            match item {
                Some(number) => println!("Found a {number}!"),
                None => println!("Found a None!"),
            }
        }
    }

    let small = vec![1, 2];
    let big = vec![1, 2, 3, 4, 5];
    let mut option_vec = Vec::new();
    option_vec.push(try_take_fifth(small));
    option_vec.push(try_take_fifth(big));
    handle_options(&option_vec);
}

pub fn ex05() {
    fn try_take_fifth(value: Vec<i32>) -> Option<i32> {
        if value.len() < 5 {
            None
        } else {
            Some(value[4])
        }
    }

    let small = vec![1, 2];
    let big = vec![1, 2, 3, 4, 5];
    for vec in vec![small, big] {
        let inside_number = try_take_fifth(vec);
        if inside_number.is_some() {
            println!("We got: {}", inside_number.unwrap());
        } else {
            println!("We got nothing.");
        }
    }
}

fn main() {
    ex05();
}
