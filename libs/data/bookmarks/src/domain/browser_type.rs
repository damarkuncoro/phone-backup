use serde::{Deserialize, Serialize};
use std::fmt;

/// Supported Mobile / Desktop Web Browser sources
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserType {
    Chrome,
    Brave,
    Edge,
    Firefox,
    SamsungBrowser,
    Opera,
    Generic,
}

impl BrowserType {
    /// Infer browser type from path or package identifier
    pub fn from_package_or_path(val: &str) -> Self {
        let v = val.to_lowercase();
        if v.contains("com.android.chrome") || v.contains("google-chrome") || v.contains("chrome") {
            Self::Chrome
        } else if v.contains("com.brave.browser") || v.contains("brave") {
            Self::Brave
        } else if v.contains("com.microsoft.emmx") || v.contains("edge") {
            Self::Edge
        } else if v.contains("org.mozilla.firefox") || v.contains("firefox") {
            Self::Firefox
        } else if v.contains("com.sec.android.app.sbrowser") || v.contains("samsung") {
            Self::SamsungBrowser
        } else if v.contains("com.opera.browser") || v.contains("opera") {
            Self::Opera
        } else {
            Self::Generic
        }
    }
}

impl fmt::Display for BrowserType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Chrome => write!(f, "Google Chrome"),
            Self::Brave => write!(f, "Brave Browser"),
            Self::Edge => write!(f, "Microsoft Edge"),
            Self::Firefox => write!(f, "Mozilla Firefox"),
            Self::SamsungBrowser => write!(f, "Samsung Internet"),
            Self::Opera => write!(f, "Opera"),
            Self::Generic => write!(f, "Browser"),
        }
    }
}
