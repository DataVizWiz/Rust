// Bring the modules into scope
use modules::book::Book;
use modules::member::Member;
use modules::borrowing::Borrowing;

fn main() {
    // Creating a new Book instance using the `new` method
    let book = Book::new("The Rust Programming Language", "Steve Klabnik and Carol Nichols", 2018);
    
    // Creating a new Member instance using the `new` method
    let member = Member::new("Metin", 1);

    println!("Book: '{}' by {}, published in {}", book.title, book.author, book.year_published);
    println!("Member: {} with ID {}", member.name, member.member_id);

    let borrow = Borrowing::new(book, member, String::from("30-08-2024"));
    borrow.display_info();
}