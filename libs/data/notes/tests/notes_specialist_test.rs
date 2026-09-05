use phone_backup_notes::{
    ChecklistItem, JsonNoteExporter, JsonNoteParser, KeepJsonParser, MarkdownNoteExporter,
    MarkdownNoteParser, NoteFactory, NoteItemBuilder, NoteType, NotesAnalytics, NotesHtmlExporter,
};

#[test]
fn test_builder_and_factory() {
    let note1 = NoteFactory::create_text_note("Meeting Notes", "Discuss Q4 objectives.");
    assert_eq!(note1.title, "Meeting Notes");
    assert_eq!(note1.content, "Discuss Q4 objectives.");
    assert_eq!(note1.note_type, NoteType::TextNote);

    let checklist = NoteFactory::create_checklist(
        "Grocery",
        vec![("Apples", true), ("Bananas", false), ("Milk", false)],
    );
    assert_eq!(checklist.note_type, NoteType::Checklist);
    assert_eq!(checklist.checklist_items.len(), 3);
    assert_eq!(checklist.snippet(50), "[1/3 tasks completed]");

    let built = NoteItemBuilder::new()
        .title("Project Plan")
        .content("Milestone 1")
        .add_checklist_item(ChecklistItem::unchecked("Setup Repo"))
        .add_tag("work")
        .is_pinned(true)
        .color("#ff0000")
        .build()
        .expect("Build should succeed");

    assert_eq!(built.title, "Project Plan");
    assert!(built.is_pinned);
    assert_eq!(built.tags, vec!["work"]);
    assert_eq!(built.color.as_deref(), Some("#ff0000"));
}

#[test]
fn test_keep_json_parser() {
    let json = r#"{
  "color": "BLUE",
  "isTrashed": false,
  "isPinned": true,
  "isArchived": false,
  "textContent": "Buy new keyboard",
  "title": "Hardware Todo",
  "userEditedTimestampUsec": 1725450000000000,
  "createdTimestampUsec": 1725450000000000,
  "listContent": [
    { "text": "Mechanical Switches", "isChecked": true },
    { "text": "Keycaps", "isChecked": false }
  ],
  "labels": [ { "name": "hardware" }, { "name": "shopping" } ]
}"#;

    let note = KeepJsonParser::parse(json).expect("Keep JSON should parse");
    assert_eq!(note.title, "Hardware Todo");
    assert!(note.is_pinned);
    assert_eq!(note.note_type, NoteType::Checklist);
    assert_eq!(note.checklist_items.len(), 2);
    assert_eq!(note.checklist_items[0].text, "Mechanical Switches");
    assert!(note.checklist_items[0].is_checked);
    assert_eq!(note.tags, vec!["hardware", "shopping"]);
}

#[test]
fn test_markdown_parser() {
    let md = r#"# Sprint Tasks

Review pull requests and prepare release.

- [ ] Code review #backend
- [x] Run unit tests #ci
- [ ] Deploy to staging #devops
"#;

    let note = MarkdownNoteParser::parse("sprint.md", md);
    assert_eq!(note.title, "Sprint Tasks");
    assert!(note.content.contains("Review pull requests"));
    assert_eq!(note.checklist_items.len(), 3);
    assert_eq!(note.checklist_items[0].text, "Code review #backend");
    assert!(!note.checklist_items[0].is_checked);
    assert!(note.checklist_items[1].is_checked);
}

#[test]
fn test_analytics_and_exporters() {
    let note1 = NoteFactory::create_pinned_note("Idea", "Build AI tool", vec!["ai", "startup"]);
    let note2 = NoteFactory::create_checklist("Todo", vec![("Task 1", true), ("Task 2", false)]);

    let stats = NotesAnalytics::compute_stats(&[note1.clone(), note2.clone()]);
    assert_eq!(stats.total_notes, 2);
    assert_eq!(stats.pinned_count, 1);
    assert_eq!(stats.checklist_count, 1);
    assert_eq!(stats.total_tasks, 2);
    assert_eq!(stats.completed_tasks, 1);

    let html = NotesHtmlExporter::export("My Notes Wall", &[note1.clone(), note2.clone()]);
    assert!(html.contains("My Notes Wall"));
    assert!(html.contains("Idea"));
    assert!(html.contains("#ai"));

    let md_text = MarkdownNoteExporter::export_single(&note2);
    assert!(md_text.contains("# Todo"));
    assert!(md_text.contains("- [x] Task 1"));
    assert!(md_text.contains("- [ ] Task 2"));

    let json_text = JsonNoteExporter::export_pretty(&[note1]).expect("JSON export should succeed");
    let parsed_back = JsonNoteParser::parse_collection(&json_text);
    assert_eq!(parsed_back.len(), 1);
    assert_eq!(parsed_back[0].title, "Idea");
}
