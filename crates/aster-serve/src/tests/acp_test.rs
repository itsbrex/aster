use std::time::Instant;

use super::*;

fn thinking_turn(chars: usize) -> Turn {
    Turn {
        event_id: "e1".into(),
        reply: String::new(),
        edits: Vec::new(),
        tool_names: HashMap::new(),
        permission: None,
        reasoning_chars: chars,
        reasoning_started: Some(Instant::now()),
        pending: Arc::default(),
    }
}

#[test]
fn each_thinking_block_counts_its_own_tokens() {
    let mut turn = thinking_turn(401);
    let done = turn.end_thinking().unwrap();
    assert_eq!(done["type"], "reasoning_done");
    assert_eq!(done["tokens"], 101);
    assert_eq!(turn.reasoning_chars, 0);
    assert!(turn.end_thinking().is_none());
}

#[test]
fn classifies_session_updates_as_notifications() {
    let message = json!({
        "jsonrpc": "2.0",
        "method": "session/update",
        "params": {
            "sessionId": "s1",
            "update": {
                "sessionUpdate": "agent_message_chunk",
                "content": { "type": "text", "text": "Hello" }
            }
        }
    });

    let Incoming::SessionUpdate(update) = classify(&message) else {
        panic!("session update was not classified as a notification");
    };
    assert_eq!(update["content"]["text"], "Hello");
}

#[test]
fn extracts_text_from_acp_content_block() {
    let content = json!({ "type": "text", "text": "Hello" });
    assert_eq!(content_text(&content), "Hello");
}
