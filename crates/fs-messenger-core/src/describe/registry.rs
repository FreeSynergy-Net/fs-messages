//! The registry of all messenger types.
//!
//! Pattern: Registry (`fs-types` `TypeRegistry`).

use fs_types::{DescriptionProblem, TypeDescription, TypeRegistry};

use super::{AccountType, ConversationType, DraftType, MessageType};

/// All types of the messenger, validated and with every reference resolved.
///
/// # Errors
/// A defect of a description (a bug of this crate; a test keeps it from shipping).
pub fn type_registry() -> Result<TypeRegistry, DescriptionProblem> {
    let mut registry = TypeRegistry::new();
    registry
        .register::<ConversationType>()?
        .register::<MessageType>()?
        .register::<AccountType>()?
        .register::<DraftType>()?;
    Ok(registry)
}

/// The descriptions as a list, in a fixed order: what `TypeCatalog.describe` answers.
pub fn describe_types() -> Vec<TypeDescription> {
    type_registry()
        .map(|r| r.iter().cloned().collect())
        .unwrap_or_default()
}
