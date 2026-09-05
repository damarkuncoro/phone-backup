# phone-backup-bookmarks 🔖

Specialist crate for mobile web browser bookmarks, reading lists, and Netscape Bookmark HTML standard export.

## 🏗 Architecture & Modules

- **`domain/`**: Bookmark entities (`BookmarkItem`, `BookmarkFolder`, URL, title, favicon, add_date).
- **`parsers/`**: Chrome Bookmarks JSON parser, Firefox SQLite / JSONLZ4 parser, and Netscape HTML bookmark format parser.
- **`exporters/`**: Industry-standard Netscape Bookmark HTML file generator and JSON catalog builder.

## 🚀 Key Features

- **Universal Browser Import**: Output Netscape HTML files can be directly imported into desktop Google Chrome, Mozilla Firefox, Safari, Brave, and Microsoft Edge.
- **Hierarchical Folder Tree**: Retains nested bookmark folders and subcategories.
- **Domain & Analytics Summary**: Computes top bookmarked domains and categorizes active URLs.
