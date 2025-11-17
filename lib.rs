pub struct Token {
    value: String,
    start: usize,
    end: usize,
    file_id: usize,
}

impl Token {
    pub fn new(value: String, start: usize, end: usize, file_id: usize) {
        Self {
            value,
            start,
            end,
            file_id,
        }
    }

    pub fn get_val(&self) -> &String {
        &self.value
    }
    pub fn get_val_mut(&mut self) -> &mut String {
        &mut self.value
    }
}

struct LexerRule(Arc<str>);

impl LexerRule {
    pub fn new(keyword: &'static str) -> Self {
        Self(Arc::new(*keyword))
    }
}

pub struct LuthersLexer {
    source: String,
    source_pointer: usize,
    file_id: usize,
    token_stream: Vec<Token>,
    rules: Vec<LexerRule>,
}

impl LuthersLexer {
    pub fn new(source: String, file_id: usize) -> Self {
        Self {
            source,
            source_pointer: 0,
            file_id,
            token_stream: Vec::new(),
            rules: Vec::new(),
        }
    }
}
