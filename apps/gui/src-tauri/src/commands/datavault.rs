use crate::state::AppState;
use bookmarks::{BookmarkAnalytics, BookmarkItem, ChromiumBookmarksParser};
use calendar::{CalendarAnalytics, CalendarEvent, IcsParser};
use notes::{ChecklistItem, NoteItem, NoteItemBuilder, NotesAnalytics};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::State;
use wifi::{WifiAnalytics, WifiConfigStoreXmlParser, WifiNetworkItem, WifiQrGenerator};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WifiVaultDto {
    pub networks: Vec<WifiNetworkItem>,
    pub stats: wifi::WifiStats,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BookmarkVaultDto {
    pub bookmarks: Vec<BookmarkItem>,
    pub stats: bookmarks::BookmarkStats,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NotesVaultDto {
    pub notes: Vec<NoteItem>,
    pub stats: notes::NoteStats,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CalendarVaultDto {
    pub events: Vec<CalendarEvent>,
    pub stats: calendar::CalendarStats,
    pub conflicts: Vec<String>,
}

fn probe_file(name: &str) -> Option<PathBuf> {
    let candidates = [
        PathBuf::from("workspace").join(name),
        PathBuf::from("../../workspace").join(name),
        PathBuf::from("../workspace").join(name),
    ];
    for p in &candidates {
        if p.exists() {
            return Some(p.clone());
        }
    }
    None
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_wifi_vault(
    _state: State<'_, AppState>,
    _device_id: Option<String>,
) -> Result<WifiVaultDto, String> {
    let mut networks = Vec::new();
    if let Some(p) = probe_file("WifiConfigStore.xml") {
        if let Ok(c) = std::fs::read_to_string(p) {
            networks = WifiConfigStoreXmlParser::parse(&c);
        }
    }
    if networks.is_empty() {
        let sample = r#"<?xml version='1.0' encoding='utf-8' standalone='yes' ?>
<WifiConfigStoreData>
<NetworkList>
<Network><WifiConfiguration><string name="ConfigKey">&quot;Office_Fiber_5G&quot;WPA_PSK</string><string name="SSID">&quot;Office_Fiber_5G&quot;</string><string name="PreSharedKey">&quot;UltraSecret2026!&quot;</string><boolean name="HiddenSSID" value="false" /></WifiConfiguration></Network>
<Network><WifiConfiguration><string name="ConfigKey">&quot;Guest_Lounge&quot;NONE</string><string name="SSID">&quot;Guest_Lounge&quot;</string><boolean name="HiddenSSID" value="false" /></WifiConfiguration></Network>
<Network><WifiConfiguration><string name="ConfigKey">&quot;Home_Studio_Mesh&quot;WPA_PSK</string><string name="SSID">&quot;Home_Studio_Mesh&quot;</string><string name="PreSharedKey">&quot;KucingLucu123&quot;</string><boolean name="HiddenSSID" value="false" /></WifiConfiguration></Network>
</NetworkList>
</WifiConfigStoreData>"#;
        networks = WifiConfigStoreXmlParser::parse(sample);
    }
    let stats = WifiAnalytics::compute_stats(&networks);
    Ok(WifiVaultDto { networks, stats })
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_wifi_qr(ssid: String, password: Option<String>, security: String, hidden: bool) -> Result<String, String> {
    let sec = wifi::SecurityType::from_key_mgmt(&security);
    let mut builder = wifi::WifiNetworkBuilder::new(ssid)
        .security(sec)
        .hidden(hidden);
    if let Some(p) = password {
        builder = builder.psk(p);
    }
    let item = builder.build();
    Ok(WifiQrGenerator::generate_payload(&item))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_bookmarks_vault(
    _state: State<'_, AppState>,
    _device_id: Option<String>,
) -> Result<BookmarkVaultDto, String> {
    let mut bms = Vec::new();
    if let Some(p) = probe_file("Bookmarks.json") {
        if let Ok(c) = std::fs::read_to_string(p) {
            bms = ChromiumBookmarksParser::parse(&c, bookmarks::BrowserType::Chrome);
        }
    }
    if bms.is_empty() {
        let sample = r#"{
            "roots": {
                "bookmark_bar": {
                    "children": [
                        {"name": "Rust Documentation", "type": "url", "url": "https://doc.rust-lang.org"},
                        {"name": "GitHub Antigravity", "type": "url", "url": "https://github.com/damarkuncoro/phone-backup"},
                        {"name": "Tauri Studio", "type": "url", "url": "https://tauri.app"}
                    ],
                    "name": "Bookmarks Bar",
                    "type": "folder"
                }
            }
        }"#;
        bms = ChromiumBookmarksParser::parse(sample, bookmarks::BrowserType::Chrome);
    }
    let stats = BookmarkAnalytics::compute_stats(&bms);
    Ok(BookmarkVaultDto { bookmarks: bms, stats })
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_notes_vault(
    _state: State<'_, AppState>,
    _device_id: Option<String>,
) -> Result<NotesVaultDto, String> {
    let mut notes_list = Vec::new();
    if let Ok(n1) = NoteItemBuilder::new()
        .id("1")
        .title("Sprint 4 Roadmap")
        .content("Complete GUI specialist views and end-to-end data export verification.")
        .add_tag("Work")
        .add_tag("Urgent")
        .build()
    {
        notes_list.push(n1);
    }
    if let Ok(n2) = NoteItemBuilder::new()
        .id("2")
        .title("Travel Packing Checklist")
        .add_tag("Personal")
        .add_checklist_item(ChecklistItem::new("Passport & Visa", true))
        .add_checklist_item(ChecklistItem::new("MacBook Pro & Charger", true))
        .add_checklist_item(ChecklistItem::new("USB-C Fast Cable", false))
        .build()
    {
        notes_list.push(n2);
    }
    let stats = NotesAnalytics::compute_stats(&notes_list);
    Ok(NotesVaultDto { notes: notes_list, stats })
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_calendar_vault(
    _state: State<'_, AppState>,
    _device_id: Option<String>,
) -> Result<CalendarVaultDto, String> {
    let ics = r#"BEGIN:VCALENDAR
VERSION:2.0
BEGIN:VEVENT
UID:evt-001@phonebackup.org
SUMMARY:Tech Architecture Review
DESCRIPTION:Review specialist data engines & Clean Architecture
DTSTART:20260905T100000Z
DTEND:20260905T113000Z
LOCATION:Virtual Room Alpha
STATUS:CONFIRMED
END:VEVENT
BEGIN:VEVENT
UID:evt-002@phonebackup.org
SUMMARY:Client Demo
DESCRIPTION:Demonstrate modern GUI & Wi-Fi QR integration
DTSTART:20260905T110000Z
DTEND:20260905T120000Z
LOCATION:Executive Boardroom
STATUS:CONFIRMED
END:VEVENT
END:VCALENDAR"#;
    let events = IcsParser::parse(ics);
    let conflicts = CalendarAnalytics::find_conflicts(&events)
        .into_iter()
        .map(|(a, b)| format!("Conflict between '{}' and '{}'", a.summary, b.summary))
        .collect();
    let stats = CalendarAnalytics::compute_stats(&events);
    Ok(CalendarVaultDto { events, stats, conflicts })
}
