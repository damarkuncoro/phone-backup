use domain::{DeviceId, FileEntry, FileId, ScanCategory, ScanFilter, ScanWarning};
use phone_backup_scanner::{
    FileClassifier, FileMerger, IncrementalScanner, NoiseFilter, ScanPipeline,
};
use std::collections::HashMap;

fn create_test_file(path: &str, size_bytes: u64, mime_type: &str) -> FileEntry {
    FileEntry {
        id: FileId(path.to_string()),
        device_id: DeviceId::new("phone_dev_1".to_string()),
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
fn test_noise_filter_patterns() {
    let filter = ScanFilter::default();

    assert!(NoiseFilter::should_ignore(
        "/storage/emulated/0/.thumbnails/t1.jpg",
        500,
        &filter
    ));
    assert!(NoiseFilter::should_ignore(
        "/storage/emulated/0/.cache/data.bin",
        1000,
        &filter
    ));
    assert!(NoiseFilter::should_ignore(
        "/storage/emulated/0/.trash/video.mp4",
        50000,
        &filter
    ));
    assert!(NoiseFilter::should_ignore(
        "/storage/emulated/0/Download/file.tmp",
        100,
        &filter
    ));
    assert!(!NoiseFilter::should_ignore(
        "/storage/emulated/0/DCIM/Camera/photo.jpg",
        2000000,
        &filter
    ));
}

#[test]
fn test_file_classifier_and_summarizer() {
    let files = vec![
        create_test_file("/storage/emulated/0/DCIM/Camera/pic.jpg", 3000, "image/jpeg"),
        create_test_file("/storage/emulated/0/Movies/vid.mp4", 10000, "video/mp4"),
        create_test_file(
            "/storage/emulated/0/Android/media/com.whatsapp/Media/IMG.jpg",
            4000,
            "image/jpeg",
        ),
        create_test_file("/storage/emulated/0/Download/app.apk", 25000, ""),
        create_test_file("/storage/emulated/0/Documents/notes.pdf", 5000, ""),
    ];

    assert_eq!(FileClassifier::classify(&files[0]), ScanCategory::Photos);
    assert_eq!(FileClassifier::classify(&files[1]), ScanCategory::Videos);
    assert_eq!(FileClassifier::classify(&files[2]), ScanCategory::WhatsApp);
    assert_eq!(FileClassifier::classify(&files[3]), ScanCategory::Apks);
    assert_eq!(FileClassifier::classify(&files[4]), ScanCategory::Documents);

    let summary = FileClassifier::summarize(&files);
    assert_eq!(summary.get(&ScanCategory::Photos).unwrap().file_count, 1);
    assert_eq!(summary.get(&ScanCategory::Videos).unwrap().file_count, 1);
    assert_eq!(summary.get(&ScanCategory::WhatsApp).unwrap().file_count, 1);
    assert_eq!(summary.get(&ScanCategory::Apks).unwrap().file_count, 1);
    assert_eq!(summary.get(&ScanCategory::Documents).unwrap().file_count, 1);
}

#[test]
fn test_file_merger_and_deduplication() {
    let mut media = create_test_file("/storage/emulated/0/DCIM/1.jpg", 1000, "image/jpeg");
    media.mime_type = "image/jpeg".to_string();

    let mut fs = create_test_file("/storage/emulated/0/DCIM/1.jpg", 1500, "");
    fs.permissions = "-rwxr-xr-x".to_string();

    let merged = FileMerger::merge_entries(media, fs);
    assert_eq!(merged.size_bytes, 1500);
    assert_eq!(merged.mime_type, "image/jpeg");
    assert_eq!(merged.permissions, "-rwxr-xr-x");
}

#[test]
fn test_incremental_scanner_diffing() {
    let file1 = create_test_file("/path/1.jpg", 1000, "image/jpeg");
    let file2 = create_test_file("/path/2.jpg", 2000, "image/jpeg");
    let file3 = create_test_file("/path/3.jpg", 3000, "image/jpeg");

    let mut prev_index = HashMap::new();
    prev_index.insert(file1.path.clone(), file1.clone());

    let mut file2_old = file2.clone();
    file2_old.size_bytes = 1800;
    prev_index.insert(file2.path.clone(), file2_old);

    let current = vec![file1.clone(), file2.clone(), file3.clone()];
    let diff = IncrementalScanner::diff(&current, &prev_index);

    assert_eq!(diff.added.len(), 1); // file3 is new
    assert_eq!(diff.modified.len(), 1); // file2 is modified
    assert_eq!(diff.removed.len(), 0);

    let (changed, unchanged) = IncrementalScanner::partition_changed(current, &prev_index);
    assert_eq!(changed.len(), 2);
    assert_eq!(unchanged.len(), 1);
}

#[test]
fn test_scan_pipeline_orchestration() {
    let mut pipeline = ScanPipeline::new(ScanFilter::default());
    pipeline.add_directory_count(3);
    pipeline.add_warning(ScanWarning {
        source: domain::ScanSource::FileSystem,
        path: "/root".to_string(),
        message: "Protected folder".to_string(),
    });

    let primary = vec![create_test_file("/DCIM/1.jpg", 1000, "image/jpeg")];
    let secondary = vec![
        create_test_file("/DCIM/1.jpg", 1000, ""),
        create_test_file("/.cache/junk.tmp", 500, ""),
    ];

    let result = pipeline.process_multi_source(primary, secondary);
    assert_eq!(result.files.len(), 1);
    assert_eq!(result.warnings.len(), 1);
    assert!(result.metrics.is_some());
    assert_eq!(result.metrics.unwrap().directories_scanned, 3);
}

#[test]
fn test_builder_and_factory_pattern() {
    use phone_backup_scanner::{ScanFilterBuilder, ScanPipelineBuilder, ScanPipelineFactory};

    let filter = ScanFilterBuilder::new()
        .exclude_noise(true)
        .min_size_bytes(100)
        .max_size_bytes(5000)
        .add_exclude_glob("*.log")
        .build();

    assert_eq!(filter.min_size_bytes, Some(100));
    assert_eq!(filter.max_size_bytes, Some(5000));
    assert_eq!(filter.custom_exclude_globs, vec!["*.log"]);

    let mut custom_pipeline = ScanPipelineBuilder::new()
        .with_filter(filter)
        .with_directory_count(4)
        .build();

    let files = vec![create_test_file("/DCIM/photo.jpg", 1000, "image/jpeg")];
    let res = custom_pipeline.process_single_source(files);
    assert_eq!(res.files.len(), 1);

    let mut adb_pipeline = ScanPipelineFactory::for_android_adb(2, vec![]);
    let adb_res = adb_pipeline.process_single_source(vec![
        create_test_file("/DCIM/1.jpg", 1000, "image/jpeg"),
        create_test_file("/.cache/tmp.dat", 100, ""),
    ]);
    assert_eq!(adb_res.files.len(), 1);

    let mut ios_pipeline = ScanPipelineFactory::for_ios_afc();
    let ios_res = ios_pipeline.process_single_source(vec![
        create_test_file("/DCIM/100APPLE/IMG_0001.JPG", 2500, "image/jpeg"),
    ]);
    assert_eq!(ios_res.files.len(), 1);
}

