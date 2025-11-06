use crate::error::{BrowserError, Result};
use crate::tools::{Tool, ToolContext, ToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for the click tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ClickParams {
    /// CSS selector or element index
    #[serde(flatten)]
    pub selector: ElementSelector,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ElementSelector {
    /// Select by CSS selector
    Css {
        /// CSS selector
        selector: String,
    },
    /// Select by index from DOM tree
    Index {
        /// Element index
        index: usize,
    },
}

/// Tool for clicking elements
#[derive(Default)]
pub struct ClickTool;

impl Tool for ClickTool {
    type Params = ClickParams;

    fn name(&self) -> &str {
        "click"
    }

    fn execute_typed(&self, params: ClickParams, context: &mut ToolContext) -> Result<ToolResult> {
        match params.selector {
            ElementSelector::Css { selector } => {
                let element = context.session.find_element(&selector)?;
                element
                    .click()
                    .map_err(|e| BrowserError::ToolExecutionFailed {
                        tool: "click".to_string(),
                        reason: e.to_string(),
                    })?;

                Ok(ToolResult::success_with(serde_json::json!({
                    "selector": selector,
                    "method": "css"
                })))
            }
            ElementSelector::Index { index } => {
                let css_selector = {
                    let dom = context.get_dom()?;
                    let selector_info = dom.get_selector(index).ok_or_else(|| {
                        BrowserError::ElementNotFound(format!("No element with index {}", index))
                    })?;
                    selector_info.css_selector.clone()
                };

                let element = context.session.find_element(&css_selector)?;
                element
                    .click()
                    .map_err(|e| BrowserError::ToolExecutionFailed {
                        tool: "click".to_string(),
                        reason: e.to_string(),
                    })?;

                Ok(ToolResult::success_with(serde_json::json!({
                    "index": index,
                    "selector": css_selector,
                    "method": "index"
                })))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_click_params_css() {
        let json = serde_json::json!({
            "selector": "#my-button"
        });

        let params: ClickParams = serde_json::from_value(json).unwrap();
        match params.selector {
            ElementSelector::Css { selector } => assert_eq!(selector, "#my-button"),
            _ => panic!("Expected CSS selector"),
        }
    }

    #[test]
    fn test_click_params_index() {
        let json = serde_json::json!({
            "index": 5
        });

        let params: ClickParams = serde_json::from_value(json).unwrap();
        match params.selector {
            ElementSelector::Index { index } => assert_eq!(index, 5),
            _ => panic!("Expected index selector"),
        }
    }
}
