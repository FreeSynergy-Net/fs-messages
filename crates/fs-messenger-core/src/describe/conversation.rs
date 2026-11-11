//! `Conversation`: a thread of messages (mail thread, Matrix room, agent session). Standard: AS2 `Collection` of `Note`
//! (ADR 0020), `context` = conversation id.
//!
//! Pattern: Registry entry (`DescribesType`).

use fs_types::{BasicType, CommandKind, DescribesType, TypeDescription};

use super::PROGRAM;
use super::build::{choice, command, field, on_record, record, text, yes_no};

/// The `Conversation` type.
pub struct ConversationType;

impl DescribesType for ConversationType {
    fn describe() -> TypeDescription {
        let mut t = TypeDescription::new("Conversation", PROGRAM, "AS2 Collection", "conversation");
        for f in [
            text("name").read_only(),
            choice("kind", &["mail", "chat", "comment", "unknown"]).read_only(),
            choice(
                "encryption",
                &[
                    "none",
                    "transport-only",
                    "megolm",
                    "openpgp",
                    "mls",
                    "unknown",
                ],
            )
            .read_only(),
            field("unreadCount", BasicType::Text).read_only(),
            yes_no("pinned").read_only(),
            field("folders", BasicType::Keywords).read_only(),
        ] {
            t = t.field(f);
        }
        for c in [
            command("conversation", "find", CommandKind::Find)
                .input(text("term"))
                .input(text("folder"))
                .input(yes_no("onlyUnread"))
                .result(record("items", "Conversation").multiple()),
            on_record(
                "conversation",
                "markRead",
                CommandKind::Change,
                "Conversation",
            )
            .input(record("upTo", "Message").required()),
            on_record("conversation", "pin", CommandKind::Other, "Conversation")
                .input(yes_no("pinned").required()),
            on_record("conversation", "move", CommandKind::Other, "Conversation")
                .input(text("folder").required()),
            on_record(
                "conversation",
                "nextUnread",
                CommandKind::Find,
                "Conversation",
            )
            .result(record("message", "Message")),
            on_record("conversation", "stop", CommandKind::Other, "Conversation"),
            on_record("conversation", "usage", CommandKind::Check, "Conversation")
                .result(field("inputTokens", BasicType::Text))
                .result(field("outputTokens", BasicType::Text))
                .result(field("costMicros", BasicType::Text))
                .result(text("currency")),
            on_record("conversation", "export", CommandKind::Other, "Conversation")
                .result(field("document", BasicType::MultilineText)),
        ] {
            t = t.command(c);
        }
        t
    }
}
