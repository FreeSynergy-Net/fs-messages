//! `Draft`: a message being written. Sending runs the encrypted-only plan (ADR 0018); scheduling is not built.
//!
//! Pattern: Registry entry (`DescribesType`).

use fs_types::{BasicType, CommandKind, DescribesType, TypeDescription};

use super::PROGRAM;
use super::build::{choice, command, field, on_record, record, text};

/// The `Draft` type.
pub struct DraftType;

impl DescribesType for DraftType {
    fn describe() -> TypeDescription {
        let mut t = TypeDescription::new("Draft", PROGRAM, "AS2 Note", "draft");
        for f in [
            field("to", BasicType::Email).multiple(),
            field("cc", BasicType::Email).multiple(),
            field("bcc", BasicType::Email).multiple(),
            text("subject"),
            field("text", BasicType::MultilineText),
            choice("mediaType", &["text-plain", "text-markdown"]),
        ] {
            t = t.field(f);
        }
        for c in [
            command("draft", "create", CommandKind::Create)
                .input(record("replyTo", "Message"))
                .result(record("created", "Draft")),
            command("draft", "change", CommandKind::Change),
            on_record("draft", "plan", CommandKind::Check, "Draft")
                .result(text("encryption"))
                .result(field("missingKeys", BasicType::Email).multiple()),
            on_record("draft", "save", CommandKind::Other, "Draft"),
            on_record("draft", "send", CommandKind::Other, "Draft")
                .result(record("sent", "Message")),
            on_record("draft", "schedule", CommandKind::Other, "Draft")
                .input(field("at", BasicType::DateTime).required())
                .unsupported(),
            on_record("draft", "discard", CommandKind::Delete, "Draft"),
        ] {
            t = t.command(c);
        }
        t
    }
}
