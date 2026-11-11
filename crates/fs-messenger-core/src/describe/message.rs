//! `Message`: one mail, chat message or agent turn. Standard: AS2 `Note` (envelope + content + immutable original,
//! ADR 0020, 0030).
//!
//! Pattern: Registry entry (`DescribesType`). `react` and `edit` are listed but unsupported: no provider carries them yet
//! (`DESIGN.md`, "Not built").

use fs_types::{BasicType, CommandKind, DescribesType, TypeDescription};

use super::PROGRAM;
use super::build::{choice, command, field, on_record, record, text, yes_no};

/// The `Message` type.
pub struct MessageType;

impl DescribesType for MessageType {
    fn describe() -> TypeDescription {
        let mut t = TypeDescription::new("Message", PROGRAM, "AS2 Note", "message");
        for f in [
            record("conversation", "Conversation").read_only(),
            record("inReplyTo", "Message").read_only(),
            field("published", BasicType::DateTime).read_only(),
            text("preview").read_only(),
            field("bodyText", BasicType::MultilineText).read_only(),
            choice("senderKind", &["person", "software", "ai", "unknown"]).read_only(),
            choice(
                "state",
                &["unknown", "sending", "sent", "delivered", "read", "failed"],
            )
            .read_only(),
            yes_no("unread").read_only(),
            yes_no("flagged").read_only(),
            yes_no("undecrypted").read_only(),
        ] {
            t = t.field(f);
        }
        for c in [
            command("message", "find", CommandKind::Find)
                .input(text("term"))
                .input(text("from"))
                .input(yes_no("unread"))
                .input(yes_no("hasAttachment"))
                .input(field("since", BasicType::DateTime))
                .result(record("items", "Message").multiple()),
            on_record("message", "delete", CommandKind::Delete, "Message"),
            on_record("message", "flag", CommandKind::Other, "Message")
                .input(yes_no("flagged").required()),
            on_record("message", "unread", CommandKind::Other, "Message")
                .input(yes_no("unread").required()),
            on_record("message", "forward", CommandKind::Other, "Message")
                .input(field("recipients", BasicType::Email).required().multiple()),
            on_record("message", "react", CommandKind::Other, "Message")
                .input(text("key"))
                .unsupported(),
            on_record("message", "edit", CommandKind::Other, "Message")
                .input(field("text", BasicType::MultilineText).required())
                .unsupported(),
        ] {
            t = t.command(c);
        }
        t
    }
}
