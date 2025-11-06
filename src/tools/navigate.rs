use crate::error::Result;
use crate::tools::{Tool, ToolContext, ToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for the navigate tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct NavigateParams {
    /// URL to navigate to
    pub url: String,

    /// Wait for navigation to complete (default: true)
    #[serde(default = "default_wait")]
    pub wait_for_load: bool,
}

fn default_wait() -> bool {
    true
}

/// Tool for navigating to a URL
#[derive(Default)]
pub struct NavigateTool;

impl Tool for NavigateTool {
    type Params = NavigateParams;

    fn name(&self) -> &str {
        "navigate"
    }

    fn execute_typed(
        &self,
        params: NavigateParams,
        context: &mut ToolContext,
    ) -> Result<ToolResult> {
        // Navigate to URL
        context.session.navigate(&params.url)?;

        // Wait for navigation if requested
        if params.wait_for_load {
            context.session.wait_for_navigation()?;
        }

        Ok(ToolResult::success_with(serde_json::json!({
            "url": params.url,
            "waited": params.wait_for_load
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigate_params_default() {
        let json = serde_json::json!({
            "url": "https://example.com"
        });

        let params: NavigateParams = serde_json::from_value(json).unwrap();
        assert_eq!(params.url, "https://example.com");
        assert!(params.wait_for_load);
    }

    #[test]
    fn test_navigate_params_explicit_wait() {
        let json = serde_json::json!({
            "url": "https://example.com",
            "wait_for_load": false
        });

        let params: NavigateParams = serde_json::from_value(json).unwrap();
        assert_eq!(params.url, "https://example.com");
        assert!(!params.wait_for_load);
    }

    #[test]
    fn test_navigate_tool_metadata() {
        let tool = NavigateTool;
        assert_eq!(tool.name(), "navigate");
        let schema = tool.parameters_schema();
        assert!(schema.is_object());
    }
}
