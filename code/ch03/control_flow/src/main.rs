use rand::Rng;

mod util;

pub fn ex02() {
    let my_number = 5;
    if my_number == 7 {
        println!("It's seven");
    } else if my_number == 6 {
        println!("It's six")
    } else {
        println!("It's a different number")
    }
}

pub fn ex01() {
    let my_number = 5;
    if my_number == 7 {
        println!("It's seven");
    }
}

pub fn ex03() {
    let mut rng = rand::thread_rng();
    let n1: i8 = rng.gen();
    let boolean = rng.gen_bool(1.0 / 3.0);
    let my_number: i8;

    if boolean {
        my_number = n1;
    } else {
        my_number = -n1;
    }
    if my_number % 2 == 1 && my_number > 0 {
        println!("It's a positive odd number");
    } else if my_number == 6 {
        println!("It's six")
    } else {
        println!("{my_number}. It's a different number")
    }
}

pub fn ex04() {
    let mut rng = rand::thread_rng();
    let my_number: u8 = rng.gen();
    match my_number {
        0 => println!("it's zero"),
        1 => println!("it's one"),
        2 => println!("it's two"),
        _ => println!("it's {my_number}"),
    }
}

pub fn ex05() {
    let sky = "cloudy";
    let temperature = "warm";
    match (sky, temperature) {
        ("cloudy", "cold") => println!("It's dark and unpleasant today"),
        ("clear", "warm") => println!("It's a nice day"),
        ("cloudy", "warm") => println!("It's dark but not bad"),
        _ => println!("Not sure what the weather is."),
    }
}

pub fn ex06() {
    let mut rng = rand::thread_rng();
    let mut n1: u8 = rng.gen();
    let boolean = rng.gen_bool(1.0 / 3.0);

    if n1 > 10 {
        n1 %= 10;
    }
    let children = n1;
    let married = boolean;
    match (children, married) {
        (children, married) if married == false => println!("Not married with {children} kids"),
        (children, married) if children == 0 && married == true => {
            println!("Married but no children")
        }
        _ => println!("Married? {married}. Number of children: {children}."),
    }
}

pub fn ex07() {
    let mut rng = rand::thread_rng();
    let mut n1: u8 = rng.gen();
    let boolean = rng.gen_bool(1.0 / 3.0);

    if n1 > 10 {
        n1 %= 10;
    }
    let children = n1;
    let married = boolean;
    let married_text: &str;
    if married {
        married_text = "True";
    } else {
        married_text = "False";
    }
    match (children, married) {
        (children, married) if !married && children == 0 => println!("Not married. No kids."),
        (children, married) if !married => println!("Not married with {children} kids."),
        (children, married) if children == 0 && married => {
            println!("Married but no children.")
        }
        _ => println!("Married? {married_text}. Number of children: {children}."),
    }
}

fn match_colors(rgb: (i32, i32, i32)) {
    match rgb {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, g, _) if g < 10 => println!("Not much green"),
        (_, _, b) if b < 10 => println!("Not much blue"),
        _ => println!("Each color has at least 10"),
    }
}

pub fn ex08() {
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);
    match_colors(first);
    match_colors(second);
    match_colors(third);
}

pub fn ex09() {
    let my_number = 10;

    if my_number == 10 {
        let some_variable = 8;
        println!("Type of {variable_name} = {type}", variable_name = stringify!(result), type = crate::util::get_type(&some_variable));
    } else {
        let some_variable = "Something else";
        println!("Type of {variable_name} = {type}", variable_name = stringify!(result), type = crate::util::get_type(&some_variable));
    }
}

fn match_number(input: i32) {
    match input {
        number @ 4 => println!("{number} is unlucky in China (sounds close to 死)!"),
        number @ 13 => println!("{number} is lucky in Italy! In bocca al lupo!"),
        number @ 14..=19 => println!("Some other number that ends with -teen: {number}"),
        _ => println!("Some other number, I guess. {input}"),
    }
}

pub fn ex10() {
    match_number(50);
    match_number(13);
    match_number(16);
    match_number(4);
}

fn main() {
    ex10();
}
