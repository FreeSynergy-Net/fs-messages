//! The data types this program owns, as a frontend can read them (ADR 0066 point 3): `Conversation`, `Message`,
//! `Account`, `Draft`. Mirrors `Messenger.types` of `messenger.capnp`; no RPC here, the server only hands this over.
//!
//! Pattern: Registry (`fs-types` `TypeRegistry`) of entries that implement the trait `DescribesType`; small Builder
//! helpers in `build.rs`. A command the core cannot do yet is listed with `supported = false` (provider rule, ADR 0045):
//! it is shown locked, and freed by the core without a change to a frontend. **No account secret appears in any field or
//! input**: the contract says so, and a test holds it.

mod account;
mod build;
mod conversation;
mod draft;
mod message;

use fs_types::{DescriptionProblem, TypeDescription, TypeRegistry};

pub use account::AccountType;
pub use conversation::ConversationType;
pub use draft::DraftType;
pub use message::MessageType;

/// The program that owns the types (ADR 0050).
pub(crate) const PROGRAM: &str = "fs-messenger";

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
