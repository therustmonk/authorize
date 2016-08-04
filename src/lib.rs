//! Generic authorization routines.

pub mod checkers;

/// Marker trait of role.
/// Implement it for your struct or enum to represent roles in your services.
pub trait Role { }

/// Implementor should be authorized.
pub trait Authorize<T: Role> {
    fn set_role(&mut self, role: Option<T>) -> Option<T>;
}
