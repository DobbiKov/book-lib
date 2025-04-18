//! A module provides the most essential function for managing books in the database

use dirs;
use loggit::debug;
use std::io;

use crate::book;
use rusqlite::{params, Connection, Result};

pub enum CreateBookError {
    BookWithNameExists,
    Other,
}

impl std::fmt::Display for CreateBookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateBookError::BookWithNameExists => write!(f, "A book with the same name exists"),
            CreateBookError::Other => write!(f, "Unexpected error"),
        }
    }
}

pub struct DbConfig {
    pub path_to_db: String,
}

fn create_folder_if_not_exist(path: &str) -> io::Result<()> {
    if !std::path::Path::new(&path).exists() {
        std::fs::create_dir(path)
    } else {
        Ok(())
    }
}

fn create_file_if_not_exist(path: &str) -> io::Result<()> {
    if !std::path::Path::new(&path).exists() {
        match std::fs::File::create(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    } else {
        Ok(())
    }
}
enum VerifyDbExistsError {
    FolderCouldNotBeCreated,
    FileCouldNotBeCreated,
    CouldNtGetHomeDirectory,
}
fn verify_db_exists() -> Result<String, VerifyDbExistsError> {
    let home_folder: String;
    let home_fold = match dirs::home_dir() {
        None => return Err(VerifyDbExistsError::CouldNtGetHomeDirectory),
        Some(r) => r,
    };
    if let Some(home_directory) = home_fold.to_str() {
        home_folder = home_directory.to_string();
    } else {
        return Err(VerifyDbExistsError::CouldNtGetHomeDirectory);
    }
    let mut path_to_config = format!("{}/.config", home_folder);
    match create_folder_if_not_exist(&path_to_config) {
        Ok(_) => {}
        Err(_) => {
            return Err(VerifyDbExistsError::FolderCouldNotBeCreated);
        }
    }

    path_to_config = format!("{}/book-cli", path_to_config);
    match create_folder_if_not_exist(&path_to_config) {
        Ok(_) => {}
        Err(_) => {
            return Err(VerifyDbExistsError::FolderCouldNotBeCreated);
        }
    }

    path_to_config = format!("{}/books.db", path_to_config);
    match create_file_if_not_exist(&path_to_config) {
        Ok(_) => {}
        Err(_) => return Err(VerifyDbExistsError::FileCouldNotBeCreated),
    }
    Ok(path_to_config)
}

//impl Default for DbConfig {
//    fn default() -> DbConfig {
//        let home_folder: String;
//        if let Ok(home_directory) = std::env::var("HOME") {
//            home_folder = home_directory;
//        } else {
//            home_folder = "/Users/dobbikov/".to_string();
//        }
//        DbConfig {
//            path_to_db: format!("{}/.config/book-cli/books.db", home_folder),
//        }
//    }
//}

fn connect_to_db() -> Connection {
    let path_to_db = match verify_db_exists() {
        Err(e) => panic!("Couldn't run app because the db doesn't exist"),
        Ok(r) => r,
    };
    let config: DbConfig = DbConfig { path_to_db };
    let db = Connection::open(config.path_to_db);
    match db {
        Ok(conn) => conn,
        Err(mess) => panic!("An error occured! {}", mess),
    }
}

fn create_table(conn: &Connection) -> Result<bool, bool> {
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS books(
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            section TEXT,
            favourite INTEGER NOT NULL DEFAULT 0 
            )",
        (),
    ) {
        Ok(_) => Ok(true),
        Err(_) => Err(false),
    }
}

pub fn setup() -> Connection {
    let conn = connect_to_db();
    let _ = create_table(&conn);
    conn
}

pub(crate) fn create_book(conn: &Connection, bk: &book::Book) -> Result<bool, CreateBookError> {
    let bk_res = get_book(conn, &bk.name);
    if bk_res.is_ok() {
        return Err(CreateBookError::BookWithNameExists);
    }
    match conn.execute(
        "INSERT INTO books (name, path, section) VALUES (?, ?, ?)",
        params![bk.name, bk.path, bk.section],
    ) {
        Ok(_) => Ok(true),
        Err(_) => Err(CreateBookError::Other),
    }
}

