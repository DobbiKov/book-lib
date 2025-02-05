pub mod book;
pub mod db;
pub mod help;

use rusqlite::Connection;
use std::process;

pub enum CreateBookError {
    ProvidedPathIsNotPdf,
    ProvidedPathIsIncorrect,
    BookNameAlreadyUsed,
    OtherError,
}

pub enum GetBooksError {
    BookOrTableDoesnotExist,
    NoBooks,
}

impl From<db::GetBooksError> for GetBooksError {
    fn from(value: db::GetBooksError) -> Self {
        match value {
            db::GetBooksError::BookOrTableDoesnotExist => GetBooksError::BookOrTableDoesnotExist,
            db::GetBooksError::NoBooks => GetBooksError::NoBooks,
        }
    }
}

pub enum GetBookError {
    TableOrBookDoesnotExist,
}

impl From<db::GetBookError> for GetBookError {
    fn from(value: db::GetBookError) -> Self {
        match value {
            db::GetBookError::TableOrBookDoesnotExist => GetBookError::TableOrBookDoesnotExist,
            _ => GetBookError::TableOrBookDoesnotExist,
        }
    }
}

pub enum RemoveBookError {
    BookDoesNotExist,
    Other,
}

pub enum OpenBookError {
    BookDoesNotExist,
    PathIsIncorrect,
    FileIsNotPDF,
    OtherError,
}

impl From<db::RemoveBookError> for RemoveBookError {
    fn from(value: db::RemoveBookError) -> Self {
        match value {
            db::RemoveBookError::BookDoesNotExist => RemoveBookError::BookDoesNotExist,
            db::RemoveBookError::Other => RemoveBookError::Other,
        }
    }
}

pub fn get_books(conn: &Connection) -> Result<Vec<book::Book>, GetBooksError> {
    match db::get_books(conn) {
        Ok(res) => Ok(res),
        Err(err) => Err(GetBooksError::from(err)),
    }
}
pub fn get_book(conn: &Connection, name: &String) -> Result<book::Book, GetBookError> {
    match db::get_book(conn, name) {
        Ok(res) => Ok(res),
        Err(err) => Err(GetBookError::from(err)),
    }
}
pub fn remove_book(conn: &Connection, name: &String) -> Result<book::Book, RemoveBookError> {
    match db::remove_book(conn, name) {
        Ok(res) => Ok(res),
        Err(err) => Err(RemoveBookError::from(err)),
    }
}
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
pub fn open_book(conn: &Connection, name: String) -> Result<(), OpenBookError> {
    let bk_res = get_book(conn, &name);
    match bk_res {
        Ok(bk) => {
            let path = bk.path;
            if !help::is_pdf(&path) {
                return Err(OpenBookError::FileIsNotPDF);
            }
            let (is_correct, _) = help::is_correct_path(&path);
            if !is_correct {
                return Err(OpenBookError::PathIsIncorrect);
            }
            process::Command::new("open")
                .args(["-a", "Skim", path.as_str()])
                .output()
                .expect("error while opening the file with Skim");
            Ok(())
        }
        Err(err) => match err {
            GetBookError::TableOrBookDoesnotExist => Err(OpenBookError::BookDoesNotExist),
        },
    }
}
