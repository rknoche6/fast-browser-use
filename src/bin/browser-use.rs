//! Browser-use MCP Server
//!
//! This binary provides a Model Context Protocol (MCP) server for browser automation.
//! It exposes browser automation tools that can be used by AI assistants and other MCP clients.

use browser_use::browser::LaunchOptions;
use browser_use::mcp::BrowserServer;
use rmcp::{ServiceExt, transport::stdio};
use std::io::{stdin, stdout};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();

    let mut headless = true;
    let mut help = false;

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--headed" | "-h" => headless = false,
            "--help" => help = true,
            _ => {}
        }
    }

    if help {
        print_help();
        return Ok(());
    }

    // Configure browser launch options
    let options = LaunchOptions {
        headless,
        ..Default::default()
    };

    // Create browser server
    let server = BrowserServer::with_options(options)
        .map_err(|e| format!("Failed to create browser server: {}", e))?;

    eprintln!("Browser-use MCP Server starting...");
    eprintln!(
        "Browser mode: {}",
        if headless { "headless" } else { "headed" }
    );
    eprintln!("Ready to accept MCP connections via stdio");

    // Start stdio transport
    let (_read, _write) = (stdin(), stdout());
    server.serve(stdio()).await?;

    Ok(())
}

fn print_help() {
    println!("Browser-use MCP Server v{}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("USAGE:");
    println!("    browser-use [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    --headed, -h    Launch browser in headed mode (default: headless)");
    println!("    --help          Print this help message");
    println!();
    println!("DESCRIPTION:");
    println!("    Provides browser automation tools via Model Context Protocol (MCP).");
    println!("    Communicates over stdio using JSON-RPC 2.0.");
    println!();
    println!("AVAILABLE TOOLS:");
    println!("    browser_navigate           - Navigate to a URL");
    println!("    browser_click              - Click on an element");
    println!("    browser_form_input_fill    - Fill an input field");
    println!("    browser_get_text           - Extract text content");
    println!("    browser_screenshot         - Take a screenshot");
    println!("    browser_evaluate           - Execute JavaScript");
    println!("    browser_wait               - Wait for a duration");
}
