use crate::book::Book;
use crate::member::Member;

pub struct Borrowing {
    pub book: Book,
    pub member: Member,
    pub borrow_date: String
}

impl Borrowing {
    pub fn new(book: Book, member: Member, borrow_date: String) -> Borrowing {
        Borrowing {
            book,
            member,
            borrow_date
        }
    }

    pub fn display_info(self) {
        println!("{}", self.book.title);
        println!("{}", self.member.name);
        println!("{}", self.borrow_date);
    }
}