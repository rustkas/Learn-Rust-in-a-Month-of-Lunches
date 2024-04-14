mod util;

pub fn ex01() {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct MyStruct {
        number: usize,
    }
    let my_struct = MyStruct { number: 1 };
    println!("{my_struct:?}");
    println!("\"{name}\" has type {type}",name =stringify!(my_struct), type=util::get_type(&my_struct));
}

pub fn ex02() {
    use std::ops::Add;

    #[allow(dead_code)]
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct ThingsToAdd {
        first_thing: u32,
        second_thing: f32,
    }

    impl Add for ThingsToAdd {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self {
                first_thing: self.first_thing + other.first_thing,
                second_thing: self.second_thing + other.second_thing,
            }
        }
    }
    let sum = ThingsToAdd {
        first_thing: 1,
        second_thing: 0.0,
    } + ThingsToAdd {
        first_thing: 2,
        second_thing: 3.0,
    };
    let result = ThingsToAdd {
        first_thing: 3,
        second_thing: 3.0,
    };
    assert_eq!(sum, result);

    println!("\"{name}\" has type {type}",name =stringify!(result), type=util::get_type(&result));
}

pub fn ex03() {
    #[allow(dead_code)]
    struct Dog {
        name: String,
    }

    #[allow(dead_code)]
    struct Parrot {
        name: String,
    }
    trait DogLike {
        fn bark(&self) {
            println!("Woof woof!");
        }
        fn run(&self) {
            println!("The dog is running!");
        }
    }
    impl DogLike for Dog {}
    impl DogLike for Parrot {}

    impl Parrot {
        // fn run(&self) {
        //     println!("The parrot is running!");
        // }
        fn run(&self) -> i32 {
            println!("The parrot is running!");
            println!("The author can have a mistake too!");
            5
        }
    }
    let rover = Dog {
        name: "Rover".to_string(),
    };
    let brian = Parrot {
        name: "Brian".to_string(),
    };

    println!("\"{name}\" has type {type}",name =stringify!(rover), type=util::get_type(&rover));
    rover.bark();
    rover.run();

    println!("\"{name}\" has type {type}",name =stringify!(brian), type=util::get_type(&brian));
    brian.bark();
    brian.run();

    println!("\"{name}\" has type {type}",name =stringify!(brian), type=util::get_type(&brian));
}

pub fn ex04() {
    struct Parrot {
        name: String,
    }
    trait DogLike {
        fn bark(&self) {
            println!("Woof woof!");
        }
        fn run(&self) {
            println!("The dog is running!");
        }
    }

    impl Parrot {
        fn run(&self) -> &str {
            println!("{name}. Parrot's own method is running!", name = self.name);
            self.name.as_str()
        }
    }
    impl DogLike for Parrot {
        fn run(&self) {
            println!("{name}. The parrot is running!", name = self.name);
        }
    }

    let brian = Parrot {
        name: "Brian".to_string(),
    };
    brian.bark();
    brian.run();
}

pub fn ex05() {
    struct Animal {
        name: String,
    }
    trait DogLike {
        fn bark(&self);
        fn run(&self);
    }
    impl DogLike for Animal {
        fn bark(&self) {
            println!("{}, stop barking!!", self.name);
        }
        fn run(&self) {
            println!("{} is running!", self.name);
        }
    }
    impl Animal {
        fn run(&self) -> &str {
            println!("{name}. Animal's own method is running!", name = self.name);
            self.name.as_str()
        }
    }
    let rover = Animal {
        name: "Rover".to_string(),
    };
    rover.bark();
    rover.run();
    // let animal = rover as DogLike;
}

pub fn ex06() {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Cat {
        name: String,
        age: u8,
    }

    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };
    println!("Mr. Mantle is a {mr_mantle:?}");
}

