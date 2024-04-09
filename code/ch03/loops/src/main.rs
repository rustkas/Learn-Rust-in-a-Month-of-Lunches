pub fn ex01() {
    loop {}
}

pub fn ex02() {
    let mut counter = 0;
    loop {
        counter += 1;
        println!("The counter is now: {counter}");
        if counter == 5 {
            break;
        }
    }
}

pub fn ex03() {
    let mut counter = 0;
    let mut counter2 = 0;
    println!("Now entering the first loop.");

    'first_loop: loop {
        counter += 1;
        println!("The counter is now: {counter}");
        if counter > 5 {
            println!("\tNow entering the second loop.");

            'second_loop: loop {
                counter2 += 1;
                println!("\t\tThe second counter is now: {counter2}");
                if counter2 == 3 {
                    break 'first_loop;
                } else if counter > 7 {
                    break 'second_loop;
                }
            }
        }
    }
}

pub fn ex04() {
    let mut counter = 0;
    while counter < 5 {
        counter += 1;
        println!("The counter is now: {counter}");
    }
}

pub fn ex05() {
    for number in 0..3 as u8 {
        println!("The number is: {}", number);
    }
    for number in 0..=3 as u8 {
        println!("The next number is: {}", number);
    }
}

pub fn ex06() {
    for _ in 0..3 {
        println!("Printing the same thing three times");
    }
}

pub fn ex07() {
    let mut counter = 50;
    let my_number = loop {
        counter += 1;
        if counter % 53 == 3 {
            break counter;
        }
    };
    println!("{my_number}");
}

fn match_colors(rbg: (i32, i32, i32)) {
    let (red, blue, green) = (rbg.0, rbg.1, rbg.2);
    println!("Comparing a color with {red} red, {blue} blue, and {green} green:");
    let color_vec = vec![(red, "red"), (blue, "blue"), (green, "green")];
    let mut all_have_at_least_10 = true;
    for (amount, color) in color_vec {
        if amount < 10 {
            all_have_at_least_10 = false;
            println!("Not much {color}.");
        }
    }
    if all_have_at_least_10 {
        println!("Each color has at least 10.")
    }

    println!();
}

pub fn ex08() {
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);
    match_colors(first);
    match_colors(second);
    match_colors(third);
}

fn main() {
    ex08();
}
