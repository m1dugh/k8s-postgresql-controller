use std::fmt::Display;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    Default(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Default(msg) => write!(f, "{msg}"),
        }
    }
}

impl From<rocket::Error> for Error {
    fn from(value: rocket::Error) -> Self {

        let kind = value.kind();

        let msg = format!("rocket error: {}", kind);

        Error::Default(msg)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
