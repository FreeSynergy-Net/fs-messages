# Changelog

Version = tag; all crates of the workspace carry the same version.

## 0.0.3

### English

- **Window and server (basics, workflow `grundfassung`).** New crates: `fs-messages-server` (the contract `messenger.capnp` draft 0.3 over a local socket through `fs-rpc` v0.0.2; answers `capabilities` with no capability ids and the contract version, and `supportReport`; everything else is `unimplemented`), `fs-messages-client` (connect with the version handshake, `Hello::fetch`, contract check: same main level, minor level of the server not lower), `fs-messages-simple` (the frontend `simple` as a process of its own: name, icon, what works today, what comes, every element locked with "coming soon"; depends only on the client and fs-components v0.0.25), `fs-messages-bin` (the command `fs-messages`: starter, `serve`, `--version`, `--help`).
- **Starter.** `fs-messages` starts or replaces the server (handshake of `fs-rpc`), asks the frontend to check the contract (`--check`, exit code 3 = does not fit) and falls back to the standard frontend `simple` with a visible message; a missing or unfitting server is said in the window.
- **Pins lifted so the tree holds one source per library:** `fs-objects` v0.0.1 to v0.0.6, `fs-interfaces` v0.0.1 to v0.0.27, new `fs-rpc` v0.0.2, `fs-types` v0.0.1 (through them); `.fs-lock-sources.toml` strict. Minimum Rust 1.89 for the workspace.
- **CI:** `check.yml` and `release.yml` call fs-release at `v0.0.10`; no tag exists for this state.
- The real icon (`packaging/net.freesynergy.Messages.png`, `.desktop`, embedded in the window); `fs-messages` depends on the package `fs-messages-simple` (deb `depends`, rpm `requires`).
- Contract: draft (`messenger.capnp`, not locked). The basics are released with this draft contract.
- Not done: the window shows no messages (the core is not wired to the server), no account setup, no lease (the server keeps running after the window closes until it is stopped or replaced).

### Deutsch


- **Fenster und Server (Grundfassung, Workflow `grundfassung`).** Neue Crates: `fs-messages-server` (der Vertrag `messenger.capnp` Entwurf 0.3 über einen lokalen Socket mit `fs-rpc` v0.0.2; antwortet auf `capabilities` ohne Fähigkeits-Kennungen mit der Vertragsversion und auf `supportReport`; alles andere ist `unimplemented`), `fs-messages-client` (Verbinden mit Versions-Handschlag, `Hello::fetch`, Vertragsprüfung: gleiche Hauptstufe, Nebenstufe des Servers nicht niedriger), `fs-messages-simple` (das Frontend `simple` als eigener Prozess: Name, Symbol, was heute geht, was kommt, jedes Element mit „kommt noch“ gesperrt; hängt nur am Client und an fs-components v0.0.25), `fs-messages-bin` (der Befehl `fs-messages`: Starter, `serve`, `--version`, `--help`).
- **Starter.** `fs-messages` startet oder ersetzt den Server (Handschlag von `fs-rpc`), lässt das Frontend den Vertrag prüfen (`--check`, Rückgabecode 3 = passt nicht) und fällt mit sichtbarer Meldung auf das Standard-Frontend `simple` zurück; ein fehlender oder unpassender Server steht im Fenster.
- **Pins angehoben, damit der Baum je Bibliothek eine Quelle hat:** `fs-objects` v0.0.1 auf v0.0.6, `fs-interfaces` v0.0.1 auf v0.0.27, neu `fs-rpc` v0.0.2, `fs-types` v0.0.1 (durch diese); `.fs-lock-sources.toml` streng. Mindest-Rust 1.89 für den Arbeitsbereich.
- **CI:** `check.yml` und `release.yml` rufen fs-release bei `v0.0.10` auf; für diesen Stand gibt es keinen Tag.
- Das echte Symbol (`packaging/net.freesynergy.Messages.png`, `.desktop`, im Fenster eingebettet); `fs-messages` hängt vom Paket `fs-messages-simple` ab (deb `depends`, rpm `requires`).
- Vertrag: Entwurf (`messenger.capnp`, nicht gesperrt). Die Grundfassung erscheint mit diesem Entwurf.
- Nicht getan: das Fenster zeigt keine Nachrichten (der Kern ist nicht am Server angeschlossen), keine Konto-Einrichtung, keine Lease (der Server läuft nach dem Schließen des Fensters weiter, bis er beendet oder ersetzt wird).
