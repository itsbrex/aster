use super::*;
use base64::Engine as _;
use std::fs;

fn fake_jwt(claims: &serde_json::Value) -> String {
    let enc = |v: &serde_json::Value| {
        base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode(serde_json::to_vec(v).expect("serialize"))
    };
    format!(
        "{}.{}.sig",
        enc(&serde_json::json!({"alg": "none"})),
        enc(claims)
    )
}

#[test]
fn finds_a_codex_cli_sign_in_with_its_account() {
    let home = tempfile::tempdir().expect("tempdir");
    let auth = codex::CodexAuth {
        tokens: Some(codex::TokenSet {
            id_token: fake_jwt(&serde_json::json!({"email": "me@example.com"})),
            access_token: "at".into(),
            refresh_token: "rt".into(),
            account_id: None,
        }),
        ..Default::default()
    };
    let path = codex::codex_cli_path(home.path());
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, serde_json::to_vec(&auth).unwrap()).unwrap();

    assert_eq!(
        found(home.path()),
        vec![Login {
            provider: "codex",
            name: "ChatGPT",
            base_url: codex::CODEX_BASE_URL,
            account: Some("me@example.com".to_string()),
        }]
    );
}

#[test]
fn finds_nothing_in_an_empty_home() {
    let home = tempfile::tempdir().expect("tempdir");
    assert_eq!(found(home.path()), Vec::new());
}
