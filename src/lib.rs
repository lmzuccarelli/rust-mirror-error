use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MirrorError {
    details: String,
}

#[allow(dead_code)]
impl MirrorError {
    pub fn new<S: Into<String>>(msg: S) -> MirrorError {
        MirrorError {
            details: msg.into(),
        }
    }
}

impl fmt::Display for MirrorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MirrorError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    // this brings everything from parent's scope into this scope
    use super::*;

    #[test]
    fn err_pass_str() {
        let err = MirrorError::new("testing error 123456");
        assert_eq!(err.to_string(), "testing error 123456");
        assert_eq!(err.description(), "testing error 123456");
    }

    #[test]
    fn err_pass_string() {
        let err = MirrorError::new(String::from("testing owned string"));
        assert_eq!(err.to_string(), "testing owned string");
        assert_eq!(err.description(), "testing owned string");
    }
}
