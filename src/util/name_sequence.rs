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

        if index < ALPHABET.len() {
            let mut name = String::with_capacity(1);

            name.push(ALPHABET[index] as char);

            return name;
        } else if index / ALPHABET.len() < ALPHABET.len() {
            let first = index / ALPHABET.len();
            let second = index % ALPHABET.len();
            let mut name = String::with_capacity(2);

            name.push(ALPHABET[first] as char);
            name.push(ALPHABET[second] as char);

            return name;
        } else {
            panic!("To many names!");
        }
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

