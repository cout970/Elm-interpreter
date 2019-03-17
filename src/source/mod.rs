use std::str::Chars;
use std::sync::Arc;

#[derive(Clone, Debug)]
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
}