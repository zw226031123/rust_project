use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("Null")]
    Null(),
    #[error("JsonParseError")]
    JsonParseError(),
    #[error("Io")]
    Io(std::io::Error),
    #[error("ParseInt")]
    ParseInt(std::num::ParseIntError),
    #[error("DecimalError")]
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
