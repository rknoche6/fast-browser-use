/// Normalize an incomplete URL by adding missing protocol and handling common patterns
pub fn normalize_url(url: &str) -> String {
    let trimmed = url.trim();

    // If already has a protocol, return as-is
    if trimmed.starts_with("http://")
        || trimmed.starts_with("https://")
        || trimmed.starts_with("file://")
        || trimmed.starts_with("data:")
        || trimmed.starts_with("about:")
        || trimmed.starts_with("chrome://")
        || trimmed.starts_with("chrome-extension://")
    {
        return trimmed.to_string();
    }

    // Relative path - return as-is
    if trimmed.starts_with('/') || trimmed.starts_with("./") || trimmed.starts_with("../") {
        return trimmed.to_string();
    }

    // localhost special case - use http by default
    if trimmed.starts_with("localhost") || trimmed.starts_with("127.0.0.1") {
        return format!("http://{}", trimmed);
    }

    // Check if it looks like a domain (contains dot or is a known TLD)
    if trimmed.contains('.') {
        // Looks like a domain - add https://
        return format!("https://{}", trimmed);
    }

    // Single word - assume it's a domain name, add www. prefix and https://
    // This handles cases like "google" -> "https://www.google.com"
    format!("https://www.{}.com", trimmed)
}
