pub fn ex01() {
    let mut new_vec = Vec::new();
    let mut counter = 1;
    loop {
        new_vec.push(counter);
        counter += 1;
        if counter == 10 {
            break;
        }
    }
    println!("{new_vec:?}");
}

pub fn ex02() {
    let new_vec = (1..).take(10).collect::<Vec<i32>>();
    println!("{new_vec:?}");

    let new_vec: Vec<i32> = (1..).take(10).collect();
    println!("{new_vec:?}");
}

pub fn ex03() {
    let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let new_vec = my_vec
        .clone()
        .into_iter()
        .skip(3)
        .take(4)
        .collect::<Vec<i32>>();
    println!("{name} = {my_vec:?}", name = stringify!(my_vec));

    println!("{name} = {new_vec:?}", name = stringify!(new_vec));
}

pub fn ex0301() {
    // let my_vec = [&0,&1,&2,&3,&4,&4,&6,&7,&8,&9,&10].to_vec();
    let my_vec = (0..10).take(10).collect::<Vec<i32>>();
    let binding = my_vec.clone();
    let new_vec = binding.iter().skip(3).take(4).collect::<Vec<&i32>>();
    println!("{name} = {my_vec:?}", name = stringify!(my_vec));

    println!("{name} = {new_vec:?}", name = stringify!(new_vec));
}

pub fn ex0302() {
    // let my_vec = [&0,&1,&2,&3,&4,&4,&6,&7,&8,&9,&10].to_vec();
    let my_vec = (0..10).take(10).collect::<Vec<i32>>();
    let mut binding = my_vec.clone();
    let new_vec = binding
        .iter_mut()
        .skip(3)
        .take(4)
        .collect::<Vec<&mut i32>>();
    println!("{name} = {my_vec:?}", name = stringify!(my_vec));

    println!("{name} = {new_vec:?}", name = stringify!(new_vec));
}

pub fn ex04() {
    let vector1 = vec![1, 2, 3];
    let mut vector2 = vec![10, 20, 30];
    println!("{name}:", name = stringify!(vector1));
    println!("\tPrinting a &i32:");
    for num in vector1.iter() {
        println!("Printing a &i32: {num}");
    }
    println!("\tPrinting a i32:");
    for num in vector1 {
        println!("Printing an i32: {num}");
    }
    println!("{name}:", name = stringify!(vector2));
    println!("\tPrinting a &mut i32:");
    for num in vector2.iter_mut() {
        *num *= 10;
        println!("num is now {num}");
    }
    println!("{vector2:?}");
    // println!("{vector1:?}");
}

pub fn ex05() {
    let vector1 = vec![1, 2, 3];
    let vector1_a = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>();
    let vector1_b = vector1.into_iter().map(|x| x * 10).collect::<Vec<i32>>();
    let mut vector2 = vec![10, 20, 30];
    vector2.iter_mut().for_each(|x| *x += 100);
    println!("{:?}", vector1_a);
    println!("{:?}", vector1_b);
    println!("{:?}", vector2);
}

pub fn ex06() {
    let my_vec = vec!['a', 'b', '거', '柳'];
    let mut my_vec_iter = my_vec.iter();
    assert_eq!(my_vec_iter.next(), Some(&'a'));
    assert_eq!(my_vec_iter.next(), Some(&'b'));
    assert_eq!(my_vec_iter.next(), Some(&'거'));
    assert_eq!(my_vec_iter.next(), Some(&'柳'));
    assert_eq!(my_vec_iter.next(), None);
    assert_eq!(my_vec_iter.next(), None);
}

#[allow(dead_code)]
pub fn ex07() {
    #[derive(Debug)]
    struct Library {
        name: String,
        books: Vec<String>,
    }
    impl Library {
        fn add_book(&mut self, book: &str) {
            self.books.push(book.to_string());
        }
        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                books: Vec::new(),
            }
        }
    }
    let my_library = Library::new("Calgary");
    println!("{my_library:?}");
}

#[allow(dead_code)]
pub fn ex08() {
    #[derive(Debug)]
    struct Library {
        name: String,
        books: BookCollection,
    }
    #[derive(Debug, Clone)]
    struct BookCollection(Vec<String>);
    impl Library {
        fn add_book(&mut self, book: &str) {
            self.books.0.push(book.to_string());
        }
        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                books: BookCollection(Vec::new()),
            }
        }
        fn get_books(&self) -> BookCollection {
            self.books.clone()
        }
    }

    impl Iterator for BookCollection {
        type Item = String;
        fn next(&mut self) -> Option<Self::Item> {
            match self.0.pop() {
                Some(book) => {
                    println!("Accessing book: {book}");
                    Some(book)
                }
                None => {
                    println!("Out of books at the library!");
                    None
                }
            }
        }
    }

    let mut my_library = Library::new("Calgary");
    my_library.add_book("The Doom of the Darksword");
    my_library.add_book("Demian - die Geschichte einer Jugend");
    my_library.add_book("구운몽");
    my_library.add_book("吾輩は猫である");
    for item in my_library.get_books() {
        println!("Now print its name: {item}");
    }
}

pub fn ex09() {
    struct GivesOne;
    impl Iterator for GivesOne {
        type Item = i32;
        fn next(&mut self) -> Option<i32> {
            Some(1)
        }
    }
    let five_ones: Vec<i32> = GivesOne.into_iter().take(5).collect();
    println!("{five_ones:?}");
}
fn main() {
    ex09();
}