pub fn ex07() {
    use std::fmt;
    struct Position {
        longitude: f32,
        latitude: f32,
    }
    impl fmt::Display for Position {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.longitude, self.latitude)
        }
    }
    let position = Position {
        longitude: 1.0,
        latitude: 1.0,
    };
    println!("{position}");
}
pub fn ex08() {
    use std::fmt;
    struct Cat {
        name: String,
        age: u8,
    }
    impl fmt::Display for Cat {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} is a cat who is {} years old.", self.name, self.age)
        }
    }
    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };
    println!("{mr_mantle}");
    println!("{cat}", cat = mr_mantle.to_string());
}

pub fn ex09() {
    use std::fmt;
    struct Cat {
        name: String,
        age: u8,
    }
    impl fmt::Display for Cat {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} is a cat who is {} years old.", self.name, self.age)
        }
    }
    fn print_excitedly(input: String) {
        println!("{input}!!!!!");
    }

    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };
    print_excitedly(mr_mantle.to_string());
    println!(
        "Mr. Mantle's String is {} letters long.",
        mr_mantle.to_string().chars().count()
    );
}

pub fn ex10() {
    use std::num::Wrapping;
    struct Monster {
        health: Wrapping<u8>,
    }
    struct Wizard;
    struct Ranger;

    trait FightClose {
        fn attack_with_sword(&self, opponent: &mut Monster) {
            opponent.health -= 10;
            println!(
                "Sword attack! Your opponent has {} health left.",
                opponent.health
            );
        }
        fn attack_with_hand(&self, opponent: &mut Monster) {
            opponent.health -= 2;
            println!(
                "Hand attack! Your opponent has {} health left.",
                opponent.health
            );
        }
    }
    impl FightClose for Wizard {}
    impl FightClose for Ranger {}
    trait FightFromDistance {
        fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
            if distance < 10 {
                opponent.health -= 10;
                println!(
                    "Bow attack! Your opponent has {} health left.",
                    opponent.health
                );
            }
        }
        fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
            if distance < 3 {
                opponent.health -= 4;
            }
            println!(
                "Rock attack! Your opponent has {} health left.",
                opponent.health
            );
        }
    }
    impl FightFromDistance for Ranger {}

    let radagast = Wizard {};
    let aragorn = Ranger {};
    let mut uruk_hai = Monster {
        health: Wrapping(40),
    };

    radagast.attack_with_sword(&mut uruk_hai);
    radagast.attack_with_hand(&mut uruk_hai);

    aragorn.attack_with_bow(&mut uruk_hai, 8);
    aragorn.attack_with_bow(&mut uruk_hai, 5);
    aragorn.attack_with_rock(&mut uruk_hai, 1);
    aragorn.attack_with_hand(&mut uruk_hai);
    aragorn.attack_with_sword(&mut uruk_hai);
}
pub fn ex11() {
    use std::fmt::Debug;
    use std::num::Wrapping;

    struct Monster {
        health: Wrapping<u8>,
    }

    trait MonsterBehavior: Debug {
        fn take_damage(&mut self, damage: i32);
        fn display_self(&self) {
            println!("The monster is now: {self:?}");
        }
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Wizard {
        health: Wrapping<u8>,
    }
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Ranger {
        health: Wrapping<u8>,
    }

    trait FightClose: Debug {
        fn attack_with_sword(&self, opponent: &mut Monster) {
            opponent.health -= 10;
            println!(
                "Sword attack! Opponent's health: {}. You are now at: {:?}",
                opponent.health, self
            );
        }
        fn attack_with_hand(&self, opponent: &mut Monster) {
            opponent.health -= 2;
            println!(
                "Hand attack! Opponent's health: {}. You are now at: {:?}",
                opponent.health, self
            );
        }
    }
    impl FightClose for Wizard {}
    impl FightClose for Ranger {}
    trait FightFromDistance {
        fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
            if distance < 10 {
                opponent.health -= 10;
                println!(
                    "Bow attack! Your opponent has {} health left.",
                    opponent.health
                );
            }
        }
        fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
            if distance < 3 {
                opponent.health -= 4;
            }
            println!(
                "Rock attack! Your opponent has {} health left.",
                opponent.health
            );
        }
    }
    impl FightFromDistance for Ranger {}

    let radagast = Wizard {
        health: Wrapping(60),
    };
    let aragorn = Ranger {
        health: Wrapping(80),
    };
    let mut uruk_hai = Monster {
        health: Wrapping(40),
    };
    radagast.attack_with_sword(&mut uruk_hai);
    aragorn.attack_with_bow(&mut uruk_hai, 8);

    radagast.attack_with_sword(&mut uruk_hai);
    radagast.attack_with_hand(&mut uruk_hai);

    aragorn.attack_with_bow(&mut uruk_hai, 8);
    aragorn.attack_with_bow(&mut uruk_hai, 5);
    aragorn.attack_with_rock(&mut uruk_hai, 1);
    aragorn.attack_with_hand(&mut uruk_hai);
    aragorn.attack_with_sword(&mut uruk_hai);
}

