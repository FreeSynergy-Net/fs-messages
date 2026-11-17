# FreeSynergy Messages — say something to someone

**One place for mail, chat and agents, and a message that is only sent if it can be sent encrypted.**

FreeSynergy Messenger treats a message as a message, whichever network brought it. Mail, chat and AI agents are different connectors under one program. You read, answer, search and file in one place, and the connectors do the talking to the outside world.

What is different here: it is encrypted only. If no way to encrypt a message to all its recipients exists, it is not sent, and there is no override. Incoming messages are marked with what the connector really knows, and anything not proven end-to-end is never shown as secure. The original of every mail is kept byte for byte. AIs are participants, not hidden listeners. Each one is marked, none acts without a release from you, and there is an emergency stop that wins over everything.

## What it can do today

Latest tag: `v0.0.2` (the core only; the tag of 0.0.3 is set at release). The version in this tree is `0.0.3` (program with a window, server and frontend).

- **A window and a server (new in 0.0.3).** `fs-messages` makes sure the server runs and opens the frontend `simple`: the name, the icon, what works today and what comes. Everything that does not work yet says "coming soon"; a missing server or a server that does not fit is said in the window, never a grey window. The window shows no messages yet: the core is not wired to it. Try `fs-messages --version`, `fs-messages --help`.

- A message model for mail, chat and agents; conversations, paging that does not jump when new mail arrives, unread marks, drafts, search, pin, move, flag, forward.
- Encrypted-only sending, with a clear refusal naming the recipients that cannot be reached.
- Messages stored with their original; very large mails leave a visible stub, not a hole.
- A mail connector (`fs-messages-mail`): fetch over IMAP or JMAP, send over SMTP or JMAP, OpenPGP with Autocrypt. Tested against a local Stalwart test server only.
- An AI guard (`fs-messages-guard`): an invitation for each AI, a release per AI and per kind of action, questions you answer for risky steps, secrets never given to an AI, and an emergency stop.
- Try the core: `cargo run -p fs-messages-core --example demo`.

## What is not there yet

- The window shows no mail and no chat yet, no account can be added in it, and there is no Telegram. The server answers only what it offers (nothing yet) and the contract version.
- A Matrix connector exists as a library (`fs-messages-matrix`, tested against one test server only); it is not wired to the window.
- Messages stay on the device they arrived on; "on all devices" is not met, on purpose, until the key vault exists.
- Editing and reacting to a message, scheduled sending, mail rules, HTML bodies, key discovery, STARTTLS and S/MIME answer "not possible".
- Account passwords cannot be stored yet because there is no key vault.
- The rule that all members must agree before an AI joins a conversation is not built. The guard's rules are provisional.
- Contract: draft (`messenger.capnp`, not locked). The basics are released with this draft contract. This frontend and server are its first consumers.

## Part of FreeSynergy

Messenger is where you talk, and later where you talk to an AI that works with the other programs. FreeSynergy is a family of programs that are free for everyone, forever, and that keep your data yours. The documentation, the manifesto and the overall plan live in [fs-documentation](https://github.com/FreeSynergy/fs-documentation); the reading version is at <https://freesynergy.github.io/fs-documentation/>. Start with the [manifesto](https://freesynergy.github.io/fs-documentation/manifesto/) if you want to know why we do this.


## For developers

- Workspace crates: `fs-messages-core` (core, no GUI, no network), `fs-messages-guard` (the AI guard), `fs-messages-mail` (the mail connector), `fs-messages-matrix` (the Matrix connector), `fs-messages-server` (the contract over a local socket), `fs-messages-client` (the contract as a client), `fs-messages-simple` (the frontend `simple`, a process of its own, depends only on the client and fs-components), `fs-messages-bin` (the command `fs-messages`: starter and `serve`).
- Engine and frontend are separate processes (ADR 0021, 0094 draft); the manifest of the frontend is `packaging/frontends/simple.metainfo.xml`.
- The contract is `messenger.capnp` in [fs-interfaces](https://github.com/FreeSynergy/fs-interfaces) (draft). Messenger owns it.
- Build and test: `cargo build --workspace` and `cargo test --workspace` (the window crates need the system libraries of Blinc; the tests do not open a window). The mail tests need the test servers in `scripts/test-servers`.
- Design: [DESIGN.md](DESIGN.md); each of the other two crates has its own `DESIGN.md`.
- Licence: MIT OR Apache-2.0.

## Auf Deutsch

**Ein Ort für Mail, Chat und KI-Agenten, und eine Nachricht wird nur gesendet, wenn sie verschlüsselt gesendet werden kann.** FreeSynergy Messages behandelt eine Nachricht als Nachricht, egal welches Netz sie gebracht hat. Es gibt keine Ausnahme vom Verschlüsseln; KI-Agenten sind sichtbare Teilnehmer, keine heimlichen Mitleser, und ein Notaus geht vor allem.

**Was es heute kann.** Letzter Tag `v0.0.2` (nur der Kern). Der Stand in diesem Baum ist `0.0.3` (Programm mit Fenster, Server und Frontend). Der Kern (Nachrichtenmodell, nur verschlüsselt senden, Mail-Anschluss, Matrix-Anschluss als Bibliothek, KI-Wächter) liegt als Bibliothek vor. Neu ist ein Fenster mit Server: `fs-messages` startet den Server und das Frontend `simple` mit Name, Symbol, „Was es heute kann“ und „Was kommt“. Was noch nicht geht, sagt „kommt noch“; fehlt der Server oder passt er nicht, steht das im Fenster. Das Fenster zeigt noch keine Nachrichten, der Kern ist noch nicht daran angeschlossen.

**Was noch nicht da ist.** Konten im Fenster hinzufügen, Nachrichten lesen und schreiben, Chat-Räume, Benachrichtigungen, KI als Teilnehmer, ein Moderator und ein geschützter Raum (geplant, kein Versprechen). Die vollständige Liste steht in der Feature-Liste (`todo/features/messages.md` in fs-documentation, sobald sie dort liegt).

**Für Entwickler.** Frontend und Engine sind getrennte Prozesse; das Frontend hängt nur am Client und an fs-components. Bauen: `cargo build --workspace`, `cargo test --workspace`. Vertrag: Entwurf (`messenger.capnp`, nicht gesperrt). Die Grundfassung erscheint mit diesem Entwurf. Lizenz: MIT OR Apache-2.0.
