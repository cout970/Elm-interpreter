
#[derive(Clone, Debug)]
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

    pub fn save(&self) -> u32 {
        self.last
    }

    pub fn restore(&mut self, save: u32) {
        self.last = save;
    }
}

