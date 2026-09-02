use crate::parsers::common::ParserUtils;
use chrono::{TimeZone, Utc};
use domain::{CallLog, Sms};

pub struct CommunicationParser;

impl CommunicationParser {
    pub fn parse_sms(output: &str) -> Vec<Sms> {
        let mut messages = Vec::new();
        for line in output.lines() {
            if let (Some(address), Some(body), Some(date_str)) = (
                ParserUtils::extract_value(line, "address"),
                ParserUtils::extract_value(line, "body"),
                ParserUtils::extract_value(line, "date"),
            ) {
                let timestamp = date_str.parse::<i64>().unwrap_or(0);
                let type_code = ParserUtils::extract_value(line, "type")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1);
                messages.push(Sms {
                    address,
                    body: body.replace("\\n", "\n"),
                    date: Utc
                        .timestamp_opt(timestamp / 1000, 0)
                        .single()
                        .unwrap_or_else(Utc::now),
                    type_code,
                });
            }
        }
        messages.sort_by_key(|b| std::cmp::Reverse(b.date));
        messages
    }

    pub fn parse_call_logs(output: &str) -> Vec<CallLog> {
        let mut logs = Vec::new();
        for line in output.lines() {
            if let (Some(number), Some(date_str), Some(duration_str)) = (
                ParserUtils::extract_value(line, "number"),
                ParserUtils::extract_value(line, "date"),
                ParserUtils::extract_value(line, "duration"),
            ) {
                let timestamp = date_str.parse::<i64>().unwrap_or(0);
                logs.push(CallLog {
                    number,
                    name: ParserUtils::extract_value(line, "name"),
                    date: Utc
                        .timestamp_opt(timestamp / 1000, 0)
                        .single()
                        .unwrap_or_else(Utc::now),
                    duration_seconds: duration_str.parse().unwrap_or(0),
                    type_code: ParserUtils::extract_value(line, "type")
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(1),
                    location: ParserUtils::extract_value(line, "geocoded_location"),
                });
            }
        }
        logs
    }
}
