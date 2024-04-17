pub fn ex01() {
    type CharacterVec = Vec<char>;
    let mut vector: CharacterVec = CharacterVec::new();
    vector.push('a');
    vector.push('b');

    println!("{vector:?}");
}
pub fn ex02() {
    type CharacterVec = Vec<char>;
    let mut vector: CharacterVec = CharacterVec::new();
    for value in 'a'..'z' {
        vector.push(value);
    }

    fn returns_some_chars(
        input: Vec<char>,
    ) -> std::iter::Take<std::iter::Skip<std::vec::IntoIter<char>>> {
        input.into_iter().skip(4).take(5)
    }
    let chars: CharacterVec = returns_some_chars(vector).collect();
    println!("{chars:?}");
}

pub fn ex0201() {
    type CharacterVec = Vec<char>;
    let mut vector: CharacterVec = CharacterVec::new();
    for value in 'a'..'z' {
        vector.push(value);
    }

    type SkipFourTakeFive = std::iter::Take<std::iter::Skip<std::vec::IntoIter<char>>>;

    fn returns_some_chars(input: Vec<char>) -> SkipFourTakeFive {
        input.into_iter().skip(4).take(5)
    }
    let chars: CharacterVec = returns_some_chars(vector).collect();
    println!("{chars:?}");
}

pub fn ex03() {
    type CharacterVec = Vec<char>;
    let mut vector: CharacterVec = CharacterVec::new();
    for value in 'a'..'z' {
        vector.push(value);
    }

    use std::iter::{Skip, Take};
    use std::vec::IntoIter;
    fn returns_some_chars(input: Vec<char>) -> Take<Skip<IntoIter<char>>> {
        input.into_iter().skip(4).take(5)
    }

    let chars: CharacterVec = returns_some_chars(vector).collect();
    println!("{chars:?}");
}

pub fn ex04() {
    type File = String;
    let my_file = File::from("I am file contents");
    let my_string = String::from("I am file contents");
    println!("{name}   = {value}", name = stringify!(my_file), value=my_file);
    println!("{name} = {value}", name = stringify!(my_string), value=my_string);
    println!("{name1} == {name2} is {result}", name1=stringify!(my_file), name2=stringify!(my_string),  result = (my_file == my_string));
}

fn main() {
    ex04();
}
