#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    value: &'a str,
    start: usize,
    end: usize,
    file_id: usize,
}

impl<'a> Token<'a> {
    pub fn new(value: &'a str, start: usize, end: usize, file_id: usize) -> Self {
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
pub struct TokenGroup<'a> {
    tokens: Vec<Token<'a>>,
}

struct LexerLuther<'a> {
    source: String,
    char_source: Vec<char>,
    buffer: Vec<Token<'a>>,
    group_stream: Vec<TokenGroup<'a>>,
    start_pointer: usize,
    end_pointer: usize,
    save_state: (usize, usize),
}

impl<'a> LexerLuther<'a> {
    fn new(source: String) -> Self {
        let new = Self {
            source: source.clone(),
            char_source: source.chars().collect(),
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
        if !(self.source.len() >= self.end_pointer - 1) {
            return Some(self.char_source[self.end_pointer - 1]);
        }
        None
    }
    fn next_nth(&mut self, n: usize) -> Option<&[char]> {
        todo!();
    }
}
