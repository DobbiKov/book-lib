# Book Lib
This library provides an interface for adding, removing and manging existing PDF files stored on your device.

## Requirements
1. unix device
1. rust
3. sqlite

## Setup
1. clone the project from the git repository or add it using `cargo add book_lib`
2. use the api in your rust project


## Usage
1. Create a connection to the database:
```rust
use book_lib::{db, book};

let connection = book_lib::db::setup();

```

2. Create a new book
```rust
let new_book = book::Book::init("book_name".to_string(), "path_to/your/file.pdf".to_string(), None, false);
book_lib::create_book(&connection, &new_book); //creating new book in the DB
```

3. Make it favourite
```rust
book_lib::update_favourite(&connection, &("book_name".to_string()), true); //true to be favourite, false not to be
```

4. Remove the book
```rust
book_lib::remove_book(&connection, &("book_name".to_string()));
```

## Examples of implementation
1. [cli for managing PDFs](https://github.com/DobbiKov/book-cli)
2. [GUI for managing PDFs](https://github.com/DobbiKov/book-manager-app)

## Contribution
Feel free to open pull requests!
