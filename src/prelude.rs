pub use std::collections::{HashMap, HashSet};
pub use crate::Policy::PermPolicy;


#[derive(Debug, Default, Clone, Copy)]
pub enum PermResult {
    Unsure,
    Deny,
    Allow,
}