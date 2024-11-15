pub enum Token<'a> {
    Selector { value: &'a str, line: usize },
    Property { value: &'a str, line: usize },
    Value { value: &'a str, line: usize },
    BlockOpen { line: usize },
    BlockClose { line: usize },
    Colon { line: usize },
    Semicolon { line: usize },
    EOF,
}

impl std::fmt::Debug for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Selector { value, .. } => write!(f, "Selector({value})"),
            Token::Property { value, .. } => write!(f, "Property({value})"),
            Token::Value { value, .. } => write!(f, "Value({value})"),
            Token::BlockOpen { .. } => write!(f, "BlockOpen"),
            Token::BlockClose { .. } => write!(f, "BlockClose"),
            Token::Colon { .. } => write!(f, "Colon"),
            Token::Semicolon { .. } => write!(f, "Semicolon"),
            Token::EOF => write!(f, "EOF"),
        }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    #[must_use]
    pub const fn new(input: &'a str) -> Self {
        Lexer {
            input,
            position: 0,
            line: 1,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.input[self.position..].chars().next()?;
        self.position += ch.len_utf8();
        if ch == '\n' {
            self.line += 1;
        }
        Some(ch)
    }

    #[allow(clippy::while_let_loop)]
    fn advance_until_line_stop(&mut self) {
        loop {
            if let Some(peek) = self.peek_char() {
                if peek == ':' || peek == ';' || peek == '\n' {
                    break;
                }
            }
            self.advance();
        }
    }

    #[allow(clippy::while_let_loop)]
    fn advance_until_comment_end(&mut self) {
        loop {
            let Some(c) = self.advance() else {
                break;
            };
            if let Some(peek) = self.peek_char() {
                if c == '*' && peek == '/' {
                    break;
                }
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    #[allow(clippy::while_let_loop)]
    fn skip_whitespace(&mut self) {
        loop {
            let Some(c) = self.peek_char() else {
                break;
            };
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    #[must_use]
    #[allow(clippy::while_let_loop)]
    pub fn tokens(&mut self) -> Vec<Token<'a>> {
        let mut tokens = Vec::with_capacity(1024 * 16); // Some size
        let mut inside_block = false;
        let mut start_pos;
        loop {
            let Some(c) = self.advance() else {
                tokens.push(Token::EOF);
                break;
            };
            match c {
                '{' => {
                    tokens.push(Token::BlockOpen { line: self.line });
                    inside_block = true;
                }
                '}' => {
                    tokens.push(Token::BlockClose { line: self.line });
                    inside_block = false;
                }
                ':' if inside_block => {
                    tokens.push(Token::Colon { line: self.line });
                }
                ';' => {
                    tokens.push(Token::Semicolon { line: self.line });
                }
                '/' => {
                    if let Some(peek) = self.peek_char() {
                        if peek == '*' {
                            self.advance_until_comment_end();
                        }
                    }
                }
                ' ' | '\n' | '\t' => {
                    self.skip_whitespace();
                }
                _ => {
                    start_pos = self.position - c.len_utf8();
                    if inside_block {
                        self.advance_until_line_stop();
                        let value = &self.input[start_pos..self.position];
                        if let Some(peek) = self.peek_char() {
                            if peek == ':' || peek == '\n' {
                                tokens.push(Token::Property {
                                    line: self.line,
                                    value,
                                });
                            } else if peek == ';' || peek == '\n' {
                                tokens.push(Token::Value {
                                    line: self.line,
                                    value,
                                });
                            }
                        }
                    } else {
                        loop {
                            if let Some(peek) = self.peek_char() {
                                if peek == '{' || peek == '\n' {
                                    break;
                                }
                            } else {
                                break;
                            }
                            self.advance();
                        }

                        tokens.push(Token::Selector {
                            line: self.line,
                            value: &self.input[start_pos..self.position],
                        });
                    }
                }
            }
        }
        tokens
    }
}
