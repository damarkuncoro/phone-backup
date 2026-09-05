use phone_backup_documents::{
    DocumentAnalyzer, DocumentFactory, DocumentItemBuilder, DocumentMetadata, DocumentType,
    PdfExtractor, TextExtractor,
};

#[test]
fn test_document_type_inference() {
    assert_eq!(DocumentType::from_path("/docs/sample.pdf"), DocumentType::Pdf);
    assert_eq!(DocumentType::from_path("/docs/sheet.xlsx"), DocumentType::Spreadsheet);
    assert_eq!(DocumentType::from_path("/docs/data.csv"), DocumentType::Spreadsheet);
    assert_eq!(DocumentType::from_path("/docs/contract.docx"), DocumentType::WordProcessing);
    assert_eq!(DocumentType::from_path("/docs/slides.pptx"), DocumentType::Presentation);
    assert_eq!(DocumentType::from_path("/books/novel.epub"), DocumentType::EBook);
    assert_eq!(DocumentType::from_path("/notes/readme.md"), DocumentType::TextOrCode);
    assert_eq!(DocumentType::from_path("/bin/app.bin"), DocumentType::Other);
}

#[test]
fn test_pdf_metadata_extractor() {
    let mock_pdf = b"%PDF-1.7\n/Type /Pages /Count 42\n/Title (Annual Financial Report 2026)\n/Author (Acme Corp)\n%%EOF";
    let meta = PdfExtractor::extract(mock_pdf);

    assert_eq!(meta.page_count, Some(42));
    assert_eq!(meta.title.as_deref(), Some("Annual Financial Report 2026"));
    assert_eq!(meta.author.as_deref(), Some("Acme Corp"));
    assert!(!meta.is_password_protected);

    let encrypted_pdf = b"%PDF-1.7\n/Encrypt 12 0 R\n/Type /Pages /Count 5\n%%EOF";
    let enc_meta = PdfExtractor::extract(encrypted_pdf);
    assert!(enc_meta.is_password_protected);
    assert_eq!(enc_meta.page_count, Some(5));
}

#[test]
fn test_text_and_snippet_extractor() {
    let text = b"The quick brown fox jumps over the lazy dog. Comprehensive backup is vital.";
    let meta = TextExtractor::extract(text);

    assert_eq!(meta.word_count, Some(13));
    assert!(meta.text_snippet.unwrap().contains("The quick brown fox"));
}

#[test]
fn test_builder_and_factory_pattern() {
    let doc_from_builder = DocumentItemBuilder::new("/storage/emulated/0/Documents/notes.txt")
        .with_size(1024)
        .with_metadata(DocumentMetadata::new().with_word_count(150))
        .build();

    assert_eq!(doc_from_builder.doc_type, DocumentType::TextOrCode);
    assert_eq!(doc_from_builder.size_bytes, 1024);
    assert_eq!(doc_from_builder.metadata.word_count, Some(150));

    let pdf = DocumentFactory::create_pdf("/docs/whitepaper.pdf", "Zero Trust Architecture", 24, 500_000);
    assert_eq!(pdf.doc_type, DocumentType::Pdf);
    assert_eq!(pdf.metadata.title.as_deref(), Some("Zero Trust Architecture"));
    assert_eq!(pdf.metadata.page_count, Some(24));

    let sheet = DocumentFactory::create_spreadsheet("/docs/budget.xlsx", "Q3 Budget", 4, 120_000);
    assert_eq!(sheet.doc_type, DocumentType::Spreadsheet);
    assert_eq!(sheet.metadata.sheet_count, Some(4));
}

#[test]
fn test_document_analyzer_filtering() {
    let docs = vec![
        DocumentFactory::create_pdf("/docs/1.pdf", "Doc 1", 10, 1000),
        DocumentFactory::create_spreadsheet("/docs/2.xlsx", "Sheet 1", 2, 2000),
        DocumentFactory::create_text("/docs/3.txt", "Hello text", 2, 500),
    ];

    let pdfs_only = DocumentAnalyzer::filter_documents(docs.clone(), Some(DocumentType::Pdf), None);
    assert_eq!(pdfs_only.len(), 1);
    assert_eq!(pdfs_only[0].doc_type, DocumentType::Pdf);

    let large_docs = DocumentAnalyzer::filter_documents(docs, None, Some(1500));
    assert_eq!(large_docs.len(), 1);
    assert_eq!(large_docs[0].doc_type, DocumentType::Spreadsheet);
}
