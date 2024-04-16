#[allow(dead_code, unused_variables)]
pub fn ex01() {
    // Okay, first I need a book struct.
    // Nothing in there yet - will add later
    struct Book;
    // A book can be hardcover or softcover, so add an enum…
    enum BookType {
        HardCover,
        SoftCover,
    }
    // should take a &Book and return an Option<String>
    fn get_book(book: &Book) -> Option<String> {
        Some("".to_string())
    }
    // should take a ref Book and return a Result...
    fn delete_book(book: &Book) -> Result<(), String> {
        Ok(())
    }
    // TODO: impl block and make these functions methods…
    // TODO: make this a proper error
    fn check_book_type(book_type: &BookType) {
        // Let's make sure the match statement works
        match book_type {
            BookType::HardCover => println!("It's hardcover"),
            BookType::SoftCover => println!("It's softcover"),
        }
    }

    let book_type = BookType::HardCover;
    // Okay, let's check this function!
    check_book_type(&book_type);
}

#[allow(dead_code, unused_variables, unused_must_use)]
pub fn ex02() {
    // Okay, first I need a book struct.
    // Nothing in there yet - will add later
    struct Book;
    // A book can be hardcover or softcover, so add an enum…
    enum BookType {
        HardCover,
        SoftCover,
    }
    // should take a &Book and return an Option<String>
    fn get_book(book: &Book) -> Option<String> {
        todo!();
    }

    // fn get_book2(book: &Book) -> WorldsBestType {
    //     todo!()
    // }

    // should take a ref Book and return a Result...
    fn delete_book(book: &Book) -> Result<(), String> {
        todo!();
    }
    // TODO: impl block and make these functions methods…
    // TODO: make this a proper error
    fn check_book_type(book_type: &BookType) {
        // Let's make sure the match statement works
        match book_type {
            BookType::HardCover => println!("It's hardcover"),
            BookType::SoftCover => println!("It's softcover"),
        }
    }

    let book_type = BookType::HardCover;
    // Okay, let's check this function!
    check_book_type(&book_type);
    // get_book(&Book);
    // delete_book(&Book);
}

#[allow(dead_code,unreachable_code)]
pub fn ex03() {
    struct Book {
        name: String,
        year: u8,
    }
    fn make_book() -> Book {
        Book {
            name: todo!(),
            year: todo!(),
        }
    }
    // make_book();
}

fn main() {
    ex03();
}
