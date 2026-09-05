use crate::builder::WifiNetworkBuilder;
use crate::domain::{SecurityType, WifiNetworkItem};
use quick_xml::events::Event;
use quick_xml::Reader;

/// Parser for Android 8-15 WifiConfigStore.xml
pub struct WifiConfigStoreXmlParser;

impl WifiConfigStoreXmlParser {
    pub fn parse(xml_content: &str) -> Vec<WifiNetworkItem> {
        let mut reader = Reader::from_str(xml_content);
        reader.trim_text(true);

        let mut networks = Vec::new();
        let mut in_wifi_config = false;

        let mut current_ssid = String::new();
        let mut current_psk = Option::<String>::None;
        let mut current_key_mgmt = String::new();
        let mut is_hidden = false;
        let mut is_metered = false;
        let mut current_attr_name = String::new();

        let mut buf = Vec::new();

        while let Ok(event) = reader.read_event_into(&mut buf) {
            match event {
                Event::Start(e) => {
                    let name = e.name();
                    if name.as_ref() == b"WifiConfiguration" || (!in_wifi_config && name.as_ref() == b"Network") {
                        in_wifi_config = true;
                        current_ssid.clear();
                        current_psk = None;
                        current_key_mgmt.clear();
                        is_hidden = false;
                        is_metered = false;
                    } else if in_wifi_config {
                        current_attr_name.clear();
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"name" {
                                if let Ok(val) = String::from_utf8(attr.value.to_vec()) {
                                    current_attr_name = val;
                                }
                            }
                        }
                    }
                }
                Event::Text(e) => {
                    if in_wifi_config && !current_attr_name.is_empty() {
                        if let Ok(text) = e.unescape() {
                            let val = text.into_owned();
                            match current_attr_name.as_str() {
                                "SSID" => {
                                    current_ssid = val.trim_matches('"').to_string();
                                }
                                "ConfigKey" if current_ssid.is_empty() => {
                                    if val.starts_with('"') {
                                        if let Some(end) = val[1..].find('"') {
                                            current_ssid = val[1..=end].to_string();
                                        } else {
                                            current_ssid = val.trim_matches('"').to_string();
                                        }
                                    } else {
                                        current_ssid = val.trim_matches('"').to_string();
                                    }
                                }
                                "PreSharedKey" | "PresharedKey" => {
                                    current_psk = Some(val.trim_matches('"').to_string());
                                }
                                "KeyMgmt" | "AllowedKeyMgmt" => {
                                    current_key_mgmt = val;
                                }
                                "HiddenSSID" => {
                                    is_hidden = val == "true";
                                }
                                "MeteredOverride" | "MeteredHint" => {
                                    is_metered = val == "1" || val == "true";
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Event::End(e) => {
                    let name = e.name();
                    if (name.as_ref() == b"WifiConfiguration" || name.as_ref() == b"Network") && in_wifi_config {
                        if !current_ssid.is_empty() {
                            let security = SecurityType::from_key_mgmt(&current_key_mgmt);
                            let mut builder = WifiNetworkBuilder::new(&current_ssid)
                                .security(security)
                                .hidden(is_hidden)
                                .metered(is_metered);

                            if let Some(psk) = current_psk.take() {
                                builder = builder.psk(psk);
                            }

                            networks.push(builder.build());
                            current_ssid.clear();
                        }
                        in_wifi_config = false;
                    }
                    current_attr_name.clear();
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }

        networks
    }
}
