use claude_code_history_viewer_lib::providers;

#[test]
fn kimi_provider_scans_projects_from_sessions_tree() {
    let base = fixture_base();

    let projects = providers::kimi::scan_projects_from_path(base.to_str().unwrap())
        .expect("scan_projects_from_path should parse fixture");

    assert_eq!(projects.len(), 1);
    let project = &projects[0];
    assert_eq!(project.name, "project-hash");
    assert_eq!(
        project.path,
        format!("kimi://{}", base.join("sessions/project-hash").display())
    );
    assert_eq!(project.actual_path, "project-hash");
    assert_eq!(project.session_count, 2);
    assert_eq!(project.message_count, 5);
    assert_eq!(project.provider.as_deref(), Some("kimi"));
    assert_eq!(project.storage_type.as_deref(), Some("jsonl"));
}

#[test]
fn kimi_provider_loads_sessions_with_titles_and_timestamps() {
    let base = fixture_base();
    let project_path = format!("kimi://{}", base.join("sessions/project-hash").display());

    let sessions =
        providers::kimi::load_sessions_from_base_path(base.to_str().unwrap(), &project_path, false)
            .expect("load_sessions_from_base_path should parse fixture");

    assert_eq!(sessions.len(), 2);
    let first = &sessions[0];
    assert_eq!(first.actual_session_id, "session-2");
    assert_eq!(first.summary.as_deref(), Some("Second session prompt"));
    assert_eq!(first.last_message_time, "2026-02-02T02:42:00+00:00");

    let second = &sessions[1];
    assert_eq!(second.actual_session_id, "session-1");
    assert_eq!(second.summary.as_deref(), Some("Implement Kimi provider"));
    assert!(second.has_tool_use);
    assert_eq!(second.message_count, 3);
    assert_eq!(second.provider.as_deref(), Some("kimi"));
}

#[test]
fn kimi_provider_loads_messages_without_internal_roles() {
    let base = fixture_base();
    let session_dir = base.join("sessions/project-hash/session-1");

    let messages = providers::kimi::load_messages_from_base_path(
        base.to_str().unwrap(),
        session_dir.to_str().unwrap(),
    )
    .expect("load_messages_from_base_path should parse fixture");

    assert_eq!(messages.len(), 4);
    assert!(messages
        .iter()
        .all(|m| m.provider.as_deref() == Some("kimi")));
    assert!(messages.iter().all(|m| m.message_type != "_system_prompt"));
    assert_eq!(messages[0].message_type, "user");
    assert_eq!(messages[1].message_type, "assistant");
    assert_eq!(messages[1].content.as_ref().unwrap()[0]["type"], "thinking");
    assert_eq!(
        messages[1].content.as_ref().unwrap()[0]["thinking"],
        "I should inspect the provider registry first."
    );
    assert_eq!(messages[2].message_type, "user");
    assert_eq!(
        messages[2].content.as_ref().unwrap()[0]["type"],
        "tool_result"
    );
    assert_eq!(messages[3].message_type, "assistant");
}

#[test]
fn kimi_provider_searches_messages_from_base_path() {
    let base = fixture_base();

    let results = providers::kimi::search_from_base_path(
        base.to_str().unwrap(),
        "inspect the provider registry",
        10,
    )
    .expect("search_from_base_path should parse fixture");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].message_type, "assistant");
    assert_eq!(results[0].project_name.as_deref(), Some("project-hash"));
}

fn fixture_base() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("kimi")
}
