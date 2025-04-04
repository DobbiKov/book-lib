//! Module for simple helping function

use crate::book;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Takes a path (as &str) to the parameter and returns true if the path supposed to be a pdf and
/// false otherwise
pub fn is_pdf(path: &str) -> bool {
    let str_path = path;
    match str_path.split(".").last() {
        Some(val) => val == "pdf",
        None => false,
    }
}

/// Takes path (as &String) to the parameter and returns (true, Some(Path)) if the path is correct (the
/// file on the path exists) and (false, None) otherwise
pub fn is_correct_path(path: &String) -> (bool, Option<PathBuf>) {
    let path = Path::new(path);
    if !path.exists() {
        (false, None)
    } else {
        (true, Some(fs::canonicalize(path).unwrap()))
    }
}

/// Returns a list of books that are only of particular sections, takes a vector of books and
/// section name
pub fn get_books_with_section(books: Vec<book::Book>, section: &String) -> Vec<book::Book> {
    let mut res: Vec<book::Book> = Vec::new();
    for bk in books {
        if let Some(bk_sec) = &bk.section {
            if bk_sec == section {
                res.push(bk);
            }
        }
    }
    res
}
