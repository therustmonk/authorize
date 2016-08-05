//! Generic authorization routines.
#[cfg(feature = "hashing")]
extern crate crypto;

pub mod checkers;
#[cfg(feature = "hashing")]
pub mod cipher;

/// Marker trait of role.
/// Implement it for your struct or enum to represent roles in your services.
pub trait Role { }

/// Implementor should be authorized.
pub trait Authorize<T: Role> {
    fn set_role(&mut self, role: Option<T>) -> Option<T>;
}
