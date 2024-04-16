#[allow(dead_code)]
pub fn ex01() {
    enum MapDirection {
        North,
        NorthEast,
        East,
        SouthEast,
        South,
        SouthWest,
        West,
        NorthWest,
    }
    fn give_direction(direction: &MapDirection) {
        match direction {
            MapDirection::North => println!("You are heading north."),
            MapDirection::NorthEast => println!("You are heading northeast."),
            _ => println!("rest"),
        }
    }
}
#[allow(dead_code)]
pub fn ex0101() {
    enum MapDirection {
        North,
        NorthEast,
        East,
        SouthEast,
        South,
        SouthWest,
        West,
        NorthWest,
    }
    fn give_direction(direction: &MapDirection) {
        use MapDirection::*;
        match direction {
            North => println!("You are heading north."),
            NorthEast => println!("You are heading northeast."),
            _ => println!("rest"),
        }
    }
}

pub fn ex02() {
    use String as S;
    let my_string: S = S::from("Hi!");
    println!(
        "{name} = {value}",
        name = stringify!(my_string),
        value = my_string
    );
}

pub fn ex03() {
    enum FileState {
        CannotAccessFile,
        FileOpenedAndReady,
        NoSuchFileExists,
        SimilarFileNameInNextDirectory,
    }

    fn give_filestate(input: &FileState) {
        use FileState::{
            CannotAccessFile as NoAccess, 
            FileOpenedAndReady as 잘됨, 
            NoSuchFileExists as NoFile,
            SimilarFileNameInNextDirectory as OtherDirectory,
        };
        match input {
            NoAccess => println!("Can't access file."),
            잘됨 => println!("Here is your file"),
            NoFile => println!("Sorry, there is no file by that name."),
            OtherDirectory => println!("Please check the other directory."),
        }
    }
    give_filestate(&FileState::CannotAccessFile);
    give_filestate(&FileState::FileOpenedAndReady);
    give_filestate(&FileState::NoSuchFileExists);
    give_filestate(&FileState::SimilarFileNameInNextDirectory);

}
fn main() {
    ex03();
}
