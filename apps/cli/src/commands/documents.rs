use anyhow::Result;
use application::BackupService;
use clap::Args;
use documents::{DocumentAnalyzer, DocumentItem, DocumentType};
use domain::DeviceId;
use std::collections::BTreeMap;

#[derive(Args, Debug)]
pub struct DocumentsArgs {
    /// Device id, e.g. 10DDAJ0G7D0002L or A1B2C3D4
    pub id: String,

    /// Filter by document format (pdf, spreadsheet, word, presentation, ebook, text)
    #[arg(short = 't', long = "type")]
    pub doc_type: Option<String>,

    /// Filter by minimum file size in bytes
    #[arg(long)]
    pub min_size: Option<u64>,

    /// Maximum number of documents to display
    #[arg(short, long, default_value_t = 20)]
    pub limit: usize,

    /// Display extracted metadata preview (author, title, pages, snippet)
    #[arg(short, long)]
    pub preview: bool,
}

fn format_bytes(bytes: u64) -> String {
    match number_prefix::NumberPrefix::decimal(bytes as f64) {
        number_prefix::NumberPrefix::Standalone(b) => format!("{:.0} B", b),
        number_prefix::NumberPrefix::Prefixed(p, n) => format!("{:.2} {}B", n, p),
    }
}

pub fn handle_documents<D, S, R, T, A, DP, P>(
    args: DocumentsArgs,
    service: &BackupService<D, S, R, T, A, DP, P>,
) -> Result<()>
where
    D: ports::DevicePort,
    S: ports::ScannerPort,
    R: ports::RepositoryPort,
    T: ports::StoragePort,
    A: ports::AppProviderPort,
    DP: ports::DataProviderPort,
    P: ports::ProgressPort,
{
    let device_id = DeviceId::new(&args.id);
    println!("📚 Inspecting documents on device {}...", args.id);

    let all_files = service.scan_device(&device_id)?;
    let mut doc_items: Vec<DocumentItem> = all_files
        .into_iter()
        .map(|f| DocumentAnalyzer::analyze_entry(&f, None))
        .filter(|d| d.doc_type != DocumentType::Other)
        .collect();

    let mut type_counts = BTreeMap::new();
    let mut type_bytes = BTreeMap::new();
    for doc in &doc_items {
        *type_counts.entry(doc.doc_type).or_insert(0) += 1;
        *type_bytes.entry(doc.doc_type).or_insert(0) += doc.size_bytes;
    }

    println!("\n📊 Document Library Summary:");
    println!("{:<20} {:>10} {:>15}", "TYPE", "FILES", "VOLUME");
    println!("{}", "-".repeat(50));
    for (t, count) in &type_counts {
        let bytes = type_bytes.get(t).copied().unwrap_or(0);
        println!("{:<20} {:>10} {:>15}", t.to_string(), count, format_bytes(bytes));
    }

    let parsed_type = args.doc_type.as_deref().map(|t| match t.to_lowercase().as_str() {
        "pdf" => DocumentType::Pdf,
        "spreadsheet" | "sheet" | "excel" | "csv" => DocumentType::Spreadsheet,
        "word" | "docx" | "doc" => DocumentType::WordProcessing,
        "presentation" | "ppt" | "pptx" => DocumentType::Presentation,
        "ebook" | "epub" => DocumentType::EBook,
        _ => DocumentType::TextOrCode,
    });

    doc_items = DocumentAnalyzer::filter_documents(doc_items, parsed_type, args.min_size);
    doc_items.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));

    println!("\n📄 Documents Discovered ({})", doc_items.len());
    println!("{:<45} {:<18} {:>12}  MODIFIED", "PATH", "TYPE", "SIZE");
    println!("{}", "-".repeat(95));
    for doc in doc_items.iter_mut().take(args.limit) {
        if args.preview {
            if let Ok(bytes) = service.read_remote_header(&device_id, &doc.path, 65536) {
                *doc = documents::DocumentAnalyzer::enrich_document(doc.clone(), &bytes);
            }
        }

        println!(
            "{:<45} {:<18} {:>12}  {}",
            doc.name,
            doc.doc_type.to_string(),
            format_bytes(doc.size_bytes),
            doc.modified_at.format("%Y-%m-%d %H:%M:%S")
        );

        if args.preview {
            if let Some(ref title) = doc.metadata.title {
                println!("   └── Title:  {}", title);
            }
            if let Some(ref author) = doc.metadata.author {
                println!("   └── Author: {}", author);
            }
            if let Some(pages) = doc.metadata.page_count {
                println!("   └── Pages:  {}", pages);
            }
            if let Some(ref snippet) = doc.metadata.text_snippet {
                println!("   └── Preview: \"{}\"", snippet);
            }
        }
    }

    if doc_items.len() > args.limit {
        println!("... and {} more documents (use --limit to show more)", doc_items.len() - args.limit);
    }

    Ok(())
}
