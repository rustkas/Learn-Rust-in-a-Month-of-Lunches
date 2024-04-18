#![allow(dead_code)]

pub fn ex01() {
    let default_i8: i8 = Default::default();
    let default_str: String = Default::default();
    let default_bool: bool = Default::default();
    println!("'{default_i8}', '{default_str}', '{default_bool}'");
}
pub fn ex02() {
    #[derive(Debug)]
    struct Character {
        name: String,
        age: u8,
        height: u32,
        weight: u32,
        lifestate: LifeState,
    }
    #[derive(Debug)]
    enum LifeState {
        Alive,
        Dead,
        NeverAlive,
        Uncertain,
    }
    impl Character {
        fn new(name: String, age: u8, height: u32, weight: u32, alive: bool) -> Self {
            Self {
                name,
                age,
                height,
                weight,
                lifestate: if alive {
                    LifeState::Alive
                } else {
                    LifeState::Dead
                },
            }
        }
    }

    let character_1 = Character::new("Billy".to_string(), 15, 170, 70, true);

    println!("{character_1:?}");
}

pub fn ex03() {
    #[derive(Debug)]
    struct Character {
        name: String,
        age: u8,
        height: u32,
        weight: u32,
        lifestate: LifeState,
    }
    #[derive(Debug)]
    enum LifeState {
        Alive,
        Dead,
        NeverAlive,
        Uncertain,
    }
    impl Default for Character {
        fn default() -> Self {
            Self {
                name: "Billy".to_string(),
                age: 15,
                height: 170,
                weight: 70,
                lifestate: LifeState::Alive,
            }
        }
    }

    let character_1 = Character::default();

    println!("{character_1:?}");
}

pub fn ex04() {
    #[derive(Debug, Default)]
    struct Size {
        height: f64,
        length: f64,
        width: f64,
    }

    let only_height = Size {
        height: 1.0,
        ..Default::default()
    };
    println!("{only_height:?}");
}

// Builder pattern

pub fn ex05() {
    #[derive(Debug)]
    enum LifeState {
        Alive,
        Dead,
        NeverAlive,
        Uncertain,
    }

    #[derive(Debug)]
    struct Character {
        name: String,
        age: u8,
        height: u32,
        weight: u32,
        lifestate: LifeState,
    }
    impl Character {
        fn height(mut self, height: u32) -> Self {
            self.height = height;
            self
        }
        fn weight(mut self, weight: u32) -> Self {
            self.weight = weight;
            self
        }
        fn name(mut self, name: &str) -> Self {
            self.name = name.to_string();
            self
        }
    }

    impl Default for Character {
        fn default() -> Self {
            Self {
                name: "Billy".to_string(),
                age: 15,
                height: 170,
                weight: 70,
                lifestate: LifeState::Alive,
            }
        }
    }

    let character_1 = Character::default().height(180).weight(60).name("Bobby");

    println!("{character_1:?}");
}

pub fn ex06() {
    #[derive(Debug)]
    enum LifeState {
        Alive,
        Dead,
        NeverAlive,
        Uncertain,
    }

    #[derive(Debug)]
    struct Character {
        name: String,
        age: u8,
        height: u32,
        weight: u32,
        lifestate: LifeState,
        can_use: bool,
    }
    impl Character {
        fn height(mut self, height: u32) -> Self {
            self.height = height;
            self.can_use = false;
            self
        }
        fn weight(mut self, weight: u32) -> Self {
            self.weight = weight;
            self.can_use = false;
            self
        }
        fn name(mut self, name: &str) -> Self {
            self.name = name.to_string();
            self.can_use = false;
            self
        }

        fn build(mut self) -> Result<Character, String> {
            if self.height < 200 && self.weight < 300 && !self.name.to_lowercase().contains("smurf")
            {
                self.can_use = true;
                Ok(self)
            } else {
                Err("Could not create character. Characters must have:
1) Height below 200
2) Weight below 300
3) A name that is not Smurf (that is a bad word)"
                    .to_string())
            }
        }
    }

    impl Default for Character {
        fn default() -> Self {
            Self {
                name: "Billy".to_string(),
                age: 15,
                height: 170,
                weight: 70,
                lifestate: LifeState::Alive,
                can_use: true,
            }
        }
    }

    let character_with_smurf = Character::default().name("Lol I am Smurf!!").build();
    let character_too_tall = Character::default().height(400).build();
    let character_too_heavy = Character::default().weight(500).build();
    let okay_character = Character::default()
        .name("Billybrobby")
        .height(180)
        .weight(100)
        .build();
    let character_vec = vec![
        character_with_smurf,
        character_too_tall,
        character_too_heavy,
        okay_character,
    ];
    for character in character_vec {
        match character {
            Ok(character) => println!("{character:?}\n"),
            Err(err_info) => println!("{err_info}\n"),
        }
    }
}

pub fn ex07() {
    #[derive(Debug)]
    pub struct Character {
        name: String,
        age: u8,
    }
    impl Default for Character {
        fn default() -> Self {
            Self {
                name: "Billy".to_string(),
                age: 15,
            }
        }
    }
    #[derive(Debug)]
    pub struct CharacterBuilder {
        pub name: String,
        pub age: u8,
    }
    impl CharacterBuilder {
        fn new(name: String, age: u8) -> Self {
            Self { name, age }
        }
        fn try_build(self) -> Result<Character, &'static str> {
            if !self.name.to_lowercase().contains("smurf") {
                Ok(Character {
                    name: self.name,
                    age: self.age,
                })
            } else {
                Err("Can't make a character with the word 'smurf' inside it!")
            }
        }
    }

    fn do_something_with_character(_character: &Character) {}

    let default_character = Character::default();
    do_something_with_character(&default_character);
    let second_character = CharacterBuilder::new("Bobby".to_string(), 27)
        .try_build()
        .unwrap();
    do_something_with_character(&second_character);
    let bad_character = CharacterBuilder::new("Smurfysmurf".to_string(), 40).try_build();
    println!("{bad_character:?}");
    // do_something_with_character(&bad_character);
}

fn main() {
    ex07();
}
