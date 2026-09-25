use super::named;

#[test]
fn dark_alias_resolves_to_default() {
    let dark = named("dark").expect("dark remains a supported alias");
    let default = named("default").expect("default is a built-in theme");

    assert_eq!(dark.name, "default");
    assert_eq!(dark.theme.accent, default.theme.accent);
}
