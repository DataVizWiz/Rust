pub struct Book {
    pub title: String,
    pub author: String,
    pub year_published: u32
}

impl Book {
    pub fn new(title: &str, author: &str, year_published: u32) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            year_published
        }   
    }
}