pub fn ex01() {
    fn prints_str(my_str: &str) {
        println!("{my_str}");
    }

    let my_string = String::from("I am a string");
    prints_str(&my_string);
    println!("{my_string}");
}
pub fn ex02() {
    fn works() -> &'static str {
        "I live forever!"
    }

    println!("{text}", text = works());
    let text = works();
    for _ in 0..3 {
        println!("{text}", text = text);
    }

    fn does_not_work() -> &'static str {
        // &String::from("Sorry, I only live inside the fn. Not 'static")
        "I live forever!"
    }
    let _ = does_not_work();
}

pub fn ex03() {
    fn returns_reference() -> String {
        let my_string = String::from("I am a string");
        my_string
    }
    let _ = returns_reference();

    fn returns_str() -> &'static str {
        let _my_string = String::from("I am a string");
        "I am a str"
    }
    let my_str = returns_str();
    println!("{my_str}");
}
// #[allow(dead_code)]
pub fn ex04() {
    #[derive(Debug)]
    struct City {
        name: &'static str,
        date_founded: u32,
    }
    let my_city = City {
        name: "Ichinomiya",
        date_founded: 1921,
    };
    println!("{} was founded in {}", my_city.name, my_city.date_founded);
}

pub fn ex05() {
    #[derive(Debug)]
    struct City {
        name: &'static str,
        date_founded: u32,
    }
    let city1 = "Ichinomiya";
    let _city_names = vec![city1.to_string(), "Kurume".to_string()];
    let name: &'static str;
    name = city1;
    let my_city = City {
        name,
        date_founded: 1921,
    };
    println!("{} was founded in {}", my_city.name, my_city.date_founded);
}

pub fn ex06() {
    #[derive(Debug)]
    struct City<'a> {
        name: &'a str,
        date_founded: u32,
    }

    let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()];
    let my_city = City {
        name: &city_names[0],
        date_founded: 1921,
    };
    println!("{} was founded in {}", my_city.name, my_city.date_founded);
}

pub fn ex0601() {
    #[derive(Debug)]
    struct City<'city> {
        name: &'city str,
        date_founded: u32,
    }

    let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()];
    let my_city = City {
        name: &city_names[0],
        date_founded: 1921,
    };
    println!("{} was founded in {}", my_city.name, my_city.date_founded);
}

pub fn ex07() {
    #[derive(Debug)]
    struct Adventurer<'a> {
        name: &'a str,
        hit_points: u32,
    }
    impl Adventurer<'_> {
        fn take_damage(&mut self) {
            self.hit_points -= 20;
            println!("{} has {} hit points left!", self.name, self.hit_points);
        }
    }
    let mut adventrer = Adventurer {
        name: "Me",
        hit_points: 100,
    };
    adventrer.take_damage();
    println!("{adventrer:?}");
}

#[allow(dead_code)]
pub fn ex08() {
    trait HasSomeLifeTime<'a, 'b> {}
    struct SomeStruct<'a, 'b> {
        name: &'a str,
        other: &'b str,
    }

    // impl <'a, 'b> HasSomeLifeTime<'a, 'b> for SomeStruct<'a, 'b> {}
    // impl <'one, 'two, 'three, 'four> HasSomeLifeTime<'one, 'three> for SomeStruct<'two, 'four> {}
    impl HasSomeLifeTime<'_, '_> for SomeStruct<'_, '_> {}
}

pub fn ex09() {
    struct Adventurer<'a> {
        name: &'a str,
        hit_points: u32,
    }
    impl Adventurer<'_> {
        fn take_damage(&mut self) {
            self.hit_points -= 20;
            println!("{} has {} hit points left!", self.name, self.hit_points);
        }
    }

    impl std::fmt::Display for Adventurer<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} has {} hit points.", self.name, self.hit_points)
        }
    }

    let mut billy = Adventurer {
        name: "Billy",
        hit_points: 100_000,
    };
    println!("{}", billy);
    billy.take_damage();
}
fn main() {
    ex09();
}
