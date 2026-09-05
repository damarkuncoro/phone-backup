use phone_backup_wifi::{
    SecurityType, WifiAnalytics, WifiConfigStoreXmlParser, WifiCsvExporter, WifiFactory,
    WifiJsonExporter, WifiJsonParser, WifiNetworkBuilder, WifiQrGenerator, WpaSupplicantExporter,
    WpaSupplicantParser,
};

#[test]
fn test_builder_and_factory() {
    let net1 = WifiNetworkBuilder::new("Office_5G")
        .psk("SuperSecretPass123")
        .security(SecurityType::Wpa3Sae)
        .hidden(true)
        .metered(false)
        .build();

    assert_eq!(net1.ssid, "Office_5G");
    assert_eq!(net1.security_type, SecurityType::Wpa3Sae);
    assert!(net1.is_hidden);
    assert_eq!(net1.masked_password(), "********");

    let open_net = WifiFactory::create_open("Starbucks_Free_WiFi");
    assert_eq!(open_net.security_type, SecurityType::Open);
    assert!(!open_net.security_type.is_secure());
    assert_eq!(open_net.masked_password(), "[None]");
}

#[test]
fn test_qr_generator() {
    let net = WifiFactory::create_wpa2("Home_Fiber", "MyHomePassword");
    let qr = WifiQrGenerator::generate_payload(&net);
    assert_eq!(qr, "WIFI:T:WPA;S:Home_Fiber;P:MyHomePassword;H:false;;");

    let open_net = WifiFactory::create_open("Airport_Guest");
    let qr_open = WifiQrGenerator::generate_payload(&open_net);
    assert_eq!(qr_open, "WIFI:T:nopass;S:Airport_Guest;H:false;;");

    let card = WifiQrGenerator::render_terminal_card(&net);
    assert!(card.contains("Home_Fiber"));
    assert!(card.contains("WPA2-PSK"));
}

#[test]
fn test_wpa_supplicant_parser_and_exporter() {
    let conf = r#"
# Sample wpa_supplicant.conf
network={
    ssid="CoffeeShop_WiFi"
    psk="latte12345"
    key_mgmt=WPA-PSK
    scan_ssid=1
}

network={
    ssid="Open_Hotspot"
    key_mgmt=NONE
}
"#;

    let parsed = WpaSupplicantParser::parse(conf);
    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0].ssid, "CoffeeShop_WiFi");
    assert_eq!(parsed[0].pre_shared_key.as_deref(), Some("latte12345"));
    assert_eq!(parsed[0].security_type, SecurityType::Wpa2Psk);
    assert!(parsed[0].is_hidden);

    assert_eq!(parsed[1].ssid, "Open_Hotspot");
    assert_eq!(parsed[1].security_type, SecurityType::Open);

    let exported = WpaSupplicantExporter::export(&parsed);
    assert!(exported.contains("ssid=\"CoffeeShop_WiFi\""));
    assert!(exported.contains("key_mgmt=NONE"));
}

#[test]
fn test_xml_store_parser() {
    let xml = r#"<?xml version='1.0' encoding='utf-8' standalone='yes' ?>
<WifiConfigStoreData>
  <int name="Version" value="3" />
  <NetworkList>
    <Network>
      <WifiConfiguration>
        <string name="ConfigKey">&quot;Home_Network&quot;WPA_PSK</string>
        <string name="SSID">&quot;Home_Network&quot;</string>
        <string name="PreSharedKey">&quot;wpa2secret&quot;</string>
        <string name="KeyMgmt">WPA-PSK</string>
        <boolean name="HiddenSSID" value="false" />
        <int name="MeteredOverride" value="0" />
      </WifiConfiguration>
    </Network>
  </NetworkList>
</WifiConfigStoreData>"#;

    let networks = WifiConfigStoreXmlParser::parse(xml);
    assert_eq!(networks.len(), 1);
    assert_eq!(networks[0].ssid, "Home_Network");
    assert_eq!(networks[0].pre_shared_key.as_deref(), Some("wpa2secret"));
    assert_eq!(networks[0].security_type, SecurityType::Wpa2Psk);
}

#[test]
fn test_analytics_and_exporters() {
    let n1 = WifiFactory::create_wpa2("Net_A", "pass1");
    let n2 = WifiFactory::create_wpa3("Net_B", "pass2");
    let n3 = WifiFactory::create_open("Net_C");

    let list = vec![n1, n2, n3];
    let stats = WifiAnalytics::compute_stats(&list);
    assert_eq!(stats.total_networks, 3);
    assert_eq!(stats.secured_networks, 2);
    assert_eq!(stats.open_networks, 1);

    let json_str = WifiJsonExporter::export(&list).unwrap();
    let re_parsed = WifiJsonParser::parse(&json_str).unwrap();
    assert_eq!(re_parsed.len(), 3);

    let csv = WifiCsvExporter::export(&list, true);
    assert!(csv.contains("Net_A"));
    assert!(csv.contains("pass1"));
}
