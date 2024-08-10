use std::{error::Error, fmt::{Debug, Display}, io};

use axum::extract::multipart::MultipartError;

#[derive(Debug)]
pub enum WriteError<T> {
    IO(io::Error),
    StreamError(T),
}

impl<T> Error for WriteError<T> 
where
    T: Debug + Display { }

impl<T> Display for WriteError<T> 
where
    T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteError::IO(e) => write!(f, "Error writing file: {}", e),
            WriteError::StreamError(e) => write!(f, "Error reading stream: {}", e),
        }
    }
}

impl<T> From<io::Error> for WriteError<T> {
    fn from(value: io::Error) -> Self {
        WriteError::IO(value)
    }
}

impl From<MultipartError> for WriteError<MultipartError> {
    fn from(value: MultipartError) -> Self {
        WriteError::StreamError(value)
    }
}

#[derive(Debug)]
pub enum ReadError {
    IO(io::Error),
}

impl Error for ReadError { }

impl Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadError::IO(e) => write!(f, "Error reading file: {}", e),
        }
    }
}

impl From<io::Error> for ReadError {
    fn from(value: io::Error) -> Self {
        ReadError::IO(value)
    }
}

#[derive(Debug)]
pub enum DeleteError {
    IO(io::Error),
}

impl Error for DeleteError { }

impl Display for DeleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeleteError::IO(e) => write!(f, "Error deleting file: {}", e),
        }
    }
}

impl From<io::Error> for DeleteError {
    fn from(value: io::Error) -> Self {
        DeleteError::IO(value)
    }
}