use std::{fmt::Display, path::{Path, PathBuf}};


#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location {
    path: PathBuf,
    line: usize,
    column: usize
}

#[macro_export]
macro_rules! here {
    () => {
        $crate::Location::new(file!(), line!() as usize, column!() as usize)        
    };
}

impl Location {
    pub fn new<P: AsRef<Path>>(path: P, line: usize, column: usize) -> Self {
        Self { path: path.as_ref().to_path_buf(), line, column }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.path.display(), self.line, self.column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(format!("{}", Location::new("test.txt", 10, 4)), format!("{}", "test.txt:10:4"))
    }
}