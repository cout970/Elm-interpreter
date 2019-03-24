use std::str::Chars;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct SourceCode {
    code: Arc<String>
}

pub type Location = u32;

impl SourceCode {
    pub fn new(code: &str) -> Self {
        SourceCode {
            code: Arc::new(code.to_string())
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        SourceCode {
            code: Arc::new(String::from_utf8_lossy(&bytes).to_string())
        }
    }

    pub fn chars(&self) -> Chars {
        self.code.chars()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.code.as_bytes()
    }

    pub fn as_str(&self) -> &str {
        self.code.as_str()
    }

    // TODO remove this is copying large strings
    pub fn to_string(&self) -> String {
        self.code.as_str().to_owned()
    }
}