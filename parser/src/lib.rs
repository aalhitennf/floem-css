pub mod analyzer;
pub mod declaration;
pub mod lexer;
mod parser;
mod read;

use analyzer::{analyze_tokens, SyntaxError};
use lexer::Lexer;
use parser::{replace_vars, Parser};

pub use parser::{PseudoClass, Rule};
pub use read::read_styles;

#[must_use]
pub fn css_to_rules(input: &str) -> Vec<Rule<'_>> {
    let tokens = Lexer::new(input).tokens();
    let rules = Parser::new(tokens).parse();
    replace_vars(rules)
}

pub fn analyze(input: &str) -> Vec<SyntaxError> {
    let tokens = Lexer::new(input).tokens();
    analyze_tokens(&tokens, input)
}
