use std;
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Parse,
    Syntax(Option<(usize, String)>, String),
    Message(&'static str),
}

impl Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Message(ref msg) => formatter.write_str(msg),
            Error::Syntax(ref line, ref reason) => {
                if let Some((line_n, line)) = line {
                    write!(formatter, "Syntax error on line {}: {}\n {}", line_n + 1, reason, line)
                } else {
                    write!(formatter, "Syntax error: {}", reason)
                }
            },
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
