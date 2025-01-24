use crate::book;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn is_pdf(path: &str) -> bool {
    let str_path = path;
    match str_path.split(".").last() {
        Some(val) => val == "pdf",
        None => false,
    }
}
pub fn is_correct_path(path: &String) -> (bool, Option<PathBuf>) {
    let path = Path::new(path);
    if !path.exists() {
        (false, None)
    } else {
        (true, Some(fs::canonicalize(path).unwrap()))
    }
}

// returns a list of books that are only of particular sections
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