pub fn ex13() {
    let result: &'static str;
    result = "less than 10";
    println!("{type}", type = util::get_type(&result));
}

pub fn ex14() {
    use std::fmt::Debug;
    use std::num::Wrapping;

    #[derive(Debug)]
    struct Monster {
        health: Wrapping<u8>,
    }

    trait MonsterBehavior: Debug {
        fn take_damage(&mut self, damage: u8);
        fn display_self(&self) {
            println!("The monster is now: {self:?}");
        }
    }

    impl MonsterBehavior for Monster {
        fn take_damage(&mut self, damage: u8) {
            self.health -= damage;
        }
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Wizard {
        health: Wrapping<u8>,
    }
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Ranger {
        health: Wrapping<u8>,
    }

    trait FightClose: Debug {
        fn attack_with_sword<T: MonsterBehavior>(&self, opponent: &mut T) {
            println!("You attack with your sword!");
            opponent.take_damage(10);
            opponent.display_self();
        }
        fn attack_with_hand(&self, opponent: &mut Monster) {
            println!("You attack with your hand!");
            opponent.take_damage(2);
            opponent.display_self();
        }
    }
    impl FightClose for Wizard {}
    impl FightClose for Ranger {}
    trait FightFromDistance {
        fn attack_with_bow<T: MonsterBehavior>(&self, opponent: &mut T, distance: u32) {
            println!("You attack with your bow!");
            if distance < 10 {
                opponent.take_damage(10);
            } else {
                println!("Too far away!");
            }
            opponent.display_self();
        }
        fn attack_with_rock<T: MonsterBehavior>(&self, opponent: &mut T, distance: u32) {
            println!("You attack with a rock!");
            if distance < 3 {
                opponent.take_damage(4);
            } else {
                println!("Too far away!");
            }
            opponent.display_self();
        }
    }
    impl FightFromDistance for Ranger {}

    let radagast = Wizard {
        health: Wrapping(60),
    };
    let aragorn = Ranger {
        health: Wrapping(80),
    };
    let mut uruk_hai = Monster {
        health: Wrapping(40),
    };
    radagast.attack_with_sword(&mut uruk_hai);
    aragorn.attack_with_bow(&mut uruk_hai, 8);

    radagast.attack_with_sword(&mut uruk_hai);
    radagast.attack_with_hand(&mut uruk_hai);

    aragorn.attack_with_bow(&mut uruk_hai, 8);
    aragorn.attack_with_bow(&mut uruk_hai, 5);
    aragorn.attack_with_rock(&mut uruk_hai, 1);
    aragorn.attack_with_hand(&mut uruk_hai);
    aragorn.attack_with_sword(&mut uruk_hai);
}

pub fn ex15() {
    use std::fmt::Debug;
    struct Monster {
        health: i32,
    }
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Wizard {
        health: i32,
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Ranger {
        health: i32,
    }
    trait Magic {}
    trait FightClose {}
    trait FightFromDistance {}
    impl FightClose for Ranger {}
    impl FightClose for Wizard {}
    impl FightFromDistance for Ranger {}
    impl Magic for Wizard {}
    fn attack_with_bow<T>(pc: &T, opponent: &mut Monster, distance: u32)
    where
        T: FightFromDistance + Debug,
    {
        if distance < 10 {
            opponent.health -= 10;
            println!(
                "Bow attack! Opponent's health: {}. You are now at: {pc:?}",
                opponent.health
            );
        }
    }
    fn attack_with_sword<T>(pc: &T, opponent: &mut Monster)
    where
        T: FightClose + Debug,
    {
        opponent.health -= 10;
        println!(
            "Sword attack! Opponent's health: {}. You are now at: {pc:?}",
            opponent.health
        );
    }
    fn fireball<T>(pc: &T, opponent: &mut Monster, distance: u32)
    where
        T: Magic + Debug,
    {
        if distance < 15 {
            opponent.health -= 20;
            println!(
                "A massive fireball! Opponent's health: {}. You are now at: {pc:?}",
                opponent.health
            );
        }
    }

    let radagast = Wizard { health: 60 };
    let aragorn = Ranger { health: 80 };
    let mut uruk_hai = Monster { health: 40 };
    attack_with_sword(&radagast, &mut uruk_hai);
    attack_with_bow(&aragorn, &mut uruk_hai, 8);
    fireball(&radagast, &mut uruk_hai, 8);
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn ex16() {
    trait French {}
    trait LawyerSkill {}
    trait MedicalSkill {}

    struct FrenchCitizen;
    struct ExchangeStudentInFrance;
    struct AmericanLawyer;
    struct AmericanDoctor;
    struct FrenchLawyer;
    struct FrenchDoctor;
    struct MrKnowsEverything;

    impl French for FrenchCitizen {}
    impl French for ExchangeStudentInFrance {}
    impl French for FrenchLawyer {}
    impl French for FrenchDoctor {}
    impl French for MrKnowsEverything {}

    impl LawyerSkill for AmericanLawyer {}
    impl LawyerSkill for FrenchLawyer {}
    impl LawyerSkill for MrKnowsEverything {}

    impl MedicalSkill for AmericanDoctor {}
    impl MedicalSkill for FrenchDoctor {}
    impl MedicalSkill for MrKnowsEverything {}

    fn speak_french<T: French>(speaker: T) {}
    fn enter_court<T: LawyerSkill>(lawyer: T) {}
    fn cure_patient<T: MedicalSkill>(doctor: T) {}
    fn enter_french_court<T: LawyerSkill + French>(lawyer: T) {}
    fn cure_french_patient<T: MedicalSkill + French>(doctor: T) {}
    fn present_medical_case_in_french_court<T: MedicalSkill + French + LawyerSkill>(lawyer: T) {}

    speak_french(FrenchCitizen);
    speak_french(ExchangeStudentInFrance);
    speak_french(FrenchLawyer);
    speak_french(FrenchDoctor);
    speak_french(MrKnowsEverything);

    enter_court(AmericanLawyer);
    enter_court(FrenchLawyer);
    enter_court(MrKnowsEverything);

    cure_patient(AmericanDoctor);
    cure_patient(FrenchDoctor);
    cure_patient(MrKnowsEverything);
    enter_french_court(FrenchLawyer);
    enter_french_court(MrKnowsEverything);
    cure_french_patient(FrenchDoctor);
    cure_french_patient(MrKnowsEverything);
    present_medical_case_in_french_court(MrKnowsEverything);
    // present_medical_case_in_french_court(FrenchDoctor);

    let american_doctor = AmericanDoctor;
    cure_patient(american_doctor);

}

fn main() {
    ex16();
}
