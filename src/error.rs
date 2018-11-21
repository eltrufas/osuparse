use std;
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Parse,
    Syntax(String),
    Message(String),
}

impl Display for Error {
    fn fmt(&self, formatter: &mut  std::fmt::Formatter) -> std::fmt::Result {
		match *self {
            Error::Message(ref msg) => formatter.write_str(msg),
            Error::Syntax(ref reason) => write!(formatter, "Syntax error: {}", reason),
            Error::Parse => formatter.write_str("Parsing error"),
        }
    }
}

/*impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Message(ref msg) => msg,
            Error::Syntax(reason) => &format!("Syntax error: {}", reason),
        }
    }
}*/
