use phone_backup_apps::{ApkSplitType, SplitApkBundle, SplitBundleAssembler};

#[test]
fn test_split_apk_bundle_assembly() {
    let apk_files = vec![
        ("com.whatsapp".to_string(), "base.apk".to_string(), 45_000_000),
        ("com.whatsapp".to_string(), "split_config.arm64_v8a.apk".to_string(), 12_000_000),
        ("com.whatsapp".to_string(), "split_config.xxhdpi.apk".to_string(), 5_000_000),
        ("com.instagram.android".to_string(), "base.apk".to_string(), 60_000_000),
    ];

    let bundles = SplitBundleAssembler::group_splits(&apk_files);

    assert_eq!(bundles.len(), 2);

    let wa_bundle = bundles.iter().find(|b| b.package_name == "com.whatsapp").unwrap();
    assert_eq!(wa_bundle.files.len(), 3);
    assert_eq!(wa_bundle.total_size(), 62_000_000);
    assert!(SplitBundleAssembler::is_installable(wa_bundle));

    let split_types: Vec<ApkSplitType> = wa_bundle.files.iter().map(|f| f.split_type).collect();
    assert!(split_types.contains(&ApkSplitType::Base));
    assert!(split_types.contains(&ApkSplitType::ConfigAbi));
    assert!(split_types.contains(&ApkSplitType::ConfigDensity));
}

#[test]
fn test_non_installable_bundle_without_base() {
    let mut bundle = SplitApkBundle::new("com.broken.app", 1);
    bundle.add_split("split_config.arm64_v8a.apk", 10_000_000);

    assert!(!SplitBundleAssembler::is_installable(&bundle));
}
