use serde::{Deserialize, Serialize};
use serde_json::Value;
use diesel::result::Error;

const BAD_REQUEST: u16 = 400;
const UNAUTHORIZED: u16 = 401;
const FORBIDDEN: u16 = 403;
const NOT_FOUND: u16 = 404;
const INTERNAL_SERVER_ERROR: u16 = 500;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExceptionBuilder {
    status_code: u16,
    message: Option<String>,
    payload: Option<Value>,
}

impl ExceptionBuilder {
    pub fn new_bad_request_exception() -> Self {
        Self {
            status_code: BAD_REQUEST,
            message: None,
            payload: None,
        }
    }

    pub fn new_unauthorized_exception() -> Self {
        Self {
            status_code: UNAUTHORIZED,
            message: None,
            payload: None,
        }
    }

    pub fn new_forbidden_exception() -> Self {
        Self {
            status_code: FORBIDDEN,
            message: None,
            payload: None,
        }
    }

    pub fn new_not_found_exception() -> Self {
        Self {
            status_code: NOT_FOUND,
            message: None,
            payload: None,
        }
    }

    pub fn new_internal_server_error_exception() -> Self {
        Self {
            status_code: INTERNAL_SERVER_ERROR,
            message: None,
            payload: None,
        }
    }

    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    pub fn payload(mut self, payload: Value) -> Self {
        self.payload = Some(payload);
        self
    }

    pub fn build(self) -> Exception {
        Exception {
            status_code: self.status_code,
            message: self.message,
            payload: self.payload,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Exception {
    pub status_code: u16,
    pub message: Option<String>,
    pub payload: Option<Value>,
}

impl Default for Exception {
    fn default() -> Self {
        Self {
            status_code: BAD_REQUEST,
            message: None,
            payload: None,
        }
    }
}
