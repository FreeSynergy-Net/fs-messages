//! The type description of the messenger: valid, resolved, no secret, honest about what is not built.

use fs_messenger_core::describe::{describe_types, type_registry};

#[test]
fn all_four_types_validate_and_resolve() {
    let r = type_registry().expect("registry");
    assert_eq!(r.len(), 4);
    assert!(
        r.unresolved_targets().is_empty(),
        "{:?}",
        r.unresolved_targets()
    );
    assert_eq!(describe_types().len(), 4);
}

#[test]
fn no_account_secret_anywhere() {
    for t in describe_types() {
        let mut names: Vec<&str> = t.fields.iter().map(|f| f.name.as_str()).collect();
        for c in &t.commands {
            names.extend(c.input.iter().chain(&c.result).map(|f| f.name.as_str()));
        }
        assert!(
            !names
                .iter()
                .any(|n| n.to_lowercase().contains("secret")
                    || n.to_lowercase().contains("password")),
            "{}: {names:?}",
            t.name
        );
    }
}

#[test]
fn what_is_not_built_is_listed_as_unsupported() {
    let r = type_registry().unwrap();
    let m = r.get("Message").unwrap();
    for name in ["react", "edit"] {
        assert!(!m.find_command(name).unwrap().supported, "{name}");
    }
    assert!(m.find_command("delete").unwrap().supported);
    assert!(
        !r.get("Draft")
            .unwrap()
            .find_command("schedule")
            .unwrap()
            .supported
    );
}

#[test]
fn record_commands_name_their_own_type() {
    let r = type_registry().unwrap();
    let pin = r.get("Conversation").unwrap().find_command("pin").unwrap();
    assert_eq!(pin.input[0].target, "Conversation");
    assert!(pin.input[0].required);
}
