//use uuid::Uuid;
use crypto::pbkdf2;

pub fn hash_password(password: &str) -> String {
    match pbkdf2::pbkdf2_simple(password, 22156) {
        Ok(hash) => hash,
        Err(_) => "*".to_owned(),
    }
}

pub fn check_password(password: &str, hash: &str) -> bool {
    match pbkdf2::pbkdf2_check(password, hash) {
        Ok(res) => res,
        Err(_) => false,
    }
}

/*
pub fn generate_token() -> String {
    let uuid = Uuid::new_v4();
    uuid.to_simple_string()
}
*/
