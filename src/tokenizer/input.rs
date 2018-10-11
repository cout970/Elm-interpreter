#[derive(PartialEq, Debug, Clone)]
pub struct Input<'a> {
    pub stream: &'a [u8],
    line: u32,
    column: u32,
}

impl<'a> Input<'a> {
    pub fn new(stream: &'a [u8]) -> Self {
        Input {
            stream,
            line: 0,
            column: 0,
        }
    }

    pub fn advance(&self, n: usize) -> Input<'a> {
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

        Input {
            stream: &self.stream[n..],
            line,
            column,
        }
    }
}