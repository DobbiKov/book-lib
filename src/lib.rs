//! # book_lib
//! A main module for the book managing library.
//! The main goal of the library is to see the PDF files from all the filesystem on your PC in one
//! place.
//! The library provides an API for
//! creating/updating/deleting a "book" that represents a PDF by it's path and name.
//!
//! The library uses sqlite as a database manager and stores the data in the
//! $HOME/.config/book-cli/books.db
//!
//! ## Usage
//! 1. Create a connection to the database:
//! ```rust
//! use book_lib::{db, book};
//!
//! let connection = book_lib::db::setup();
//!
//! ```
//!
//! 2. Create a new book
//! ```rust
//! let new_book = book::Book::init("book_name".to_string(), "path_to/your/file.pdf".to_string(), None, false);
//! book_lib::create_book(&connection, &new_book); //creating new book in the DB
//! ```
//!
//! 3. Open your book
//! ```rust
//! book_lib::open_book(&connection, &"book_name".to_string()); // open the book by the default PDF viewer
//! ```
//!
//! 4. Make it favourite
//! ```rust
//! book_lib::update_favourite(&connection, &("book_name".to_string()), true); //true to be favourite, false not to be
//! ```
//!
//! 5. Remove the book
//! ```rust
//! book_lib::remove_book(&connection, &("book_name".to_string()));
//! ```
//!
//! ## Examples of implementation
//! ## Examples of implementation
//! 1. [cli for managing PDFs](https://github.com/DobbiKov/book-cli)
//! 2. [GUI for managing PDFs](https://github.com/DobbiKov/book-manager-app)

pub mod book;
pub mod db;
pub mod errors;
pub mod help;

use errors::{
    CreateBookError, GetBookError, GetBooksError, OpenBookError, RemoveBookError,
    UpdateFavouriteError,
};
use rusqlite::Connection;
use std::process;

/// Returns all the books stored in the database or an error.
pub fn get_books(conn: &Connection) -> Result<Vec<book::Book>, GetBooksError> {
    match db::get_books(conn) {
        Ok(res) => Ok(res),
        Err(err) => Err(GetBooksError::from(err)),
    }
}

/// Returns a book by given name or an error.
pub fn get_book(conn: &Connection, name: &String) -> Result<book::Book, GetBookError> {
    match db::get_book(conn, name) {
        Ok(res) => Ok(res),
        Err(err) => Err(GetBookError::from(err)),
    }
}

/// Removes a book by the given name or an error.
pub fn remove_book(conn: &Connection, name: &String) -> Result<book::Book, RemoveBookError> {
    match db::remove_book(conn, name) {
        Ok(res) => Ok(res),
        Err(err) => Err(RemoveBookError::from(err)),
    }
}

/// Creates a book by the given book data.
pub fn create_book(conn: &Connection, bk: &book::Book) -> Result<bool, CreateBookError> {
    if !help::is_pdf(&bk.path) {
        return Err(CreateBookError::ProvidedPathIsNotPdf);
    }
    let (is_correct, _) = help::is_correct_path(&bk.path);
    if !is_correct {
        return Err(CreateBookError::ProvidedPathIsIncorrect);
    }
    match db::create_book(conn, bk) {
        Ok(_) => Ok(true),
        Err(err) => match err {
            db::CreateBookError::BookWithNameExists => Err(CreateBookError::BookNameAlreadyUsed),
            _ => Err(CreateBookError::OtherError),
        },
    }
}

// Opens a book by the given name or returns an error.
//pub fn open_book(conn: &Connection, name: &String) -> Result<(), OpenBookError> {
//    // TODO: redo it
//    // for any OS
//    let bk_res = get_book(conn, name);
//    match bk_res {
//        Ok(bk) => {
//            let path = bk.path;
//            if !help::is_pdf(&path) {
//                return Err(OpenBookError::FileIsNotPDF);
//            }
//            let (is_correct, _) = help::is_correct_path(&path);
//            if !is_correct {
//                return Err(OpenBookError::PathIsIncorrect);
//            }
//            process::Command::new("open")
//                .args(["-a", "Skim", path.as_str()])
//                .output()
//                .expect("error while opening the file with Skim");
//            Ok(())
//        }
//        Err(err) => match err {
//            GetBookError::TableOrBookDoesnotExist => Err(OpenBookError::BookDoesNotExist),
//        },
//    }
//}

/// Update the books favourite state by the book's name.
pub fn update_favourite(
    conn: &Connection,
    name: &String,
    favourite: bool,
) -> Result<book::Book, UpdateFavouriteError> {
    match db::update_favourite_error(conn, name, favourite) {
        Ok(book) => Ok(book),
        Err(err) => Err(UpdateFavouriteError::from(err)),
    }
}
