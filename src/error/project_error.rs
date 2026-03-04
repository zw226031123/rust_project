#[derive(Debug)]
pub enum ProjectError {
    Null(),
    JsonParseError(),
    Io(std::io::Error),
    ParseInt(std::num::ParseIntError),
    DecimalError(rust_decimal::Error),
}

impl From<std::io::Error> for ProjectError {
    fn from(error: std::io::Error) -> ProjectError {
        ProjectError::Io(error)
    }
}

impl From<rust_decimal::Error> for ProjectError {
    fn from(error: rust_decimal::Error) -> ProjectError {
        ProjectError::DecimalError(error)
    }
}
