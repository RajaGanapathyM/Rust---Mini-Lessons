#[derive(Debug)]
struct Book {
    title: String,
    borrower: Option<String>,
}

fn borrow_book(book: &mut Book, borrower: &str) -> bool {
    if book.borrower.is_none() {
        book.borrower = Some(borrower.to_string());
        true
    } else {
        false
    }
}

fn return_book(book: &mut Book) {
    book.borrower = None;
}

fn who_borrowed(book: &Book) -> Option<&str> {
    book.borrower.as_deref()
}

fn main() {
    let mut book = Book {
        title: String::from("The Rust Programming Language"),
        borrower: None,
    };

    if borrow_book(&mut book, "Booker") {
        println!("{} has been borrowed by {:?}", book.title,book.borrower);
    } else {
        println!("{} is already borrowed.", book.title);
    }

    if let Some(name) = who_borrowed(&book) {
        println!("{} is currently borrowed by {}.", book.title, name);
    }

    return_book(&mut book);
    println!("Book returned. Current State:{:?} ", book);
}
