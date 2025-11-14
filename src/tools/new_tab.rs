use crate::error::Result;
use crate::tools::{Tool, ToolContext, ToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for the new_tab tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct NewTabParams {
    /// URL to open in the new tab
    pub url: String,
}

/// Tool for opening a new tab
#[derive(Default)]
pub struct NewTabTool;

impl Tool for NewTabTool {
    type Params = NewTabParams;

    fn name(&self) -> &str {
        "new_tab"
    }

    fn execute_typed(&self, params: NewTabParams, context: &mut ToolContext) -> Result<ToolResult> {
        // Create a new tab (this also sets it as active)
        // Note: We need mutable access to session, but ToolContext only provides &BrowserSession
        // The session's new_tab() method requires &mut self, so we'll need to use the underlying browser

        // Since BrowserSession methods like new_tab() require &mut self but we only have &BrowserSession,
        // we need to work around this by using the browser's new_tab() directly
        let tab = context.session.browser().new_tab().map_err(|e| {
            crate::error::BrowserError::TabOperationFailed(format!("Failed to create tab: {}", e))
        })?;

        // Navigate to the URL
        tab.navigate_to(&params.url).map_err(|e| {
            crate::error::BrowserError::NavigationFailed(format!(
                "Failed to navigate to {}: {}",
                params.url, e
            ))
        })?;

        // Wait for navigation to complete
        tab.wait_until_navigated().map_err(|e| {
            crate::error::BrowserError::NavigationFailed(format!(
                "Navigation to {} did not complete: {}",
                params.url, e
            ))
        })?;

        // Bring the new tab to front
        tab.activate().map_err(|e| {
            crate::error::BrowserError::TabOperationFailed(format!("Failed to activate tab: {}", e))
        })?;

        Ok(ToolResult::success_with(serde_json::json!({
            "url": params.url,
            "message": format!("Opened new tab with URL: {}", params.url)
        })))
    }
}
