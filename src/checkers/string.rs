use std::collections::HashMap;
use Role;
use super::TokenChecker;

pub enum Rule {
    Once,
    Multiple,
}

pub struct StringChecker<T> {
    tokens: HashMap<String, (Rule, Box<Fn() -> T>)>,
}

impl<T> StringChecker<T> {
    pub fn new() -> Self {
        StringChecker {
            tokens: HashMap::new(),
        }
    }

    pub fn add_token<F>(&mut self, rule: Rule, token: &str, generator: F)
        where F: Fn() -> T + 'static {
        self.tokens.insert(token.to_owned(), (rule, Box::new(generator)));
    }
}

impl<T: Role> TokenChecker<T> for StringChecker<T> {
    fn get_role_for_token(&mut self, token: &str) -> Option<T> {
        let (result, remove) = match self.tokens.get(token) {
            Some(&(Rule::Multiple, ref role)) => (Some(role()), false),
            Some(&(Rule::Once, ref role)) => (Some(role()), true),
            None => (None, false),
        };
        if remove {
            self.tokens.remove(token);
        }
        result
    }
}
