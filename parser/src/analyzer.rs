use crate::lexer::Token;

pub struct SyntaxError<'a> {
    pub line: usize,
    pub column: usize,
    pub error: &'static str,
    pub value: &'a str,
}

impl std::fmt::Display for SyntaxError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "error: {}\n {}:{} | {}",
            self.error, self.line, self.column, self.value
        )
    }
}

fn find_line_with_value<'a>(lines: &[&'a str], line: usize, m: &str) -> (usize, &'a str) {
    let line = lines[line - 1];
    let col = line
        .find(m)
        .unwrap_or_else(|| panic!("Analyzer error: cant find column for {m}"));
    (col, line)
}

/// Work in progress
#[must_use]
pub fn analyze_tokens<'a>(tokens: &[Token<'a>], input: &'a str) -> Vec<SyntaxError<'a>> {
    let mut errors: Vec<SyntaxError> = Vec::with_capacity(tokens.len());
    let mut tokens = tokens.iter().peekable();
    let lines = input.lines().collect::<Vec<_>>();
    loop {
        let Some(token) = tokens.next() else {
            break;
        };

        match token {
            Token::Selector { line, value } => {
                let line = *line;
                let (column, value) = find_line_with_value(&lines, line, value);
                if let Some(peek) = tokens.peek() {
                    if !matches!(peek, Token::BlockOpen { .. } | Token::Selector { .. }) {
                        let error = SyntaxError {
                            line,
                            column,
                            error: "Expecting { after selector",
                            value,
                        };
                        errors.push(error);
                    }
                } else {
                    let error = SyntaxError {
                        line,
                        column,
                        error: "Unexpected EOF",
                        value,
                    };
                    errors.push(error);
                }
            }
            Token::Property { line, value } => {
                if let Some(peek) = tokens.peek() {
                    if !matches!(peek, Token::Colon { .. }) {
                        let line = *line;
                        let (column, value) = find_line_with_value(&lines, line, value);
                        let error = SyntaxError {
                            line,
                            column,
                            error: "Unexpected token after property",
                            value,
                        };
                        errors.push(error);
                    }
                } else {
                    todo!()
                }
            }
            Token::Value { value, line } => {
                if let Some(peek) = tokens.peek() {
                    if !matches!(peek, Token::Semicolon { .. }) {
                        let line = *line;
                        let (column, value) = find_line_with_value(&lines, line, value);
                        let error = SyntaxError {
                            line,
                            column,
                            error: "Expecting ; after value",
                            value,
                        };
                        errors.push(error);
                    }
                } else {
                    todo!()
                }
            }
            Token::Semicolon { line } => {
                if let Some(peek) = tokens.peek() {
                    if !matches!(peek, Token::BlockClose { .. } | Token::Property { .. }) {
                        let line = *line;
                        let (column, value) = find_line_with_value(&lines, line, ";");
                        let error = SyntaxError {
                            line,
                            column,
                            error: "Expecting property or } after ;",
                            value,
                        };
                        errors.push(error);
                    }
                } else {
                    todo!()
                }
            }
            Token::Colon { line } => {
                if let Some(peek) = tokens.peek() {
                    if !matches!(peek, Token::Value { .. }) {
                        let line = *line;
                        let (column, value) = find_line_with_value(&lines, line, ":");
                        let error = SyntaxError {
                            line,
                            column,
                            error: "Expecting value after :",
                            value,
                        };
                        errors.push(error);
                    }
                } else {
                    todo!()
                }
            }
            _ => (),
        }
    }

    errors
}
