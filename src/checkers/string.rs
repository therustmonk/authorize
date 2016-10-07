use std::error;
use std::collections::HashMap;
use Role;
use super::*;

enum Rule<T: Role> {
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

    fn add_rule(&mut self, token: &str, rule: Rule<T>) {
        self.tokens.insert(token.to_owned(), rule);
    }

    pub fn add_once(&mut self, token: &str, role: T) {
        let rule = Rule::Once(Some(role));
        self.add_rule(token, rule);
    }
}

impl<T: Role + Clone + Send + 'static> StringChecker<T> {
    pub fn add_multiple(&mut self, token: &str, role: T) {
        let generator = move || Some(role.clone());
        let rule = Rule::Multiple(Box::new(generator));
        self.add_rule(token, rule);
    }
}

impl<T: Role, E: error::Error> TokenChecker<T, E> for StringChecker<T> {
    fn get_role_for_token(&mut self, token: &str) -> Result<T, E> {
        let (result, remove) = match self.tokens.get_mut(token) {
            Some(&mut Rule::Multiple(ref generator)) => (generator(), false),
            Some(&mut Rule::Once(ref mut role)) => (role.take(), true),
            None => (None, false),
        };
        if remove {
            self.tokens.remove(token);
        }
        Ok(result)
    }
}
