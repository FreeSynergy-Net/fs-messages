//! `Account`: a transport (mail, Matrix, MIMI). The secret is **never** a field or an input here (contract rule); it
//! comes in through the key vault path (ADR 0024), not through a generated form.
//!
//! Pattern: Registry entry (`DescribesType`).

use fs_types::{CommandKind, DescribesType, TypeDescription};

use super::PROGRAM;
use super::build::{choice, command, field, on_record, yes_no};
use fs_types::BasicType;

/// The `Account` type.
pub struct AccountType;

impl DescribesType for AccountType {
    fn describe() -> TypeDescription {
        let mut t = TypeDescription::new("Account", PROGRAM, "AS2 Actor", "account");
        t = t
            .field(field("address", BasicType::Text).required())
            .field(choice("kind", &["mail", "matrix", "mimi"]).required())
            .field(yes_no("encryptionReady").read_only());
        for c in [
            command("account", "find", CommandKind::Find)
                .result(super::build::record("items", "Account").multiple()),
            command("account", "create", CommandKind::Create)
                .input(field("address", BasicType::Text).required())
                .input(choice("kind", &["mail", "matrix", "mimi"]).required())
                .result(super::build::record("created", "Account")),
            on_record("account", "delete", CommandKind::Delete, "Account"),
        ] {
            t = t.command(c);
        }
        t
    }
}
