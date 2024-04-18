#![allow(dead_code)]
const OKAY_CHARACTERS: &str = "1234567890- ";

#[derive(Default)]
enum Operation {
    #[default]
    Add,
    Subtract,
}

#[derive(Default)]
struct Subtractor {
    total: i32,
    num_to_parse: String,
    operation: Operation,
}

impl Subtractor {
    fn switch_operation(&mut self) {
        self.operation = match self.operation {
            Operation::Add => Operation::Subtract,
            Operation::Subtract => Operation::Add,
        }
    }

    fn do_operation(&mut self) {
        let num = self.num_to_parse.parse::<i32>().unwrap();
        match self.operation {
            Operation::Add => self.total += num,
            Operation::Subtract => self.total -= num,
        }
        self.operation = Operation::Add;
        self.num_to_parse.clear();
    }

    fn math(&mut self, input: &str) -> i32 {
        if input
            .chars()
            .any(|character| !OKAY_CHARACTERS.contains(character))
        {
            panic!("Please only input numbers, -, or spaces.");
        }
        let input = input
            .trim_end_matches(|x| "- ".contains(x))
            .chars()
            .filter(|x| *x != ' ')
            .collect::<String>();
        for character in input.chars() {
            match character {
                '-' => {
                    if !self.num_to_parse.is_empty() {
                        self.do_operation();
                    }
                    self.switch_operation();
                }
                number => self.num_to_parse.push(number),
            }
        }
        if !self.num_to_parse.is_empty() {
            self.do_operation();
        }
        self.total
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one_minus_two_is_minus_one() {
        let mut calc = Subtractor::default();
        assert_eq!(calc.math("1 - 2"), -1);
    }
    #[test]
    fn one_minus_minus_one_is_two() {
        let mut calc = Subtractor::default();
        assert_eq!(calc.math("1 - -1"), 2);
    }
    #[test]
    fn three_minus_three_minus_three_minus_minus_three_is_zero() {
        let mut calc = Subtractor::default();
        assert_eq!(calc.math("3-3-3--3"), 0);
    }
    #[test]
    fn eighteen_minus_nine_minus_nine_is_zero_even_with_characters_on_the_end() {
        let mut calc = Subtractor::default();
        assert_eq!(calc.math("18 - 9 -9-----"), 0);
    }
    #[test]
    #[should_panic]
    fn panics_when_characters_not_right() {
        let mut calc = Subtractor::default();
        calc.math("7 - seven");
    }
}
