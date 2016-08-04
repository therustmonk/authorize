use super::Role;

pub mod string;
pub use self::string::Rule;
pub use self::string::StringChecker;

pub trait TokenChecker<T: Role> {
    fn get_role_for_token(&mut self, token: &str) -> Option<T>;
}

pub trait CredentialChecker<T: Role> {
    fn get_role_for_credential(&mut self, login: &str, password: &str) -> Option<T>;
}
