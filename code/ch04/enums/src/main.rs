mod util;

pub fn ex01() {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum Climate {
        Tropical,
        Dry,
        Temperate,
        Continental,
        Polar,
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Country {
        population: u32,
        capital: String,
        leader_name: String,
        climate: Climate,
    }

    let kalmykia = Country {
        population: 500_000,
        capital: String::from("Elista"),
        leader_name: String::from("Batu Khasikov"),
        climate: Climate::Continental,
    };
    println!(
        "{variable_name} = {kalmykia:?}",
        variable_name = stringify!(kalmykia)
    );
    println!("Type of {variable_name} = {type}", variable_name = stringify!(kalmykia), type = crate::util::get_type(&kalmykia));
}

pub fn ex02() {
    #[derive(Debug)]
    enum ThingsInTheSky {
        Sun,
        Stars,
    }
    fn create_skystate(time: i32) -> ThingsInTheSky {
        match time {
            6..=18 => ThingsInTheSky::Sun,
            _ => ThingsInTheSky::Stars,
        }
    }

    fn check_skystate(state: &ThingsInTheSky) {
        match state {
            ThingsInTheSky::Sun => println!("I can see the sun!"),
            ThingsInTheSky::Stars => println!("I can see the stars!"),
        }
    }

    let time = 8;
    let skystate = create_skystate(time);
    check_skystate(&skystate);

    println!(
        "{variable_name} = {skystate:?}",
        variable_name = stringify!(skystate)
    );
    println!("Type of {variable_name} = {type}", variable_name = stringify!(skystate), type = crate::util::get_type(&skystate));
}

pub fn ex03() {
    #[derive(Debug)]
    enum ThingsInTheSky {
        Sun(String),
        Stars(String),
    }

    fn create_skystate(time: i32) -> ThingsInTheSky {
        match time {
            6..=18 => ThingsInTheSky::Sun(String::from("I can see the sun!")),
            _ => ThingsInTheSky::Stars(String::from("I can see the stars!")),
        }
    }

    fn check_skystate(state: &ThingsInTheSky) {
        match state {
            ThingsInTheSky::Sun(description) => println!("{description}"),
            ThingsInTheSky::Stars(n) => println!("{n}"),
        }
    }

    let time = 100;
    let skystate = create_skystate(time);
    check_skystate(&skystate);

    println!(
        "{variable_name} = {skystate:?}",
        variable_name = stringify!(skystate)
    );
    println!("Type of {variable_name} = {type}", variable_name = stringify!(skystate), type = crate::util::get_type(&skystate));
}

pub fn ex04() {
    #[derive(Debug)]
    #[allow(dead_code)]
    enum Mood {
        Happy,
        Sleepy,
        NotBad,
        Angry,
    }
    fn match_mood(mood: &Mood) -> i32 {
        let happiness_level = match mood {
            Mood::Happy => 10,
            Mood::Sleepy => 6,
            Mood::NotBad => 7,
            Mood::Angry => 2,
        };
        happiness_level
    }

    let array = [Mood::NotBad, Mood::Sleepy, Mood::NotBad, Mood::Angry];
    for my_mood in array {
        let happiness_level = match_mood(&my_mood);
        println!(
            "{my_mood:?}. Out of 1 to 10, my happiness is {happiness_level}",
            my_mood = my_mood
        );
    }
}

pub fn ex05() {
    #[derive(Debug)]
    #[allow(dead_code)]
    enum Mood {
        Happy,
        Sleepy,
        NotBad,
        Angry,
    }
    fn match_mood(mood: &Mood) -> i32 {
        use Mood::*;
        let happiness_level = match mood {
            Happy => 10,
            Sleepy => 6,
            NotBad => 7,
            Angry => 2,
        };
        happiness_level
    }

    let array = [Mood::NotBad, Mood::Sleepy, Mood::NotBad, Mood::Angry];
    for my_mood in array {
        let happiness_level = match_mood(&my_mood);
        println!(
            "{my_mood:?}. Out of 1 to 10, my happiness is {happiness_level}",
            my_mood = my_mood
        );
    }
}

pub fn ex06() {
    use std::mem::size_of_val;
    let jaurim = "자우림";
    let adrian = "Adrian Fahrenheit ?epe?";
    let size_of_jaurim = size_of_val(jaurim);
    let size_of_adrian = size_of_val(adrian);
    println!("Size of \"{jaurim}\" = {size_of_jaurim}, size of \"{adrian}\" = {size_of_adrian}");
}

pub fn ex07() {
    #[derive(Debug, Clone)]
    enum Season {
        Spring,
        Summer,
        Autumn,
        Winter,
    }

    use Season::*;
    let four_seasons = [Spring, Summer, Autumn, Winter];
    for season in four_seasons {
        let index = season.clone() as u8;
        let name = season.clone();

        println!("{name:?} has index = {index}");
    }
}

pub fn ex08() {
    #[derive(Debug, Clone)]
    enum Star {
        BrownDwarf = 10,
        RedDwarf = 50,
        YellowStar = 100,
        RedGiant = 1000,
        DeadStar,
    }

    use Star::*;
    let starvec = [BrownDwarf, RedDwarf, YellowStar, RedGiant, DeadStar];
    for star in starvec {
        let index = star.clone() as u32;
        let name = star.clone();

        println!("{name:?} has index = {index}");
        println!("\tResult of checking star size:");
        match star as u32 {
            size if size <= 80 => println!("\t\tNot the biggest star."),
            size if size >= 80 && size <= 200 => println!("\t\tThis is a good-sized star."),
            other_size => println!("\t\tThat star is pretty big! It's {other_size}"),
        }
    }
    println!("");
}

pub fn ex09() {
    let vec01 = Vec::<u32>::new();
    let vec02 = Vec::<i32>::new();
    println!("Type of {variable_name} = {type}", variable_name = stringify!(vec01), type = crate::util::get_type(&vec01));
    println!("Type of {variable_name} = {type}", variable_name = stringify!(vec02), type = crate::util::get_type(&vec02));

    #[derive(Debug)]
    #[allow(dead_code)]
    enum Number {
        U32(u32),
        I32(i32),
    }
    let mut vec03 = Vec::<Number>::new();
    vec03.push(Number::I32(0));
    vec03.push(Number::U32(0));
    println!(
        "{variable_name} = {value:?}",
        variable_name = stringify!(vec03),
        value = vec03
    );
    println!("Type of {variable_name} = {type}", variable_name = stringify!(vec03), type = crate::util::get_type(&vec03));
    println!("");
    fn get_number(input: i32) -> Number {
        let number = match input.is_positive() {
            true => Number::U32(input as u32),
            false => Number::I32(input),
        };
        number
    }

    let my_vec = [get_number(-800), get_number(8)];

    println!(
        "{variable_name} = {value:?}",
        variable_name = stringify!(my_vec),
        value = my_vec
    );
    println!("Type of {variable_name} = {type}", variable_name = stringify!(my_vec), type = crate::util::get_type(&my_vec));

    for item in my_vec {
        match item {
            Number::U32(number) => println!("A u32 with the value {number}"),
            Number::I32(number) => println!("An i32 with the value {number}"),
        }
    }
}

pub fn ex10() {
    let mut my_string = String::from("I feel excited");
    my_string.push('!');

    println!(
        "{variable_name} = {value:?}",
        variable_name = stringify!(my_string),
        value = my_string
    );
    println!("Type of {variable_name} = {type}", variable_name = stringify!(my_string), type = crate::util::get_type(&my_string));
}
pub fn ex11() {
    #[derive(Debug)]
    #[allow(dead_code)]
    enum Mood {
        Good,
        Bad,
        Sleepy,
    }
    #[derive(Debug)]
    #[allow(dead_code)]
    enum AnimalType {
        Cat,
        Dog,
    }
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Animal {
        age: u8,
        animal_type: AnimalType,
        mood: Mood,
    }

    #[allow(dead_code)]
    impl Animal {
        fn new_cat(age: u8) -> Self {
            Self {
                age: age,
                animal_type: AnimalType::Cat,
                mood: Mood::Good,
            }
        }
        fn new_dog(age:u8) -> Self {
            Self {
                age: age,
                animal_type: AnimalType::Dog,
                mood: Mood::Good,
            }
        }
        fn check_type(&self) {
            match self.animal_type {
                AnimalType::Dog => println!("The animal is a dog"),
                AnimalType::Cat => println!("The animal is a cat"),
            }
        }
        fn change_to_dog(&mut self) {
            self.animal_type = AnimalType::Dog;
            println!("Changed animal to dog! Now it's {self:?}");
        }
        fn change_to_cat(&mut self) {
            self.animal_type = AnimalType::Cat;
            println!("Changed animal to cat! Now it's {self:?}");
        }
    }

    let mut new_animal = Animal::new_cat(10);
    new_animal.check_type();
    new_animal.change_to_dog();
    new_animal.check_type();
    new_animal.change_to_cat();
    new_animal.check_type();
}

pub fn ex12() {
    #[derive(Debug)]
    #[allow(dead_code)]
    enum Mood {
        Good,
        Bad,
        Sleepy,
    }
    use Mood::*;
    impl Mood {
        fn check(&self) {
            match self {
                Good => println!("Feeling good!"),
                Bad => println!("Eh, not feeling so good"),
                Sleepy => println!("Need sleep NOW"),
            }
        }
    }

    let mut my_mood = Sleepy;
    my_mood.check();
    my_mood = Good;
    my_mood.check();
    my_mood = Bad;
    my_mood.check();
    my_mood = Sleepy;
    println!(
        "{variable_name} = {value:?}",
        variable_name = stringify!(my_mood),
        value = my_mood
    );
    println!("Type of {variable_name} = {type}", variable_name = stringify!(my_mood), type = crate::util::get_type(&my_mood));
}

fn main() {
    ex11();
}
