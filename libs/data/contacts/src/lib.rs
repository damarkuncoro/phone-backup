pub mod book;
pub mod builder;
pub mod formatters;
pub mod intelligence;
pub mod model;
pub mod vcard;

pub use book::ContactBook;
pub use builder::{ContactBookBuilder, ContactBuilder};
pub use formatters::{ContactFormatterFactory, CsvFormatter, ExportFormat, JsonFormatter};
pub use intelligence::{ContactDiff, ContactDiffEngine, ContactMatcher, ContactMerger, MatchConfidence, PhoneNormalizer};
pub use model::{
    Contact, ContactPhoto, EmailAddress, EmailType, Organization, PhoneNumber, PhoneType,
    PostalAddress, SocialProfile, StructuredName,
};
pub use vcard::{PhotoHandler, VCardParser, VCardVersion, VCardWriter};
