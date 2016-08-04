use std::collections::HashMap;
use Role;
use super::TokenChecker;

pub enum Rule<T: Role> {
    Once(Option<T>),
    Multiple(Box<Fn() -> Option<T> + Send>),
}

pub struct StringChecker<T: Role> {
    tokens: HashMap<String, Rule<T>>,
}

impl<T: Role> StringChecker<T> {
    pub fn new() -> Self {
        StringChecker {
            tokens: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, token: &str, rule: Rule<T>) {
        self.tokens.insert(token.to_owned(), rule);
    }
}

impl<T: Role> TokenChecker<T> for StringChecker<T> {
    fn get_role_for_token(&mut self, token: &str) -> Option<T> {
        let (result, remove) = match self.tokens.get_mut(token) {
            Some(&mut Rule::Multiple(ref generator)) => (generator(), false),
            Some(&mut Rule::Once(ref mut role)) => (role.take(), true),
            None => (None, false),
        };
        if remove {
            self.tokens.remove(token);
        }
        result
    }
}
