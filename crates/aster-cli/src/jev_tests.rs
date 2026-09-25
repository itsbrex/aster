#[cfg(feature = "jev")]
use super::*;

#[cfg(feature = "jev")]
#[test]
fn snapshot_keeps_the_last_entries_bounded() {
    let wire: Vec<serde_json::Value> = (0..10)
        .map(|i| serde_json::json!({ "role": "assistant", "content": format!("round {i}") }))
        .collect();
    let snap = snapshot(&wire);
    assert!(snap.contains("round 9"));
    assert!(!snap.contains("round 3"));
    let long = "x".repeat(2000);
    let wire = vec![serde_json::json!({ "role": "user", "content": long })];
    assert!(snapshot(&wire).chars().count() < 600);
}
