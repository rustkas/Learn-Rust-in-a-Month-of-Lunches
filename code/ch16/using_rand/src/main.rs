#![allow(dead_code)]

pub fn ex01() {
    use rand::random;
    for _ in 0..5 {
        let random_u16 = random::<u16>();
        print!("{random_u16} ");
    }
}

pub fn ex02() {
    use rand::{thread_rng, Rng};

    let mut number_maker = thread_rng();
    for _ in 0..5 {
        print!("{} ", number_maker.gen_range(1..11));
    }
    println!();
}

pub fn ex03() {
    use rand::{thread_rng, Rng};
    #[derive(Debug)]
    struct Character {
        strength: u8,
        dexterity: u8,
        constitution: u8,
        intelligence: u8,
        wisdom: u8,
        charisma: u8,
    }
    #[derive(Copy, Clone)]
    enum Dice {
        Three,
        Four,
    }
    fn roll_dice(dice_choice: Dice) -> u8 {
        let mut generator = thread_rng();
        let mut total = 0;
        match dice_choice {
            Dice::Three => {
                for _ in 0..3 {
                    total += generator.gen_range(1..=6);
                }
            }
            Dice::Four => {
                let mut results = vec![];
                (0..4).for_each(|_| results.push(generator.gen_range(1..=6)));
                results.sort();
                results.remove(0);
                total += results.into_iter().sum::<u8>();
            }
        }
        total
    }
    impl Character {
        fn new(dice_choice: Dice) -> Self {
            let mut stats = (0..6).map(|_| roll_dice(dice_choice));
            Self {
                strength: stats.next().unwrap(),
                dexterity: stats.next().unwrap(),
                constitution: stats.next().unwrap(),
                intelligence: stats.next().unwrap(),
                wisdom: stats.next().unwrap(),
                charisma: stats.next().unwrap(),
            }
        }
    }

    let weak_billy = Character::new(Dice::Three);
    let strong_billy = Character::new(Dice::Four);
    println!("{weak_billy:#?}");
    println!("{strong_billy:#?}");
}

fn main() {
    ex03();
}
