//! The window: name, the real icon (embedded PNG, own design), the content.
//!
//! Pattern: Adapter: our blocks in, a screen of the fs-components shell out; no widget is painted by hand.

use std::sync::Arc;

use fs_components::blinc::layout::div::div;
use fs_components::blinc::layout::image::image;
use fs_components_app::{AppError, AppWidget, Element, Menu, MenuEntry, Screen, Status, Surface};
use fs_components_mf2::Mf2Texts;
use fs_components_widgets::{Block, Inline, TextView, styles};
use fs_components_words::{Args, Words};

use crate::content::{coming, start};
use crate::link::Link;

/// The icon of the program, embedded in the binary.
const ICON: &[u8] = include_bytes!("../assets/fs-messages.png");

fn icon_uri() -> String {
    use base64::Engine;
    format!(
        "data:image/png;base64,{}",
        base64::engine::general_purpose::STANDARD.encode(ICON)
    )
}

/// The one screen of the window.
struct Home {
    name: String,
    tagline: String,
    body: Vec<Block>,
    /// The start page carries the header with the icon; the page "what comes" is the list alone.
    header: bool,
}

impl Surface for Home {
    fn build(&self, _width: f32, _height: f32) -> Box<dyn Element> {
        // The icon of the program (own design, embedded; see packaging/ICON.md), as a data address Blinc loads.
        let icon = image(icon_uri()).size(96.0, 96.0).no_fade();
        let heading = TextView::new(vec![
            Block::Heading {
                level: 1,
                text: Inline::plain(self.name.as_str()),
            },
            Block::Paragraph(Inline::plain(self.tagline.as_str())),
        ])
        .build();
        let header = div().flex_row().gap(16.0).child(icon).child(heading);
        let body = TextView::new(self.body.clone()).build();
        let page = div().flex_col().gap(12.0).p(24.0);
        Box::new(if self.header {
            page.child(header).child(body)
        } else {
            page.child(body)
        })
    }

    fn styles(&self) -> String {
        styles()
    }
}

/// The window for `link`, with the menus whose entries are all locked ("coming soon").
///
/// # Errors
/// The shell could not be made (a name that is empty, for example).
pub fn app(texts: &Mf2Texts, link: &Link) -> Result<AppWidget, AppError> {
    let say = |id: &str| texts.say(&id.into(), &Args::new()).text().to_owned();
    let soon = say("soon");
    let page = |header: bool, body: Vec<Block>| {
        Arc::new(Home {
            name: say("app-name"),
            tagline: say("app-tagline"),
            body,
            header,
        })
    };
    let widget = AppWidget::new(
        say("app-name"),
        vec![
            Screen::new("home", say("screen-start"), page(true, start(texts, link))),
            Screen::new("coming", say("screen-coming"), page(false, coming(texts))),
        ],
    )?;
    let locked = |id: &str| {
        MenuEntry::new(id, format!("{} ({soon})", say(&format!("menu-{id}")))).disabled()
    };
    widget.set_menus(
        vec![Menu::new(
            say("menu-program"),
            vec![locked("new"), locked("accounts"), locked("search")],
        )],
        |_| {},
    )?;
    widget.set_status(Some(Status::Info(say("status-ready"))));
    Ok(widget)
}
