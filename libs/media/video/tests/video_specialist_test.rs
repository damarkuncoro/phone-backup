use chrono::Utc;
use phone_backup_video::{
    VideoAnalyzer, VideoContainer, VideoFactory, VideoItemBuilder, VideoMetadata, VideoQuality,
};

#[test]
fn test_video_container_detection() {
    assert_eq!(VideoContainer::from_extension("mp4"), VideoContainer::Mp4);
    assert_eq!(VideoContainer::from_extension("MKV"), VideoContainer::Mkv);
    assert_eq!(VideoContainer::from_extension("webm"), VideoContainer::WebM);
    assert_eq!(VideoContainer::from_extension("avi"), VideoContainer::Avi);
    assert_eq!(VideoContainer::from_extension("mov"), VideoContainer::Mov);
    assert_eq!(VideoContainer::from_extension("3gp"), VideoContainer::ThreeGP);
}

#[test]
fn test_video_quality_tiering() {
    assert_eq!(VideoQuality::from_dimensions(3840, 2160), VideoQuality::Uhd4K);
    assert_eq!(VideoQuality::from_dimensions(2160, 3840), VideoQuality::Uhd4K);
    assert_eq!(VideoQuality::from_dimensions(2560, 1440), VideoQuality::Qhd2K);
    assert_eq!(VideoQuality::from_dimensions(1920, 1080), VideoQuality::Fhd1080p);
    assert_eq!(VideoQuality::from_dimensions(1080, 1920), VideoQuality::Fhd1080p); // Vertical video
    assert_eq!(VideoQuality::from_dimensions(1280, 720), VideoQuality::Hd720p);
    assert_eq!(VideoQuality::from_dimensions(854, 480), VideoQuality::Sd480p);
    assert_eq!(VideoQuality::from_dimensions(320, 240), VideoQuality::LowRes);
    assert_eq!(VideoQuality::from_dimensions(0, 0), VideoQuality::Unknown);
}

#[test]
fn test_video_item_builder() {
    let now = Utc::now();
    let meta = VideoMetadata::new(1920, 1080, 125.4);

    let item = VideoItemBuilder::new()
        .path("/DCIM/Camera/VID_20260904_120000.mp4")
        .size_bytes(45_000_000)
        .modified_at(now)
        .metadata(meta)
        .build()
        .expect("Builder should succeed");

    assert_eq!(item.container, VideoContainer::Mp4);
    assert_eq!(item.size_bytes, 45_000_000);
    assert_eq!(item.quality(), VideoQuality::Fhd1080p);
    assert_eq!(item.resolution(), "1920x1080");
    assert_eq!(item.duration_display(), "02:05");
}

#[test]
fn test_video_factory() {
    let item_mp4 = VideoFactory::create_mp4("/videos/clip.mp4", 1024);
    assert_eq!(item_mp4.container, VideoContainer::Mp4);

    let item_mkv = VideoFactory::create_mkv("/videos/movie.mkv", 2048);
    assert_eq!(item_mkv.container, VideoContainer::Mkv);

    let item_auto = VideoFactory::create_from_path("/videos/anim.webm", 512);
    assert_eq!(item_auto.container, VideoContainer::WebM);
}

#[test]
fn test_mp4_header_mock_extraction() {
    // Construct synthetic MP4 with ftyp and moov atom containing tkhd and mvhd
    let mut data = Vec::new();
    // ftyp atom
    data.extend_from_slice(&[0, 0, 0, 16]);
    data.extend_from_slice(b"ftypisom");
    data.extend_from_slice(&[0, 0, 2, 0]);

    // moov atom with mvhd and tkhd
    let mut moov_payload = Vec::new();

    // mvhd atom: size 108
    moov_payload.extend_from_slice(&[0, 0, 0, 108]);
    moov_payload.extend_from_slice(b"mvhd");
    moov_payload.push(0); // version 0
    moov_payload.extend_from_slice(&[0, 0, 0]); // flags
    moov_payload.extend_from_slice(&[0, 0, 0, 0]); // creation time
    moov_payload.extend_from_slice(&[0, 0, 0, 0]); // mod time
    moov_payload.extend_from_slice(&[0, 0, 3, 232]); // timescale = 1000
    moov_payload.extend_from_slice(&[0, 0, 234, 96]); // duration = 60000 (60s)
    moov_payload.resize(108, 0);

    // tkhd atom: size 92
    let mut tkhd = Vec::new();
    tkhd.extend_from_slice(&[0, 0, 0, 92]);
    tkhd.extend_from_slice(b"tkhd");
    tkhd.resize(84, 0);
    // width: 1920 (0x0780), 0
    tkhd.extend_from_slice(&[0x07, 0x80, 0, 0]);
    // height: 1080 (0x0438), 0
    tkhd.extend_from_slice(&[0x04, 0x38, 0, 0]);
    moov_payload.extend_from_slice(&tkhd);

    // avc1 codec marker
    moov_payload.extend_from_slice(b"avc1");

    let moov_size = (moov_payload.len() + 8) as u32;
    data.extend_from_slice(&moov_size.to_be_bytes());
    data.extend_from_slice(b"moov");
    data.extend_from_slice(&moov_payload);

    let meta = VideoAnalyzer::analyze_bytes(&VideoContainer::Mp4, &data)
        .expect("Should extract metadata from synthetic MP4");

    assert_eq!(meta.width, 1920);
    assert_eq!(meta.height, 1080);
    assert_eq!(meta.duration_secs, 60.0);
    assert_eq!(meta.quality_tier, VideoQuality::Fhd1080p);
    assert_eq!(meta.video_codec.as_deref(), Some("H.264 / AVC"));
}
