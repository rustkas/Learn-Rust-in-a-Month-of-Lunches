pub fn ex01() {
    fn print_maximum(one: i32, two: i32) {
        let higher = if one > two { one } else { two };
        println!("{higher} is higher");
    }
    print_maximum(8, 10);
}

pub fn ex02() {
    use std::fmt::Display;
    fn print_maximum<T: PartialOrd + Display>(one: T, two: T) {
        let higher = if one > two { one } else { two };
        println!("{higher} is higher");
    }
    print_maximum(8, 10);
}

pub fn ex03() {
    use std::fmt::Display;
    fn prints_it(input: impl Into<String> + Display) {
        println!("You can print many things, including {input}");
    }

    let name = "Tuon";
    let string_name = String::from("Tuon");
    prints_it(name);
    prints_it(string_name);
}

pub fn ex04() {
    use std::fmt::Display;

    fn prints_it_impl_trait(input: impl Display) {
        println!("You can print many things, including {input}");
        }
        fn prints_it_regular_generic<T: Display>(input: T) {
        println!("You can print many things, including {input}");
        }

        prints_it_regular_generic::<u8>(100);
        prints_it_regular_generic(100);
        prints_it_impl_trait(100);
        prints_it_impl_trait(100u8);
        // prints_it_impl_trait::<u8>(100); 
}

// pub fn ex05() {
//     use std::fmt::Display;
//     fn gives_higher(one: impl PartialOrd + Display, two: impl PartialOrd + Display) {
//         let higher = if one > two { one } else { two };
//         println!("{higher} is higher.");
//     }
//     gives_higher(8, 10);
// }

pub fn ex06() {
    fn returns_a_closure(input: &str) -> impl FnMut(i32) -> i32 {
        match input {
            "double" => |mut number| {
                number *= 2;
                println!("Doubling number. Now it is {number}");
                number
            },
            "triple" => |mut number| {
                number *= 3;
                println!("Tripling number. Now it is {number}");
                number
            },
            _ => |number| {
                println!("Sorry, it's the same: {number}.");
                number
            },
        }
    }

    let my_number = 10;
    let mut doubles = returns_a_closure("double");
    let mut triples = returns_a_closure("triple");
    let mut does_nothing = returns_a_closure("HI");
    let _doubled = doubles(my_number);
    let _tripled = triples(my_number);
    let _same = does_nothing(my_number);
}


pub fn ex07() {
    enum TimeOfDay {
        Dawn,
        Day,
        Sunset,
        Night,
    }

    fn make_fear_closure(input: TimeOfDay) -> impl FnMut(&mut f64) {
        match input {
            TimeOfDay::Dawn => |x: &mut f64| {
                *x *= 0.5;
                println!(
                    "The morning sun has vanquished the horrible night.
You no longer feel afraid.\n Fear: {x}"
                );
            },
            TimeOfDay::Day => |x: &mut f64| {
                *x *= 0.2;
                println!("What a nice day!\n Fear: {x}");
            },
            TimeOfDay::Sunset => |x: &mut f64| {
                *x *= 1.4;
                println!("The sun is almost down! Oh dear.\n Fear: {x}");
            },
            TimeOfDay::Night => |x: &mut f64| {
                *x *= 5.0;
                println!("What a horrible night to have a curse.\n Fear: {x}");
            },
        }
    }

    use TimeOfDay::*;
    let mut fear = 10.0;
    let mut make_daytime = make_fear_closure(Day);
    let mut make_sunset = make_fear_closure(Sunset);
    let mut make_night = make_fear_closure(Night);
    let mut make_morning = make_fear_closure(Dawn);
    make_daytime(&mut fear);
    make_sunset(&mut fear);
    make_night(&mut fear);
    make_morning(&mut fear);
}

fn main() {
    ex07();
}
