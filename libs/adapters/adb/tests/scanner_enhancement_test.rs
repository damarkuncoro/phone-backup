use domain::{DeviceId, FileEntry, FileId, ScanCategory, ScanFilter};
use phone_backup_adapter_adb::scanner::{FileClassifier, NoiseFilter, ScanMetricsTracker};

fn make_file(path: &str, size_bytes: u64, mime_type: &str) -> FileEntry {
    FileEntry {
        id: FileId(path.to_string()),
        device_id: DeviceId::new("test_device".to_string()),
        path: path.to_string(),
        name: path.rsplit('/').next().unwrap_or(path).to_string(),
        size_bytes,
        modified_at: chrono::Utc::now(),
        mime_type: mime_type.to_string(),
        permissions: "-rw-r--r--".to_string(),
        hash_sha256: None,
        thumbnail_hash: None,
        media_info: None,
    }
}

#[test]
fn test_noise_filter_system_and_cache() {
    let filter = ScanFilter::default();

    assert!(NoiseFilter::should_ignore(
        "/storage/emulated/0/DCIM/.thumbnails/thumb_1.jpg",
        1000,
        &filter
    ));
    assert!(NoiseFilter::should_ignore(
        "/storage/emulated/0/Android/data/com.app/cache/cache.tmp",
        5000,
        &filter
    ));
    assert!(NoiseFilter::should_ignore(
        "/storage/emulated/0/.trash/deleted.mp4",
        100000,
        &filter
    ));
    assert!(NoiseFilter::should_ignore(
        "/storage/emulated/0/Download/temp.crdownload",
        50000,
        &filter
    ));

    // Valid file should NOT be ignored
    assert!(!NoiseFilter::should_ignore(
        "/storage/emulated/0/DCIM/Camera/IMG_2026.jpg",
        2_500_000,
        &filter
    ));
}

#[test]
fn test_noise_filter_size_bounds_and_custom_globs() {
    let mut filter = ScanFilter::default();
    filter.min_size_bytes = Some(1024);
    filter.max_size_bytes = Some(10_000_000);
    filter.custom_exclude_globs = vec![".bak".to_string()];

    // Too small (< 1024)
    assert!(NoiseFilter::should_ignore(
        "/storage/emulated/0/Documents/tiny.txt",
        500,
        &filter
    ));

    // Too big (> 10MB)
    assert!(NoiseFilter::should_ignore(
        "/storage/emulated/0/Movies/huge.mkv",
        20_000_000,
        &filter
    ));

    // Custom glob match
    assert!(NoiseFilter::should_ignore(
        "/storage/emulated/0/Documents/backup.bak",
        5000,
        &filter
    ));

    // In bounds
    assert!(!NoiseFilter::should_ignore(
        "/storage/emulated/0/Documents/doc.pdf",
        5000,
        &filter
    ));
}

#[test]
fn test_file_classifier_categories() {
    let wa_img = make_file(
        "/storage/emulated/0/Android/media/com.whatsapp/WhatsApp/Media/WhatsApp Images/IMG.jpg",
        200000,
        "image/jpeg",
    );
    assert_eq!(FileClassifier::classify(&wa_img), ScanCategory::WhatsApp);

    let photo = make_file(
        "/storage/emulated/0/DCIM/Camera/photo.heic",
        3000000,
        "image/heic",
    );
    assert_eq!(FileClassifier::classify(&photo), ScanCategory::Photos);

    let video = make_file("/storage/emulated/0/Movies/sample.mp4", 15000000, "video/mp4");
    assert_eq!(FileClassifier::classify(&video), ScanCategory::Videos);

    let audio = make_file("/storage/emulated/0/Music/song.flac", 8000000, "audio/flac");
    assert_eq!(FileClassifier::classify(&audio), ScanCategory::Audio);

    let doc = make_file(
        "/storage/emulated/0/Documents/report.pdf",
        500000,
        "application/pdf",
    );
    assert_eq!(FileClassifier::classify(&doc), ScanCategory::Documents);

    let apk = make_file(
        "/storage/emulated/0/Download/base.apk",
        25000000,
        "application/vnd.android.package-archive",
    );
    assert_eq!(FileClassifier::classify(&apk), ScanCategory::Apks);

    let download = make_file(
        "/storage/emulated/0/Download/archive.zip",
        1000000,
        "application/zip",
    );
    assert_eq!(FileClassifier::classify(&download), ScanCategory::Downloads);
}

#[test]
fn test_classifier_summary_and_metrics() {
    let files = vec![
        make_file("/storage/emulated/0/DCIM/Camera/1.jpg", 1000, "image/jpeg"),
        make_file("/storage/emulated/0/DCIM/Camera/2.jpg", 2000, "image/jpeg"),
        make_file("/storage/emulated/0/Movies/clip.mp4", 5000, "video/mp4"),
    ];

    let summary = FileClassifier::summarize(&files);
    assert_eq!(summary.get(&ScanCategory::Photos).unwrap().file_count, 2);
    assert_eq!(summary.get(&ScanCategory::Photos).unwrap().total_bytes, 3000);
    assert_eq!(summary.get(&ScanCategory::Videos).unwrap().file_count, 1);
    assert_eq!(summary.get(&ScanCategory::Videos).unwrap().total_bytes, 5000);

    let mut tracker = ScanMetricsTracker::start();
    tracker.add_directories(2);
    tracker.set_files_scanned(files.len());
    let metrics = tracker.finish();

    assert_eq!(metrics.files_scanned, 3);
    assert_eq!(metrics.directories_scanned, 2);
}
