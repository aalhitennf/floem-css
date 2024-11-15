use std::{borrow::Cow, iter::Zip};

use smallvec::SmallVec;

use crate::lexer::Token;

/// Parser that turns lexer tokens into `ParserToken` and builds
/// a `Rule`
pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
}

pub enum ParserToken<'a> {
    Selector { value: &'a str },
    Property { value: &'a str },
    Value { value: &'a str },
}

impl<'a> ParserToken<'a> {
    #[must_use]
    pub const fn from_token(token: &Token<'a>) -> Option<Self> {
        match token {
            Token::Selector { value, .. } => Some(ParserToken::Selector { value }),
            Token::Property { value, .. } => Some(ParserToken::Property { value }),
            Token::Value { value, .. } => Some(ParserToken::Value { value }),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub enum PseudoClass {
    Hover,
    Active,
    ActiveHover,
    Disabled,
    DisabledHover,
    Focus,
    FocusHover,
    Placeholder,
    Selection,
}

impl PseudoClass {
    pub const fn parse_str(s: &str) -> Option<Self> {
        let val = match s.as_bytes() {
            b":hover" => Self::Hover,
            b":focus" => Self::Focus,
            b":focus:hover" => Self::FocusHover,
            b":active" => Self::Active,
            b":active:hover" => Self::ActiveHover,
            b":disabled" => Self::Disabled,
            b":disabled:hover" => Self::DisabledHover,
            b"::placeholder" => Self::Placeholder,
            b"::selection" => Self::Selection,
            _ => return None,
        };
        Some(val)
    }
}

pub struct Selector<'a> {
    pub selector: &'a str,
    pub pseudo_class: Option<PseudoClass>,
}

fn split_value(value: &str) -> Selector {
    if let Some(colon_column) = value.find(':') {
        Selector {
            selector: &value[..colon_column],
            pseudo_class: PseudoClass::parse_str(&value[colon_column..]),
        }
    } else {
        Selector {
            selector: value,
            pseudo_class: None,
        }
    }
}

#[cold]
fn split_double_colon(value: &str) -> Selector {
    // ::whatever:hover
    //           ^ find this
    if let Some(colon_column) = value[2..].find(':') {
        Selector {
            selector: &value[..colon_column],
            pseudo_class: PseudoClass::parse_str(&value[colon_column..]),
        }
    } else {
        Selector {
            selector: value,
            pseudo_class: None,
        }
    }
}

impl<'a> From<&'a str> for Selector<'a> {
    #[inline]
    fn from(value: &'a str) -> Self {
        if value == ":root" {
            return Self {
                selector: value,
                pseudo_class: None,
            };
        }
        if value.starts_with("::") {
            return split_double_colon(value);
        }
        split_value(value)
    }
}

pub struct Rule<'a> {
    pub selectors: SmallVec<[Selector<'a>; 4]>,
    pub properties: SmallVec<[Cow<'a, str>; 4]>,
    pub values: SmallVec<[Cow<'a, str>; 4]>,
}

impl Rule<'_> {
    #[must_use]
    pub const fn new_const() -> Self {
        Self {
            selectors: SmallVec::<[Selector<'_>; 4]>::new_const(),
            properties: SmallVec::<[Cow<'_, str>; 4]>::new_const(),
            values: SmallVec::<[Cow<'_, str>; 4]>::new_const(),
        }
    }

    pub fn iter_props(
        &self,
    ) -> Zip<std::slice::Iter<'_, Cow<'_, str>>, std::slice::Iter<'_, Cow<'_, str>>> {
        self.properties.iter().zip(self.values.iter())
    }

    pub fn remove(&mut self, index: usize) {
        self.properties.remove(index);
        self.values.remove(index);
    }
}

impl<'a> Parser<'a> {
    #[must_use]
    pub const fn new(tokens: Vec<Token<'a>>) -> Self {
        Self { tokens }
    }

    fn selector_count(&self) -> usize {
        self.tokens
            .iter()
            .filter(|t| matches!(t, Token::Selector { .. }))
            .count()
    }

    #[must_use]
    pub fn parse(self) -> Vec<Rule<'a>> {
        let mut rules = Vec::with_capacity(self.selector_count());
        let mut props = SmallVec::<[ParserToken; 16]>::new_const();
        let mut tokens = self
            .tokens
            .iter()
            .filter_map(ParserToken::from_token)
            .peekable();
        'main: loop {
            let Some(token) = tokens.next() else {
                break 'main;
            };

            let ParserToken::Selector { value: selector } = token else {
                continue 'main;
            };

            let mut rule = Rule::new_const();
            props.clear();
            rule.selectors
                .extend(selector.split(',').map(str::trim).map(Selector::from));
            'props: loop {
                let Some(peek) = tokens.peek() else {
                    break 'props;
                };
                if matches!(peek, ParserToken::Selector { .. }) {
                    break 'props;
                }
                if let Some(next) = tokens.next() {
                    props.push(next);
                }
            }
            for chunk in props.chunks_exact(2) {
                if let [ParserToken::Property { value: prop_value }, ParserToken::Value { value }] =
                    chunk
                {
                    rule.properties.push(Cow::Borrowed(prop_value));
                    rule.values.push(Cow::Borrowed(value));
                }
            }
            rules.push(rule);
        }
        rules
    }
}

pub(crate) fn replace_vars(mut rules: Vec<Rule<'_>>) -> Vec<Rule<'_>> {
    let Some(root_idx) = rules
        .iter()
        .position(|rule| rule.selectors.iter().any(|s| s.selector == ":root"))
    else {
        return rules;
    };

    replace_root_vars(root_idx, &mut rules);

    rules
}

fn replace_root_vars(root_idx: usize, rules: &mut Vec<Rule>) {
    let root = &mut rules[root_idx];
    let mut indexes = SmallVec::<[usize; 16]>::new();
    let mut keys = SmallVec::<[String; 16]>::new();
    let mut values = SmallVec::<[String; 16]>::new();
    find_vars(root, &mut indexes, &mut keys, &mut values);
    remove_vars(root, &indexes);
    let mut replace_indexes = SmallVec::<[usize; 16]>::new();
    for rule in rules {
        replace_indexes.clear();
        let indexes_iter = rule
            .values
            .iter()
            .enumerate()
            .filter_map(|(i, v)| is_var_reference(v).then_some(i));
        replace_indexes.extend(indexes_iter);
        for idx in replace_indexes.iter() {
            replace_var(&mut rule.values[*idx], &keys, &values);
        }
    }
}

fn replace_var(value: &mut Cow<'_, str>, keys: &[String], values: &[String]) {
    let var_name = get_var_name(value);
    if let Some(idx) = keys.iter().position(|k| k == var_name) {
        *value = Cow::Owned(values[idx].clone());
    }
}

fn remove_vars(root: &mut Rule, indexes: &[usize]) {
    // Important to reverse the iterator
    for index in indexes.iter().rev() {
        root.remove(*index);
    }
}

fn find_vars(
    root: &mut Rule,
    indexes: &mut SmallVec<[usize; 16]>,
    keys: &mut SmallVec<[String; 16]>,
    values: &mut SmallVec<[String; 16]>,
) {
    for (i, (k, v)) in root.iter_props().enumerate() {
        if is_var_definition(k) {
            indexes.push(i);
            keys.push(k.to_string());
            values.push(v.to_string());
        }
    }
}

fn is_var_definition(value: &str) -> bool {
    value.starts_with("--")
}

fn is_var_reference(value: &str) -> bool {
    value.ends_with(')') && value.starts_with("var(")
}

#[cold]
#[inline(never)]
fn get_var_name(value: &str) -> &str {
    &value[4..value.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_vars_ok() {
        // Setup initial rules with variables
        let rules = vec![
            Rule {
                selectors: SmallVec::from_vec(vec![Selector {
                    selector: ":root",
                    pseudo_class: None,
                }]),
                properties: SmallVec::from_vec(vec![Cow::Borrowed("--main-color")]),
                values: SmallVec::from_vec(vec![Cow::Borrowed("blue")]),
            },
            Rule {
                selectors: SmallVec::from_vec(vec![Selector {
                    selector: ".button",
                    pseudo_class: None,
                }]),
                properties: SmallVec::from_vec(vec![Cow::Borrowed("background-color")]),
                values: SmallVec::from_vec(vec![Cow::Borrowed("var(--main-color)")]),
            },
        ];

        // Call the function to replace variables
        let updated_rules = replace_vars(rules);

        // Check that the variable was replaced correctly
        assert_eq!(updated_rules[1].values[0], Cow::Borrowed("blue"));
    }
}
