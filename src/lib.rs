pub mod error;
pub mod browser;
pub mod dom;

pub use error::{BrowserError, Result};
pub use browser::{BrowserSession, LaunchOptions, ConnectionOptions};
pub use dom::{DomTree, ElementNode, ElementSelector, SelectorMap, BoundingBox};
