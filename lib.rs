#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    value: &'a str,
    start: usize,
    end: usize,
    file_id: usize,
}

impl Token {
    pub fn new(value: &str, start: usize, end: usize, file_id: usize) {
        Self {
            value,
            start,
            end,
            file_id,
        }
    }
    pub fn get_val(&self) -> &str {
        self.value
    }
}

#[derive(Debug, Clone)]
pub struct TokenGroup {
    tokens: Vec<Token>,
}

struct LexerLuther {
    source: String,
    buffer: Vec<Token>,
    group_stream: Vec<TokenGroup>,
    start_pointer: usize,
    end_pointer: usize,
    save_state: (usize, usize),
}

impl LexerLuther {
    fn new(source: String) -> Self {
        let new = Self {
            source: String,
            buffer: Vec::new(),
            group_stream: Vec::new(),
            start_pointer: 0,
            end_pointer: 0,
            save_state: (0, 0),
        };
        new
    }
    fn next(&mut self) -> Option<char> {
        self.end_pointer += 1;
        if !(self.source.len() >= end_pointer - 1) {

        }
        None
    }
    fn next_nth(&mut self, n: usize) -> Option<&[char]> {

    }
}
