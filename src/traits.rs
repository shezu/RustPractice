use chrono::prelude::*;

#[derive(Debug)]
pub struct Book {
    pub author: String,
    pub publisher : String
}

pub trait BookInformation {
    fn new(&self, author : String, publisher: String) -> Self;
    fn signature_info(&self) -> String;
}

impl BookInformation for Book {

    fn new(&self, author: String, publisher: String) -> Book {
        Book{ author, publisher }
    }

    fn signature_info(&self) -> String {
        String::from(format!("author : {}, publisher : {}",self.author,self.publisher))
    }
} 

pub fn print_book_info<T : BookInformation> (item : T) {
    println!("{}",item.signature_info());
    println!("{}",Utc::now());
}