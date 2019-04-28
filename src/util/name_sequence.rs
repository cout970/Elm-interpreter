#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameSequence {
    last: u32
}

const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

impl NameSequence {
    pub fn new() -> Self {
        Self { last: 0 }
    }

    pub fn next(&mut self) -> String {
        let index = self.last as usize;
        self.last += 1;

//        if index < ALPHABET.len() {
//            let mut name = String::with_capacity(1);
//
//            name.push(ALPHABET[index] as char);
//
//            return name;
//        } else if index / ALPHABET.len() < ALPHABET.len() {
//            let first = index / ALPHABET.len();
//            let second = index % ALPHABET.len();
//            let mut name = String::with_capacity(2);
//
//            name.push(ALPHABET[first] as char);
//            name.push(ALPHABET[second] as char);
//
//            return name;
//        } else {
//            panic!("To many names!");
//        }

        Self::format_radix(index as u32, 26)
    }

    // https://stackoverflow.com/questions/50277050/is-there-a-built-in-function-that-converts-a-number-to-a-string-in-any-base
    fn format_radix(mut x: u32, radix: u32) -> String {
        let mut result = vec![];

        loop {
            let m = x % radix;
            x = x / radix;

            result.push(b"abcdefghijklmnopqrstuvwxyz"[m as usize]);
            if x == 0 {
                break;
            }
        }

        String::from_utf8_lossy(&result.into_iter().rev().collect::<Vec<_>>()).to_string()
    }

    /// Generates a sequence of names with a prefix and number at the end
    /// For example, the prefix "number" will generate:
    ///
    /// number, number1, number2, number3, number4, etc
    ///
    pub fn next_with_prefix(&mut self, prefix: &str) -> String {
        let index = self.last as usize;
        self.last += 1;

        if index == 0 {
            prefix.to_string()
        } else {
            format!("{}{}", prefix, index)
        }
    }

    pub fn save(&self) -> u32 {
        self.last
    }

    pub fn restore(&mut self, save: u32) {
        self.last = save;
    }
}

