#![allow(dead_code)]

pub fn ex01() {
    let value = 7;
    let reference = &7;
    println!("{}", value == *reference);
}

pub fn ex02() {
    struct HoldsANumber(u8);

    let boxed_number = Box::new(20);
    println!("This works fine: {}", *boxed_number);
    // let my_number = HoldsANumber(20);
    // println!("This fails though: {}", *my_number + 20);
}

pub fn ex03() {
    use std::ops::Deref;

    #[derive(Debug)]
    struct HoldsANumber(u8);

    impl Deref for HoldsANumber {
        type Target = u8;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    let boxed_number = Box::new(20);
    println!("This works fine: {}", *boxed_number);
    let my_number = HoldsANumber(20);
    println!("This fails though: {}", *my_number + 20);
}

pub fn ex04() {
    use std::ops::Deref;

    #[derive(Debug)]
    struct HoldsANumber(u8);

    impl HoldsANumber {
        fn prints_the_number_times_two(&self) {
            println!("{}", self.0 * 2);
        }
    }

    impl Deref for HoldsANumber {
        type Target = u8;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    let boxed_number = Box::new(20);
    println!("This works fine: {}", *boxed_number);

    let my_number = HoldsANumber(20);
    println!("{:?}", my_number.checked_sub(100));
    my_number.prints_the_number_times_two();
}

pub fn ex05() {
    use std::ops::{Deref, DerefMut};
    struct HoldsANumber(u8);
    impl HoldsANumber {
        fn prints_the_number_times_two(&self) {
            println!("{}", self.0 * 2);
        }
    }
    impl Deref for HoldsANumber {
        type Target = u8;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl DerefMut for HoldsANumber {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    let mut my_number = HoldsANumber(20);
    *my_number = 30;
    println!("{:?}", my_number.checked_sub(100));
    my_number.prints_the_number_times_two();
}

pub fn ex06() {
    #[derive(Debug)]
    struct Character {
        name: String,
        strength: u8,
        dexterity: u8,
        intelligence: u8,
        hit_points: i8,
    }
    impl Character {
        fn new(
            name: String,
            strength: u8,
            dexterity: u8,
            intelligence: u8,
            hit_points: i8,
        ) -> Self {
            Self {
                name,
                strength,
                dexterity,
                intelligence,
                hit_points,
            }
        }
    }

    let billy = Character::new("Billy".to_string(), 9, 12, 7, 10);
    println!("{billy:?}");
}

// Wrong way to implement Deref

pub fn ex0601() {
    use std::ops::{Deref, DerefMut};

    #[derive(Debug)]
    struct Character {
        name: String,
        strength: u8,
        dexterity: u8,
        intelligence: u8,
        hit_points: i8,
    }
    impl Character {
        fn new(
            name: String,
            strength: u8,
            dexterity: u8,
            intelligence: u8,
            hit_points: i8,
        ) -> Self {
            Self {
                name,
                strength,
                dexterity,
                intelligence,
                hit_points,
            }
        }
    }

    impl Deref for Character {
        type Target = i8;
        fn deref(&self) -> &Self::Target {
            &self.hit_points
        }
    }
    impl DerefMut for Character {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.hit_points
        }
    }

    let billy = Character::new("Billy".to_string(), 9, 12, 7, 10);
    println!("{billy:?}");

    let mut billy = Character::new("Billy".to_string(), 9, 12, 7, 10);
    let mut brandy = Character::new("Brandy".to_string(), 10, 8, 9, 10);
    *billy -= 10;
    *brandy += 1;
    let mut hit_points_vec = vec![];
    hit_points_vec.push(*billy);
    hit_points_vec.push(*brandy);
}
fn main() {
    ex06();
}
