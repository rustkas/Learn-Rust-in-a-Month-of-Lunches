pub fn ex01() {
    let array_vec = Vec::from([8, 9, 10]);
    println!("Vec from array: {array_vec:?}");
    let str_vec = Vec::from("What kind of Vec am I?");
    println!("Vec from str: {str_vec:?}");
    let string_vec = Vec::from("What will a String be?".to_string());
    println!("Vec from String: {string_vec:?}");
}

pub fn ex02() {
    #[derive(Debug)]
    struct City {
        name: String,
        population: u32,
    }
    impl City {
        fn new(name: &str, population: u32) -> Self {
            Self {
                name: name.to_string(),
                population,
            }
        }
    }

    #[derive(Debug)]
    struct Country {
        cities: Vec<City>,
    }

    impl From<Vec<City>> for Country {
        fn from(cities: Vec<City>) -> Self {
            Self { cities }
        }
    }
    impl Country {
        fn print_cities(&self) {
            for city in &self.cities {
                println!("{:?} has a population of {:?}.", city.name, city.population);
            }
        }
    }

    let helsinki = City::new("Helsinki", 631_695);
    let turku = City::new("Turku", 186_756);

    let finland_cities = vec![helsinki, turku];
    let finland = Country::from(finland_cities);
    finland.print_cities();
}

#[allow(dead_code)]
#[warn(unused_variables)]
pub fn ex03() {
    #[derive(Clone, Debug)]
    struct File(String);

    impl std::fmt::Display for File {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let as_bytes = format!("{:?}", self.0.as_bytes());
            write!(f, "{as_bytes}")
        }
    }

    let my_file = File(String::from("I am file contents"));
    let my_string = String::from("I am file contents");
    // println!("{}", my_file == my_string);
    println!("{}", my_file.0 == my_string);

    let file = File(String::from("I am file contents"));
    println!("{file:?}");
    println!("{file}");
    println!();
}

fn main() {
    ex03();
}
