pub mod error;
pub mod browser;
pub mod dom;
pub mod tools;

#[cfg(feature = "mcp-server")]
pub mod mcp;

pub use error::{BrowserError, Result};
pub use browser::{BrowserSession, LaunchOptions, ConnectionOptions};
pub use dom::{DomTree, ElementNode, ElementSelector, SelectorMap, BoundingBox};
pub use tools::{Tool, ToolRegistry, ToolResult, ToolContext};

#[cfg(feature = "mcp-server")]
pub use mcp::BrowserServer;
