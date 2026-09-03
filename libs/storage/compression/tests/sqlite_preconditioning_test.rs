use phone_backup_compression::preprocessing::SqliteZeroFillPreconditioner;

#[test]
fn test_non_sqlite_bypass() {
    let preconditioner = SqliteZeroFillPreconditioner::new();
    let sample = b"Just some normal text data not sqlite";
    let res = preconditioner.precondition(sample).expect("precondition failed");
    assert_eq!(res, sample);
}

#[test]
fn test_sqlite_freelist_zero_filling() {
    let preconditioner = SqliteZeroFillPreconditioner::new();
    let page_size = 4096;
    let mut db = vec![0u8; page_size * 3]; // 3 pages: Page 1 (header), Page 2 (freelist trunk), Page 3 (freelist leaf)

    // Set SQLite magic
    db[0..16].copy_from_slice(b"SQLite format 3\0");
    // Page size 4096 (0x1000)
    db[16..18].copy_from_slice(&4096u16.to_be_bytes());
    // First freelist trunk page is Page 2
    db[32..36].copy_from_slice(&2u32.to_be_bytes());
    // Total freelist pages = 1
    db[36..40].copy_from_slice(&1u32.to_be_bytes());

    // Setup Page 2 (Trunk Page): offset 4096
    let trunk_offset = 4096;
    // Next trunk = 0 (none)
    db[trunk_offset..trunk_offset + 4].copy_from_slice(&0u32.to_be_bytes());
    // Number of leaf pages on this trunk = 1
    db[trunk_offset + 4..trunk_offset + 8].copy_from_slice(&1u32.to_be_bytes());
    // Pointer to leaf page = Page 3
    db[trunk_offset + 8..trunk_offset + 12].copy_from_slice(&3u32.to_be_bytes());

    // Fill Page 3 (Leaf Page, offset 8192) with dirty leftover random garbage
    let leaf_offset = 8192;
    for (i, b) in db[leaf_offset..leaf_offset + page_size].iter_mut().enumerate() {
        *b = (i % 255) as u8;
    }

    let preconditioned = preconditioner.precondition(&db).expect("preconditioning should succeed");

    // Verify Page 3 is now completely zeroed out
    assert!(preconditioned[leaf_offset..leaf_offset + page_size].iter().all(|&b| b == 0));

    // Verify Page 1 header remains intact
    assert_eq!(&preconditioned[0..16], b"SQLite format 3\0");
    assert_eq!(&preconditioned[16..18], &4096u16.to_be_bytes());
}
