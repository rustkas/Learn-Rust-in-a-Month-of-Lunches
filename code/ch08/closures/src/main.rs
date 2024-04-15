pub fn ex01() {
    let my_closure = || println!("This is a closure");
    my_closure();
}

pub fn ex02() {
    let my_closure = |x: i32| println!("{x}");
    my_closure(5);
    my_closure(5 + 5);
}

pub fn ex03() {
    let my_closure = || {
        let number = 7;
        let other_number = 10;
        println!("The two numbers are {number} and {other_number}.");
    };
    my_closure();
}
pub fn ex04() {
    let number_one = 6;
    let number_two = 10;
    let my_closure = || println!("{}", number_one + number_two);
    my_closure();
}

pub fn ex05() {
    let number_one = 6;
    let number_two = 10;
    let my_closure = |x: i32| println!("{}", number_one + number_two + x);
    my_closure(5);
}

pub fn ex06() {
    (1..3).for_each(|num| println!("{num}"));
    (1..=3).for_each(|num| {
        println!("Got a {num}!");
        if num % 2 == 0 {
            println!("It's even")
        } else {
            println!("It's odd")
        };
    });
}

pub fn ex07() {
    let my_vec = vec![8, 9, 10];
    let fourth = my_vec.get(2).unwrap_or_else(|| {
        if let Some(val) = my_vec.get(2) {
            val
        } else {
            &0
        }
    });
    println!("{fourth}");
    let fourth = my_vec.get(3).unwrap_or_else(|| &0);
    println!("{fourth}");
}

pub fn ex08() {
    let char_vec = vec!['z', 'y', 'x'];
    char_vec
        .iter()
        .enumerate()
        .for_each(|(index, c)| println!("Index {index} is: {c}"));
}

pub fn ex09() {
    let num_vec = vec![2, 4, 6];
    let double_vec: Vec<i32> = num_vec.iter().map(|num| num * 2).collect();
    println!("{:?}", double_vec);
}
pub fn ex0901() {
    let num_vec = vec![2, 4, 6];
    let _ = num_vec
        .iter()
        .enumerate()
        .map(|(index, num)| println!("Index {index} is {num}"));
}

pub fn ex10() {
    use std::collections::HashMap;
    let some_keys = vec![0, 1, 2, 3, 4, 5];
    let some_values = vec!["zero", "one", "two", "three", "four", "five"];
    let number_word_hashmap = some_keys
        .into_iter()
        .zip(some_values.into_iter())
        .collect::<HashMap<_, _>>();
    println!(
        "The value at key 2 is: {}",
        number_word_hashmap.get(&2).unwrap()
    );
}
pub fn ex1001() {
    use std::collections::HashMap;
    let some_keys = vec![0, 1, 2, 3, 4, 5];
    let some_values = vec!["zero", "one", "two", "three", "four", "five"];
    let number_word_hashmap: HashMap<_, _> =
        some_keys.into_iter().zip(some_values.into_iter()).collect();
    println!(
        "The value at key 2 is: {}",
        number_word_hashmap.get(&2).unwrap()
    );
}
pub fn ex1002() {
    use std::collections::HashMap;
    let keys = vec![0, 1, 2, 3, 4, 5].into_iter();
    let values = vec!["zero", "one", "two", "three", "four", "five"].into_iter();
    let number_word_hashmap: HashMap<_, _> = keys.zip(values).collect();
    println!(
        "The value at key 2 is: {}",
        number_word_hashmap.get(&2).unwrap()
    );
}

pub fn ex11() {
    let numbers_together = "140399923481800622623218009598281";
    for (index, num) in numbers_together.char_indices() {
        match (index % 3, num) {
            (0 | 1, num) => print!("{num}"),
            _ => print!(
                "{value}: {string}\t",
                value = num,
                string = String::from(num)
            ),
        }
    }
    println!();
}

pub fn ex12() {
    let my_vec = vec![8, 9, 10];
    my_vec
        .iter()
        .for_each(|_| println!("We didn't use the variables at all"));
}

fn main() {
    ex12();
}
