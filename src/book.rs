//! A module that contains a Book implementation and associated methods and functions.

use std::cmp::Ordering;

#[derive(Clone, Debug)]
/// A struct representing a book
///
/// The struct contains book name, path and optional section
pub struct Book {
    /// path to the book in the system
    pub path: String,
    /// name given to the book
    pub name: String,
    /// Some(section name) or None
    pub section: Option<String>,
    /// book marked as favourite
    pub favourite: bool,
}

impl Book {
    /// Init function for the book that takes each filed and returns a Book
    pub fn init(name: String, path: String, section: Option<String>, favourite: bool) -> Book {
        Book {
            path,
            name,
            section,
            favourite,
        }
    }
}

/// Takes a filename from a long path
///
/// ## Example
/// ```rust
/// let path = "path/to/my/file_unique_name.pdf";
/// let file_name = extract_file_name_from_path(path);
/// assert_eq!(file_name, "file_unique_name.pdf");
/// ```
fn extract_file_name_from_path(path: &str) -> String {
    if !path.contains('/') {
        path.to_string()
    } else {
        path.split("/").last().unwrap().to_string()
    }
}

/// Takes a book and prints it to the terminal
fn print_book(bk: Book) {
    let res = format!(
        "{0}[{1}]: {2}",
        bk.name,
        if let Some(sec) = bk.section {
            sec
        } else {
            "".to_string()
        },
        extract_file_name_from_path(&bk.path)
    );
    print!("{}", res);
}

/// Takes a vector of books and prints it
///
/// Also it takes an indent: u16 paramter that makes as many spaces as given to the indent
/// parameter before each book
pub fn print_books(books: Vec<Book>, indent: u16) {
    for bk in books {
        (0..indent).for_each(|_i| {
            print!(" ");
        });
        print_book(bk);
        println!();
    }
}

/// Takes a vector of books sorts it by section.
///
/// Return format:
/// {
/// (section_1, {book_3, book_5}),
/// (section_2, {book_1, book_2, book_4})
/// }
pub fn sort_books_by_section(books: Vec<Book>) -> Vec<(String, Vec<Book>)> {
    let mut bks = books;
    let mut res: Vec<(String, Vec<Book>)> = Vec::new();
    fn compare(bk1: &Book, bk2: &Book) -> Ordering {
        bk1.section.cmp(&bk2.section)
    }
    bks.sort_by(compare);
    let mut temp_res: Vec<Book> = Vec::new();
    let mut curr_section: String = "".to_string();

    for bk in bks {
        let mut sec: String = "".to_string();
        if let Some(_sec) = &bk.section {
            sec = _sec.clone();
        }

        if sec != curr_section {
            if !temp_res.is_empty() {
                res.push((curr_section, temp_res));
                temp_res = Vec::new();
            }
            curr_section = sec
        }
        temp_res.push(bk);
    }
    res.push((curr_section, temp_res));
    res
}
