pub mod diff;
pub mod matcher;
pub mod merger;
pub mod normalizer;

pub use diff::{ContactDiff, ContactDiffEngine};
pub use matcher::{ContactMatcher, MatchConfidence};
pub use merger::ContactMerger;
pub use normalizer::PhoneNormalizer;
