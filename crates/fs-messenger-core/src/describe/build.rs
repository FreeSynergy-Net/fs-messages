//! Small builders so each type file reads as a list. Labels are translation keys (`coding.md` section 2a):
//! `messenger.<field>` and `<type>.cmd.<command>`.
//!
//! Pattern: Builder, thin helpers over the `fs-types` builders.

use fs_types::{BasicType, CommandDescription, CommandKind, FieldDescription};

fn kebab(name: &str) -> String {
    let mut out = String::new();
    for c in name.chars() {
        if c.is_ascii_uppercase() {
            out.push('-');
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

pub(super) fn field(name: &str, basic: BasicType) -> FieldDescription {
    FieldDescription::new(name, basic).label(&format!("messenger.{}", kebab(name)))
}

pub(super) fn text(name: &str) -> FieldDescription {
    field(name, BasicType::Text)
}

pub(super) fn choice(name: &str, keys: &[&str]) -> FieldDescription {
    field(name, BasicType::Choice).choices(keys)
}

/// A reference to a record of `target`.
pub(super) fn record(name: &str, target: &str) -> FieldDescription {
    field(name, BasicType::Reference).target(target)
}

/// A yes/no value: the basic types have no boolean, so it is a choice of two keys.
pub(super) fn yes_no(name: &str) -> FieldDescription {
    choice(name, &["yes", "no"])
}

pub(super) fn command(scope: &str, name: &str, kind: CommandKind) -> CommandDescription {
    CommandDescription::new(name, kind, &format!("{scope}.cmd.{}", kebab(name)))
}

/// A command on one record: its first input is the reference to `target`.
pub(super) fn on_record(
    scope: &str,
    name: &str,
    kind: CommandKind,
    target: &str,
) -> CommandDescription {
    command(scope, name, kind).input(record(scope, target).required())
}