#[derive(Debug)]
pub(crate) enum RemoveBookError {
    BookDoesNotExist,
    Other,
}

impl std::fmt::Display for RemoveBookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RemoveBookError::BookDoesNotExist => write!(f, "this book doesn't exist"),
            RemoveBookError::Other => write!(f, "unexpected error in removing book"),
        }
    }
}

fn remove_book_from_db(conn: &Connection, name: &String) -> Result<usize> {
    let res = conn.execute("DELETE FROM books WHERE name = ?", params![name]);
    debug!("Name: {}, query: {}", name, "temp");
    res
}
pub(crate) fn remove_book(conn: &Connection, name: &String) -> Result<book::Book, RemoveBookError> {
    let bk_res = get_book(conn, name);
    if let Ok(bk) = bk_res {
        let del_res = remove_book_from_db(conn, name);
        if del_res.is_ok() {
            Ok(bk)
        } else {
            Err(RemoveBookError::Other)
        }
    } else {
        Err(RemoveBookError::BookDoesNotExist)
    }
}

pub enum GetBookError {
    QueryError(rusqlite::Error),
    EmptyList,
    NoneElement,
    TableOrBookDoesnotExist,
}

pub fn get_book(conn: &Connection, name: &String) -> Result<book::Book, GetBookError> {
    let stmt = conn.prepare("SELECT * FROM books WHERE name = :name;");
    if let Ok(mut stmt_res) = stmt {
        match stmt_res.query_map(&[(":name", name)], |row| {
            let _: u32 = row.get(0)?;
            Ok(book::Book {
                name: row.get(1)?,
                path: row.get(2)?,
                section: row.get(3)?,
                favourite: row.get(4)?,
            })
        }) {
            Ok(mut book_iter) => {
                if let Some(bk_) = book_iter.next() {
                    if let Ok(bk) = bk_ {
                        Ok(bk)
                    } else {
                        Err(GetBookError::NoneElement)
                    }
                } else {
                    Err(GetBookError::EmptyList)
                }
            }
            Err(e) => Err(GetBookError::QueryError(e)),
        }
    } else {
        Err(GetBookError::TableOrBookDoesnotExist)
    }
}

pub(crate) enum GetBooksError {
    BookOrTableDoesnotExist,
    NoBooks,
}

impl std::fmt::Display for GetBooksError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetBooksError::BookOrTableDoesnotExist => {
                write!(f, "Table doesn't exist or there's no books")
            }
            GetBooksError::NoBooks => write!(f, "There's no books!"),
        }
    }
}

pub(crate) fn get_books(conn: &Connection) -> Result<Vec<book::Book>, GetBooksError> {
    let stmt = conn.prepare("SELECT * FROM books");
    if let Ok(mut stmt_res) = stmt {
        match stmt_res.query_map([], |row| {
            let _: u32 = row.get(0)?;
            Ok(book::Book {
                name: row.get(1)?,
                path: row.get(2)?,
                section: row.get(3)?,
                favourite: row.get(4)?,
            })
        }) {
            Ok(book_iter) => {
                let mut res: Vec<book::Book> = Vec::new();
                for bk_ in book_iter.flatten() {
                    res.push(bk_);
                }
                Ok(res)
            }
            Err(_) => Err(GetBooksError::NoBooks),
        }
    } else {
        Err(GetBooksError::BookOrTableDoesnotExist)
    }
}

pub enum UpdateFavouriteError {
    BookDoesNotExist,
    OtherError,
}

pub(crate) fn update_favourite_error(
    conn: &Connection,
    name: &String,
    favourite: bool,
) -> Result<book::Book, UpdateFavouriteError> {
    if get_book(conn, name).is_err() {
        return Err(UpdateFavouriteError::BookDoesNotExist);
    }
    let stmt = conn.execute(
        "UPDATE books SET favourite = ?1 WHERE name = ?2",
        params![(favourite as u8), name,],
    );
    match stmt {
        Ok(_) => {
            if let Ok(book) = get_book(conn, name) {
                Ok(book)
            } else {
                Err(UpdateFavouriteError::BookDoesNotExist)
            }
        }
        Err(e) => {
            debug!("{}", e);
            Err(UpdateFavouriteError::OtherError)
        }
    }
}
