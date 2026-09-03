use phone_backup_apps::AxmlParser;

fn create_synthetic_axml_chunk() -> Vec<u8> {
    let mut bytes = Vec::new();

    // 1. XML Header (8 bytes): type=0x0003, header_size=8, chunk_size=placeholder
    bytes.extend_from_slice(&0x0003u16.to_le_bytes());
    bytes.extend_from_slice(&8u16.to_le_bytes());
    bytes.extend_from_slice(&0u32.to_le_bytes()); // placeholder chunk_size

    // 2. StringPool chunk (type=0x0001)
    let s1 = "com.antigravity.phonebackup";
    let s2 = "android.permission.CAMERA";

    let mut sp_data = Vec::new();
    // offsets relative to strings_start
    let off0 = 0u32;
    let off1 = (2 + s1.len()) as u32;

    sp_data.extend_from_slice(&off0.to_le_bytes());
    sp_data.extend_from_slice(&off1.to_le_bytes());

    // Strings (UTF-8 format: char_len, byte_len, bytes...)
    sp_data.push(s1.len() as u8);
    sp_data.push(s1.len() as u8);
    sp_data.extend_from_slice(s1.as_bytes());

    sp_data.push(s2.len() as u8);
    sp_data.push(s2.len() as u8);
    sp_data.extend_from_slice(s2.as_bytes());

    let strings_start = 28 + 8; // 28 bytes header + 2 offsets (8 bytes)
    let chunk_size = 28 + sp_data.len() as u32;

    let mut sp_chunk = Vec::new();
    sp_chunk.extend_from_slice(&0x0001u16.to_le_bytes()); // type
    sp_chunk.extend_from_slice(&28u16.to_le_bytes());     // header_size
    sp_chunk.extend_from_slice(&chunk_size.to_le_bytes()); // chunk_size
    sp_chunk.extend_from_slice(&2u32.to_le_bytes());       // string_count = 2
    sp_chunk.extend_from_slice(&0u32.to_le_bytes());       // style_count = 0
    sp_chunk.extend_from_slice(&(1u32 << 8).to_le_bytes());// flags = UTF-8
    sp_chunk.extend_from_slice(&(strings_start as u32).to_le_bytes()); // strings_start
    sp_chunk.extend_from_slice(&0u32.to_le_bytes());       // styles_start
    sp_chunk.extend_from_slice(&sp_data);

    bytes.extend_from_slice(&sp_chunk);

    // Update total file size in header
    let total_size = bytes.len() as u32;
    bytes[4..8].copy_from_slice(&total_size.to_le_bytes());

    bytes
}

#[test]
fn test_parse_axml_string_pool() {
    let axml_data = create_synthetic_axml_chunk();
    let strings = AxmlParser::extract_string_pool(&axml_data).expect("AXML parsing failed");

    assert_eq!(strings.len(), 2);
    assert_eq!(strings[0], "com.antigravity.phonebackup");
    assert_eq!(strings[1], "android.permission.CAMERA");
}

#[test]
fn test_invalid_axml_magic_fails_gracefully() {
    let bad_bytes = vec![0x00, 0x00, 0x08, 0x00, 0x10, 0x00, 0x00, 0x00];
    let res = AxmlParser::extract_string_pool(&bad_bytes);
    assert!(res.is_err());
}
