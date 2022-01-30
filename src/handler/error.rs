use std::error;
use std::fmt;

#[derive(Debug)]
pub enum HandleError {
    Parse,
    ParseResponse(String),
    NotFoundSecret(String),
    FailedTranslation(String),
    Internal(String),
    AuthenticationForbiden,
}

impl fmt::Display for HandleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HandleError::Parse => write!(f, "Parse error:"),
            HandleError::ParseResponse(e) => write!(f, "{}", e),
            HandleError::NotFoundSecret(e) => write!(f, "{}", e),
            HandleError::FailedTranslation(e) => write!(f, "{}", e),
            HandleError::Internal(e) => write!(f, "{}", e),
            HandleError::AuthenticationForbiden => write!(f, "Authentication error:"),
        }
    }
}

impl error::Error for HandleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            HandleError::Parse => None,
            HandleError::ParseResponse(_) => None,
            HandleError::NotFoundSecret(_) => None,
            HandleError::FailedTranslation(_) => None,
            HandleError::Internal(_) => None,
            HandleError::AuthenticationForbiden => None,
        }
    }
}
