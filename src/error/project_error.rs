pub enum ProjectError {
    Io(std::io::Error),
    ParseInt(std::num::ParseIntError),
}

impl From<std::io::Error> for ProjectError {
    fn from(error: std::io::Error) -> ProjectError {
        ProjectError::Io(error)
    }
}
