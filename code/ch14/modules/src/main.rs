#![allow(dead_code)]

pub fn ex01() {
    mod print_things {
        use std::fmt::Display;
        pub fn prints_one_thing<T: Display>(input: T) {
            println!("{input}");
        }
        #[derive(Debug)]
        pub struct Email(pub String);
    }

    use print_things::prints_one_thing;
    prints_one_thing(6);
    prints_one_thing("Trying to print a string...".to_string());

    use print_things::Email;
    let email = Email("new@mail.dev".to_string());
    println!("{email:?}");
}

pub fn ex02() {
    mod print_things {
        #[derive(Debug)]
        pub struct Billy {
            name: String,
            pub times_to_print: u32,
        }
        impl Billy {
            pub fn new(times_to_print: u32) -> Self {
                Self {
                    name: "Billy".to_string(),
                    times_to_print,
                }
            }
            pub fn print_billy(&self) {
                for _ in 0..self.times_to_print {
                    println!("{}", self.name);
                }
            }
        }
    }

    use print_things::*;
    let my_billy = Billy::new(3);
    my_billy.print_billy();
}

pub fn ex03() {
    mod country {
        fn print_country(country: &str) {
            println!("We are in the country of {country}");
        }
        pub mod province {
            fn print_province(province: &str) {
                println!("in the province of {province}");
            }
            pub mod city {
                pub fn print_city(country: &str, province: &str, city: &str) {
                    // ::country::print_country(country);

                    super::super::print_country(country);
                    // crate::country::province::print_province(province);
                    super::print_province(province);
                    println!("in the city of {city}");
                }
            }
        }
    }
    country::province::city::print_city("Canada", "New Brunswick", "Moncton");
}

fn main() {
    ex03();
}
