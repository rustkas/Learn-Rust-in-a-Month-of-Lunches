mod util;

pub fn ex01() {
    struct Person {
        name: String,
        real_name: String,
        height: u8,
        happiness: bool,
    }

    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false,
    };
    let Person {
        name,
        real_name,
        height,
        happiness,
    } = papa_doc;

    let is_happy_text = if happiness { "True" } else { "False" };
    println!(
        "They call him \"{name}\" but his real name is \"{real_name}\".
He is {height} cm tall and is he happy? {is_happy_text}."
    );
}

pub fn ex02() {
    struct Person {
        name: String,
        real_name: String,
        height: u8,
        happiness: bool,
    }

    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false,
    };
    let Person {
        name: fake_name,
        real_name,
        height: cm,
        happiness,
    } = papa_doc;

    let is_happy_text = if happiness { "True" } else { "False" };
    println!(
        "They call him \"{fake_name}\" but his real name is \"{real_name}\".
He is {cm} cm tall and is he happy? {is_happy_text}."
    );
}

pub fn ex03() {
    struct City {
        name: String,
        name_before: String,
        population: u32,
        date_founded: u32,
    }

    #[allow(unused_variables)]
    impl City {
        fn new(name: &str, name_before: &str, population: u32, date_founded: u32) -> Self {
            Self {
                name: String::from(name),
                name_before: String::from(name_before),
                population,
                date_founded,
            }
        }
        fn print_names(&self) {
            let City {
                name,
                name_before,
                population,
                date_founded,
            } = self;
            println!("The city {name} used to be called {name_before}. Population is {population}. The date of foundation is {date_founded}");
        }
    }

    let tallinn = City::new("Tallinn", "Reval", 426_538, 1219);
    tallinn.print_names();
}

pub fn ex04() {
    #[allow(dead_code)]
    struct City {
        name: String,
        name_before: String,
        population: u32,
        date_founded: u32,
    }

    #[allow(unused_variables)]
    impl City {
        fn new(name: &str, name_before: &str, population: u32, date_founded: u32) -> Self {
            Self {
                name: String::from(name),
                name_before: String::from(name_before),
                population,
                date_founded,
            }
        }
        fn print_names(&self) {
            let City {
                name, name_before, ..
            } = self;
            println!("The city {name} used to be called {name_before}.");
        }
    }

    let tallinn = City::new("Tallinn", "Reval", 426_538, 1219);
    tallinn.print_names();
}

pub fn ex05() {
    #[allow(dead_code)]
    struct Person {
        name: String,
        real_name: String,
        height: u8,
        happiness: bool,
    }
    fn check_if_happy(person: &Person) {
        println!("Is {} happy? {}", person.name, person.happiness);
    }
    fn check_if_happy_destructured(Person { name, happiness, .. }: &Person) {
        println!("Is {name} happy? {happiness}");
    }

    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false,
    };
    check_if_happy(&papa_doc);
    check_if_happy_destructured(&papa_doc);
}

fn main() {
    ex05();
}
