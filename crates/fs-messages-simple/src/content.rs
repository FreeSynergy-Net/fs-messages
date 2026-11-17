//! What the window says, as blocks of the shared text view: the state, what works today, what comes.
//!
//! Pattern: Builder: a pure function from the words and the link state to blocks, so a test can read the window.

use fs_components_mf2::Mf2Texts;
use fs_components_widgets::{Block, Inline, ListItem};
use fs_components_words::{Args, Words};

use crate::link::Link;

/// How many entries the lists of `today-*` and `coming-*` have; a test checks that every id exists in both languages.
pub const TODAY: usize = 3;
/// See [`TODAY`].
pub const COMING: usize = 8;
/// The items that are not decided yet (moderator and protected space, calls): they say "planned, not scheduled" in their own words instead of "coming soon".
const OPEN: [usize; 2] = [7, 8];

fn say(texts: &Mf2Texts, id: &str, args: &Args) -> String {
    texts.say(&id.into(), args).text().to_owned()
}

fn plain(texts: &Mf2Texts, id: &str) -> String {
    say(texts, id, &Args::new())
}

fn list(items: Vec<String>) -> Block {
    Block::List {
        ordered: false,
        start: 1,
        items: items
            .into_iter()
            .map(|t| ListItem::new(vec![Block::Paragraph(Inline::plain(t))]))
            .collect(),
    }
}

/// The start page below the header: the server state and what works today.
#[must_use]
pub fn start(texts: &Mf2Texts, link: &Link) -> Vec<Block> {
    let (state, hint) = match link {
        Link::Connected { contract } => (
            say(
                texts,
                "state-connected",
                &Args::new().text("contract", contract.as_str()),
            ),
            None,
        ),
        Link::Mismatch(reason) => (
            say(
                texts,
                "state-contract",
                &Args::new().text("reason", reason.as_str()),
            ),
            Some(plain(texts, "state-hint-contract")),
        ),
        Link::Down(reason) => (
            say(
                texts,
                "state-down",
                &Args::new().text("reason", reason.as_str()),
            ),
            Some(plain(texts, "state-hint-down")),
        ),
    };
    vec![
        Block::Quote(
            std::iter::once(Block::Paragraph(Inline::plain(state)))
                .chain(hint.map(|h| Block::Paragraph(Inline::plain(h))))
                .collect(),
        ),
        Block::Heading {
            level: 2,
            text: Inline::plain(plain(texts, "today-title")),
        },
        list(
            (1..=TODAY)
                .map(|n| plain(texts, &format!("today-{n}")))
                .collect(),
        ),
    ]
}

/// The page "what comes": every item locked with "coming soon".
#[must_use]
pub fn coming(texts: &Mf2Texts) -> Vec<Block> {
    let soon = plain(texts, "soon");
    vec![
        Block::Heading {
            level: 2,
            text: Inline::plain(plain(texts, "coming-title")),
        },
        list(
            (1..=COMING)
                .map(|n| {
                    let said = plain(texts, &format!("coming-{n}"));
                    if OPEN.contains(&n) {
                        said
                    } else {
                        format!("{said} ({soon})")
                    }
                })
                .collect(),
        ),
    ]
}

/// Both pages as one list, for the tests that read the whole window.
#[must_use]
pub fn blocks(texts: &Mf2Texts, link: &Link) -> Vec<Block> {
    let mut all = start(texts, link);
    all.extend(coming(texts));
    all
}

#[cfg(test)]
mod tests {
    use super::{COMING, OPEN, TODAY, blocks, plain};
    use crate::link::Link;
    use crate::words::words;
    use fs_components_words::Locale;

    fn flat(blocks: &[super::Block]) -> String {
        format!("{blocks:?}")
    }

    #[test]
    fn every_listed_item_exists_and_every_coming_item_is_locked() {
        let texts = words(&Locale::english());
        for n in 1..=TODAY {
            assert_ne!(plain(&texts, &format!("today-{n}")), "");
        }
        let said = flat(&blocks(
            &texts,
            &Link::Connected {
                contract: "c".into(),
            },
        ));
        assert_eq!(said.matches("coming soon").count(), COMING - OPEN.len());
        assert_eq!(said.matches("not scheduled").count(), OPEN.len());
    }

    #[test]
    fn a_missing_server_is_said_in_the_window_with_the_reason() {
        let texts = words(&Locale::english());
        let said = flat(&blocks(&texts, &Link::Down("nobody home".into())));
        assert!(said.contains("No server is running: nobody home"), "{said}");
    }

    #[test]
    fn a_wrong_contract_is_said_in_the_window_with_the_reason() {
        let texts = words(&Locale::english());
        let said = flat(&blocks(&texts, &Link::Mismatch("speaks 9.0".into())));
        assert!(
            said.contains("does not fit the server: speaks 9.0"),
            "{said}"
        );
    }

    #[test]
    fn the_moderator_and_the_protected_space_are_named_without_a_promise() {
        let texts = words(&Locale::english());
        let said = flat(&blocks(
            &texts,
            &Link::Connected {
                contract: "c".into(),
            },
        ));
        assert!(said.contains("moderator") && said.contains("no promise"));
    }
}
