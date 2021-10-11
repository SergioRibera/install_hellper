use core::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    UnsupportedSystem,
    ExecFailed(io::Error),
    IO(io::Error),
    SystemTime(std::time::SystemTimeError),
    General(String),
    Unknown,
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match *self {
            UnsupportedSystem => write!(fmt, "System is not supported"),
            ExecFailed(ref e) => write!(fmt, "Execution failed: {}", e),
            IO(ref e) => write!(fmt, "IO error: {}", e),
            SystemTime(ref e) => write!(fmt, "System time error: {}", e),
            General(ref e) => write!(fmt, "Error: {}", e),
            Unknown => write!(fmt, "An unknown error occurred"),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match *self {
            UnsupportedSystem => "unsupported system",
            ExecFailed(_) => "execution failed",
            IO(_) => "io error",
            SystemTime(_) => "system time",
            General(_) => "general error",
            Unknown => "unknown error",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        use self::Error::*;
        match *self {
            UnsupportedSystem => None,
            ExecFailed(ref e) => Some(e),
            IO(ref e) => Some(e),
            SystemTime(ref e) => Some(e),
            General(_) => None,
            Unknown => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IO(e)
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(e: std::time::SystemTimeError) -> Error {
        Error::SystemTime(e)
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(e: Box<dyn std::error::Error>) -> Error {
        Error::General(e.to_string())
    }
}
