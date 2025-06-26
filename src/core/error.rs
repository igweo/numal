use std::fmt;

#[derive(Debug)]
pub enum NumalError {
    InvalidInput(String),
    DidNotConverge,
    DerivativeNotComputable,
    LibErr(String),
}

impl fmt::Display for NumalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumalError::InvalidInput(msg) => write!(f, "INVALID INPUT: {msg}"),
            NumalError::DidNotConverge => write!(f, "FAILED TO CONVERGE"),
            NumalError::DerivativeNotComputable => write!(f, "DERIVATIVE NOT COMPUTABLE"),
            NumalError::LibErr(msg) => write!(f, "NUMAL LIB ERROR: {msg}"),
        }
    }
}
