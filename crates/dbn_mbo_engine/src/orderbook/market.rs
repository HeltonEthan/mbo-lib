use dbn::{MboMsg, Publisher, Record};
use std::collections::HashMap;

use crate::orderbook::book::Book;

#[derive(Debug, Default)]
pub struct Market {
    books: HashMap<u32, Vec<(Publisher, Book)>>,
}

impl Market {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply(&mut self, mbo: MboMsg) {
        let publisher = mbo.publisher().unwrap();
        let books = self.books.entry(mbo.hd.instrument_id).or_default();
        let book = if let Some((_, book)) = books.iter_mut().find(|(book_pub, _)| *book_pub == publisher) {
            book
        } else {
            books.push((publisher, Book::default()));
            &mut books.last_mut().unwrap().1
        };
        book.apply(mbo);
    }
}
