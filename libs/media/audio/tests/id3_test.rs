use phone_backup_audio::Id3Reader;

#[test]
fn test_parse_id3v1_metadata() {
    let mut mp3_bytes = vec![0u8; 1000];
    let mut id3v1 = [0u8; 128];
    id3v1[0..3].copy_from_slice(b"TAG");

    // Title: "Imagine"
    id3v1[3..10].copy_from_slice(b"Imagine");
    // Artist: "John Lennon"
    id3v1[33..44].copy_from_slice(b"John Lennon");
    // Album: "Imagine Album"
    id3v1[63..76].copy_from_slice(b"Imagine Album");
    // Year: "1971"
    id3v1[93..97].copy_from_slice(b"1971");

    mp3_bytes[1000 - 128..].copy_from_slice(&id3v1);

    let meta = Id3Reader::read_tags(&mp3_bytes);

    assert_eq!(meta.title.as_deref(), Some("Imagine"));
    assert_eq!(meta.artist.as_deref(), Some("John Lennon"));
    assert_eq!(meta.album.as_deref(), Some("Imagine Album"));
    assert_eq!(meta.year, Some(1971));
}
