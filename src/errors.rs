use super::db;

pub enum CreateBookError {
    ProvidedPathIsNotPdf,
    ProvidedPathIsIncorrect,
    BookNameAlreadyUsed,
    OtherError,
}

impl std::fmt::Display for CreateBookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateBookError::ProvidedPathIsNotPdf => write!(f, "Provdied path is not a PDF file!"),
            CreateBookError::ProvidedPathIsIncorrect => write!(f, "Provide path is incorrect!"),
            CreateBookError::BookNameAlreadyUsed => write!(f, "Provided name is already in use!"),
            CreateBookError::OtherError => write!(f, "Unexpected error!"),
        }
    }
}

pub enum UpdateFavouriteError {
    BookDoesNotExist,
    Other,
}

impl std::fmt::Display for UpdateFavouriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdateFavouriteError::BookDoesNotExist => write!(f, "The book doesn't exist!"),
            UpdateFavouriteError::Other => write!(f, "Unexpected error!"),
        }
    }
}

impl From<db::UpdateFavouriteError> for UpdateFavouriteError {
    fn from(value: db::UpdateFavouriteError) -> Self {
        match value {
            db::UpdateFavouriteError::BookDoesNotExist => UpdateFavouriteError::BookDoesNotExist,
            db::UpdateFavouriteError::OtherError => UpdateFavouriteError::Other,
        }
    }
}

pub enum GetBooksError {
    BookOrTableDoesnotExist,
    NoBooks,
}

impl std::fmt::Display for GetBooksError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetBooksError::BookOrTableDoesnotExist => {
                write!(f, "Books or the table of books doesn't exist!")
            }
            GetBooksError::NoBooks => write!(f, "The table of books is empty!"),
        }
    }
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

impl std::fmt::Display for GetBookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetBookError::TableOrBookDoesnotExist => {
                write!(f, "The book or a table of books doesn't exist!")
            }
        }
    }
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

impl std::fmt::Display for RemoveBookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RemoveBookError::BookDoesNotExist => write!(f, "This book does not exist!"),
            RemoveBookError::Other => write!(f, "Unexpected error!"),
        }
    }
}

pub enum OpenBookError {
    BookDoesNotExist,
    PathIsIncorrect,
    FileIsNotPDF,
    OtherError,
}

impl std::fmt::Display for OpenBookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenBookError::BookDoesNotExist => write!(f, "This book does not exist!"),
            OpenBookError::PathIsIncorrect => write!(f, "Provided path is incorrect!"),
            OpenBookError::FileIsNotPDF => write!(f, "Provided path is not a PDF file!"),
            OpenBookError::OtherError => write!(f, "Unexpected error!"),
        }
    }
}

impl From<db::RemoveBookError> for RemoveBookError {
    fn from(value: db::RemoveBookError) -> Self {
        match value {
            db::RemoveBookError::BookDoesNotExist => RemoveBookError::BookDoesNotExist,
            db::RemoveBookError::Other => RemoveBookError::Other,
        }
    }
}
