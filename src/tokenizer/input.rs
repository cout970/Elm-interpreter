#[derive(PartialEq, Debug, Clone, Copy)]
pub struct InputSlice<'a> {
    pub stream: &'a [u8],
    line: u32,
    column: u32,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Location {
    pub line: u32,
    pub column: u32,
}

impl<'a> InputSlice<'a> {
    pub fn new(stream: &'a [u8]) -> Self {
        InputSlice {
            stream,
            line: 0,
            column: 0,
        }
    }

    pub fn advance(&self, n: usize) -> InputSlice<'a> {
        let skipped: &[u8] = &self.stream[0..n];

        let mut line = self.line;
        let mut column = self.column;

        for c in skipped {
            if *c as char == '\n' {
                line += 1;
                column = 0;
            } else {
                column += 1;
            }
        }

        InputSlice {
            stream: &self.stream[n..],
            line,
            column,
        }
    }

    pub fn get_location(&self) -> Location {
        Location { line: self.line, column: self.column }
    }
}