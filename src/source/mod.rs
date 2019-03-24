use std::str::Chars;
use std::sync::Arc;

/// Internal container for ELM source code,
/// the main use of this container is to avoid duplication of large files
#[derive(Clone, Debug, PartialEq)]
pub struct SourceCode {
    code: Arc<String>
}

/// Identifies a position in the stream of chars in the source code
pub type Location = u32;

impl SourceCode {
    /// Creates a SourceCode instance wrapping a string
    pub fn from_string(code: String) -> Self {
        let mut code = code;

        // Padding to detect the end of code while tokenizing, this avoids having to check for
        // the end of code before reading every character
        code.push('\0');
        code.push('\0');
        code.push('\0');
        code.push('\0');

        SourceCode {
            code: Arc::new(code)
        }
    }

    /// Creates a SourceCode instance cloning a string slice
    /// This method is not recommended for large strings, use [from_string] instead
    pub fn from_str(code: &str) -> Self {
        Self::from_string(code.to_string())
    }

    /// Creates a SourceCode instance from an vector of bytes.
    ///
    /// If the vec contains any invalid UTF-8 sequences the function will replace them with
    /// a replacement character: �
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self::from_string(String::from_utf8_lossy(&bytes).to_string())
    }

    /// Creates a SourceCode instance from an slice of bytes.
    ///
    /// If the vec contains any invalid UTF-8 sequences the function will replace them with
    /// a replacement character: �
    pub fn from_slice(bytes: &[u8]) -> Self {
        Self::from_string(String::from_utf8_lossy(&bytes).to_string())
    }

    /// Returns a real size of the source code
    pub fn len(&self) -> usize {
        self.code.len() - 4
    }

    /// Returns a character iterator for the code
    pub fn chars(&self) -> Chars {
        self.code.chars()
    }

    /// Returns a byte slice of the code
    pub fn as_bytes(&self) -> &[u8] {
        self.code.as_bytes()
    }

    /// Returns a string slice of the code
    pub fn as_str(&self) -> &str {
        self.code.as_str()
    }

    // TODO this will be removed because it causes a copy of the original code
    /// Creates a string from the code
    pub fn to_string(&self) -> String {
        self.code.as_str().to_owned()
    }
}