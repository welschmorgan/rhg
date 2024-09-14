use std::{
  fmt::Display,
  path::{Path, PathBuf},
};

use crate::{here, Location};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorKind {
    IO,
    Unknown
}

#[derive(Debug, Clone)]
pub struct Error {
  kind: ErrorKind,
  message: String,
  cause: Option<Box<Self>>,
  location: Location,
}

impl Error {
  pub fn new(kind: ErrorKind, message: String, cause: Option<Error>, location: Location) -> Self {
    Self {
      kind,
      message,
      cause: cause.map(|c| Box::new(c)),
      location,
    }
  }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}] {} at {}", self.kind, self.message, self.location)?;
        if let Some(c) = &self.cause {
            write!(f, "\nCaused by: {}", c)?;
        }
        Ok(())
    }
}
impl std::error::Error for Error {}

#[macro_export]
macro_rules! err  {
    ($kind:expr,$msg:expr) => {
        Err($crate::Error::new($kind, format!("{}", $msg), None, $crate::here!()))
    };
    ($kind:expr,$msg:expr,$cause:expr) => {
        Err($crate::Error::new($kind, format!("{}", $msg), Option<$cause>, $crate::here!()))
    };
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::new(ErrorKind::IO, value.to_string(), None, here!())
    }
}

pub fn create_window() {
  todo!("create_window")
}
