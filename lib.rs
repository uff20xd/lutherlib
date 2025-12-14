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
    buffer: Vec<Token>,
    group_stream: Vec<TokenGroup>,
    start_pointer: usize,
    end_pointer: usize,
    save_state: (usize, usize),
}
