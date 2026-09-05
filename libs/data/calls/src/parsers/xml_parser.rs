use crate::domain::{CallLogItem, CallType};
use chrono::{DateTime, Utc};
use quick_xml::events::Event;
use quick_xml::Reader;

/// Parser for standard Android Call Log XML backup files.
pub struct XmlCallParser;

impl XmlCallParser {
    /// Parses Call Log XML data into a vector of `CallLogItem`.
    pub fn parse(xml_content: &str) -> Vec<CallLogItem> {
        let mut reader = Reader::from_str(xml_content);
        reader.trim_text(true);

        let mut items = Vec::new();
        let mut buf = Vec::new();
        let mut auto_id = 1usize;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) | Ok(Event::Start(ref e)) if e.name().as_ref() == b"call" => {
                    let mut number = String::new();
                    let mut duration_secs = 0u64;
                    let mut timestamp = Utc::now();
                    let mut call_type = CallType::Unknown;
                    let mut contact_name = None;
                    let mut sim_slot = None;

                    for attr in e.attributes().flatten() {
                        let key = attr.key.as_ref();
                        let val = String::from_utf8_lossy(&attr.value);

                        match key {
                            b"number" => number = val.to_string(),
                            b"duration" => duration_secs = val.parse().unwrap_or(0),
                            b"date" => {
                                if let Ok(ms) = val.parse::<i64>() {
                                    if let Some(dt) = DateTime::from_timestamp_millis(ms) {
                                        timestamp = dt;
                                    }
                                }
                            }
                            b"type" => {
                                if let Ok(code) = val.parse::<u32>() {
                                    call_type = CallType::from_android_type(code);
                                }
                            }
                            b"name" | b"contact_name" => {
                                if !val.trim().is_empty() && val != "(Unknown)" {
                                    contact_name = Some(val.to_string());
                                }
                            }
                            b"subscription_id" | b"sim_slot" => {
                                sim_slot = val.parse::<u8>().ok();
                            }
                            _ => {}
                        }
                    }

                    if !number.is_empty() {
                        let mut item = CallLogItem::new(
                            format!("call_{}", auto_id),
                            number,
                            call_type,
                            timestamp,
                            duration_secs,
                        );
                        if let Some(name) = contact_name {
                            item = item.with_name(name);
                        }
                        if let Some(slot) = sim_slot {
                            item = item.with_sim_slot(slot);
                        }
                        items.push(item);
                        auto_id += 1;
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => break,
                _ => {}
            }
            buf.clear();
        }

        items
    }
}
