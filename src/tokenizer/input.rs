use tokenizer::Location;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct InputSlice<'a> {
    pub stream: &'a [u8],
    pos: u32,
}

impl<'a> InputSlice<'a> {
    pub fn new(stream: &'a [u8]) -> Self {
        InputSlice {
            stream,
            pos: 0,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.stream.len()
    }

    #[inline]
    pub fn advance(&self, n: usize) -> InputSlice<'a> {
        InputSlice {
            stream: &self.stream[n..],
            pos: self.pos + (n as u32),
        }
    }

    pub fn get_location(&self) -> Location {
        self.pos
    }
}