use phone_backup_apps::{AppPackageBuilder, AppType};

#[test]
fn test_app_package_builder_fluent_creation() {
    let pkg = AppPackageBuilder::new("com.google.android.youtube", "YouTube")
        .with_version(190500, "19.05.36")
        .with_app_type(AppType::User)
        .with_sizes(42_000_000, 150_000_000)
        .with_splits(4)
        .build();

    assert_eq!(pkg.package_name, "com.google.android.youtube");
    assert_eq!(pkg.app_name, "YouTube");
    assert_eq!(pkg.version_code, 190500);
    assert_eq!(pkg.version_name, "19.05.36");
    assert_eq!(pkg.app_type, AppType::User);
    assert_eq!(pkg.apk_size_bytes, 42_000_000);
    assert_eq!(pkg.data_size_bytes, 150_000_000);
    assert!(pkg.is_split);
    assert_eq!(pkg.splits_count, 4);
}
