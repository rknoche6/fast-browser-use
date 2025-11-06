//! DOM extraction and manipulation module
//!
//! This module provides functionality for extracting and working with the DOM structure
//! of web pages. It includes:
//! - ElementNode: Representation of DOM elements
//! - DomTree: Complete DOM tree with indexing for interactive elements
//! - SelectorMap: Mapping of indices to element selectors

pub mod element;
pub mod selector_map;
pub mod tree;

pub use element::{BoundingBox, ElementNode};
pub use selector_map::{ElementSelector, SelectorMap};
pub use tree::DomTree;

use crate::error::Result;
use headless_chrome::Tab;
use std::sync::Arc;

/// Extract the DOM tree from a browser tab
pub fn extract_dom(tab: &Arc<Tab>) -> Result<DomTree> {
    DomTree::from_tab(tab)
}

/// Extract and simplify the DOM tree
pub fn extract_simplified_dom(tab: &Arc<Tab>) -> Result<DomTree> {
    let mut tree = DomTree::from_tab(tab)?;
    tree.simplify();
    Ok(tree)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_node_export() {
        let element = ElementNode::new("div");
        assert_eq!(element.tag_name, "div");
    }

    #[test]
    fn test_selector_map_export() {
        let map = SelectorMap::new();
        assert!(map.is_empty());
    }

    #[test]
    fn test_dom_tree_export() {
        let root = ElementNode::new("body");
        let tree = DomTree::new(root);
        assert_eq!(tree.root.tag_name, "body");
    }
}
