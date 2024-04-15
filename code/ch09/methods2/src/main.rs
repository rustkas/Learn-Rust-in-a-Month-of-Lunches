pub fn ex01() {
    for num in ["9", "nine", "ninety-nine", "9.9"]
        .into_iter()
        .map(|num| num.parse::<f32>())
    {
        println!("{num:?}");
    }
}
pub fn ex0101() {
    for num in ["9", "nine", "ninety-nine", "9.9"]
        .into_iter()
        .map(|num| num.parse::<f32>())
        .flatten()
    {
        println!("{num:?}");
    }
}

pub fn ex02() {
    fn in_char_vec(char_vec: &Vec<char>, check: char) {
        println!(
            "Is {check} inside? {}",
            char_vec.iter().any(|&char| char == check)
        );
    }

    let char_vec = ('a'..'働').collect::<Vec<char>>();
    // char_vec.clone().into_iter().for_each(|char|print!("{char}, "));
    in_char_vec(&char_vec, 'i');
    in_char_vec(&char_vec, '뷁');
    in_char_vec(&char_vec, '鑿');
    let smaller_vec = ('A'..'z').collect::<Vec<char>>();
    println!(
        "All alphabetic? {}",
        smaller_vec.iter().all(|&x| x.is_alphabetic())
    );
    println!(
        "All less than the character 행? {}",
        smaller_vec.iter().all(|&x| x < '행')
    );
}

pub fn ex0201() {
    fn in_char_vec(char_vec: &Vec<char>, check: char) {
        println!(
            "Is {check} inside? {}",
            char_vec.iter().any(|&char| char == check)
        );
    }

    let char_vec = ('a'..'働').collect::<Vec<char>>();
    // char_vec.clone().into_iter().for_each(|char|print!("{char}, "));
    println!();
    in_char_vec(&char_vec, 'i');
    in_char_vec(&char_vec, '僋');
    in_char_vec(&char_vec, '僉');
    let smaller_vec = ('A'..'z').collect::<Vec<char>>();
    println!(
        "All alphabetic? {}",
        smaller_vec.iter().all(|&x| x.is_alphabetic())
    );
    println!(
        "All less than the character 働? {}",
        smaller_vec.iter().all(|&x| x < '働')
    );
}

pub fn ex03() {
    let mut big_vec = vec![6; 1000];
    big_vec.push(5);
    let mut iterator = big_vec.iter().rev();
    assert_eq!(iterator.next(), Some(&5));
    assert_eq!(iterator.next(), Some(&6));
}
pub fn ex04() {
    let mut big_vec = vec![6; 1000];
    big_vec.push(5);
    println!("{:?}", big_vec.iter().rev().any(|&number| number == 5));
}
pub fn ex05() {
    let mut big_vec = vec![6; 1000];
    big_vec.push(5);
    let mut num_loops = 0;
    let mut big_iter = big_vec.into_iter();
    loop {
        num_loops += 1;
        if big_iter.next() == Some(5) {
            break;
        }
    }
    println!("Number of loops: {num_loops}");
}

// position, find
pub fn ex06() {
    let num_vec = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 11];
    println!("{:?}", num_vec.iter().find(|number| *number % 3 == 0));
    println!("{:?}", num_vec.iter().position(|number| *number % 3 == 0));
    println!("{:?}", num_vec.iter().find(|number| *number % 13 == 0));
    println!("{:?}", num_vec.iter().position(|number| *number % 11 == 0));
}

// Cycling, zipping, folding, and more
pub fn ex07() {
    let even_odd_iter = ["even", "odd"].into_iter().cycle();
    let even_odd_vec: Vec<(i32, &str)> = (0..=5).zip(even_odd_iter).collect();
    println!("{:?}", even_odd_vec);
}

pub fn ex08() {
    let ten_chars: Vec<char> = ('a'..).take(10).collect();
    let skip_then_ten_chars: Vec<char> = ('a'..).skip(1300).take(10).collect();
    println!("{ten_chars:?}");
    println!("{skip_then_ten_chars:?}");
}

pub fn ex09() {
    let some_numbers = vec![9, 6, 9, 10, 11];
    println!(
        "{}",
        some_numbers
            .iter()
            .fold(0, |total_so_far, next_number| total_so_far + next_number)
    );
}

pub fn ex10() {
    #[derive(Debug)]
    struct CombinedEvents {
        num_of_events: u32,
        data: Vec<String>,
    }

    impl Default for CombinedEvents {
        fn default() -> Self {
            CombinedEvents {
                num_of_events: 0,
                data: Vec::new(),
            }
        }
    }

    let events = [
        "Went to grocery store",
        "Came home",
        "Fed cat",
        "Fed cat again",
    ];
    let empty_events = CombinedEvents::default();

    let combined_events = events
        .iter()
        .fold(empty_events, |mut total_events, next_event| {
            total_events.num_of_events += 1;
            total_events.data.push(next_event.to_string());
            total_events
        });
    println!("{combined_events:#?}");
}

pub fn ex11() {
    let mut number_iter = [7, 8, 9, 10].into_iter();
    let _first_two = number_iter.by_ref().take(2).collect::<Vec<_>>();
    let _second_two = number_iter.take(2).collect::<Vec<_>>();
}

pub fn ex12() {
    let num_vec = vec![1, 2, 3, 4, 5, 6, 7];
    for chunk in num_vec.chunks(3) {
        println!("{:?}", chunk);
    }
    println!();
    for window in num_vec.windows(3) {
        println!("{:?}", window);
    }
}

pub fn ex13() {
    let some_str = "Er ist noch nicht erklärt. Aber es gibt Krieg. Verlaß dich drauf.";
    for (index, item) in some_str.match_indices(|c| c > 'z') {
        println!("{item} at {index}");
    }
    for (index, item) in some_str.match_indices(". ") {
        println!("'{item}' at index {index}");
    }
}

pub fn ex14() {
    let just_numbers = vec![1];
    let mut number_iter = just_numbers.iter().peekable();
    for _ in 0..1 {
        println!("I love the number {}", number_iter.peek().unwrap());
        println!("I really love the number {}", number_iter.peek().unwrap());
        println!("{} is such a nice number", number_iter.peek().unwrap());
        number_iter.next();
    }
}

fn main() {
    ex14();
}
