// JavaScript code to extract simplified DOM structure
// Returns JSON string to avoid object serialization issues
JSON.stringify((function() {
    function isVisible(element) {
        if (!element) return false;
        
        const style = window.getComputedStyle(element);
        if (style.display === 'none' || style.visibility === 'hidden' || style.opacity === '0') {
            return false;
        }
        
        const rect = element.getBoundingClientRect();
        return rect.width > 0 && rect.height > 0;
    }

    function extractElement(element, depth = 0, maxDepth = 10) {
        if (depth > maxDepth) return null;
        
        const tagName = element.tagName.toLowerCase();
        
        // Skip certain elements
        if (['script', 'style', 'noscript', 'meta', 'link'].includes(tagName)) {
            return null;
        }

        const node = {
            tag_name: tagName,
            attributes: {},
            text_content: null,
            children: [],
            is_visible: isVisible(element),
            is_interactive: false,
            bounding_box: null
        };

        // Extract attributes
        for (let attr of element.attributes) {
            node.attributes[attr.name] = attr.value;
        }

        // Get bounding box if visible
        if (node.is_visible) {
            const rect = element.getBoundingClientRect();
            node.bounding_box = {
                x: rect.x,
                y: rect.y,
                width: rect.width,
                height: rect.height
            };
        }

        // Extract text content (only direct text, not from children)
        const textContent = Array.from(element.childNodes)
            .filter(n => n.nodeType === Node.TEXT_NODE)
            .map(n => n.textContent.trim())
            .filter(t => t.length > 0)
            .join(' ');
        
        if (textContent) {
            node.text_content = textContent;
        }

        // Recursively extract children
        for (let child of element.children) {
            const childNode = extractElement(child, depth + 1, maxDepth);
            if (childNode) {
                node.children.push(childNode);
            }
        }

        return node;
    }

    // Start extraction from body
    const body = document.body;
    if (!body) {
        return { tag_name: 'body', attributes: {}, children: [], is_visible: false };
    }

    return extractElement(body);
})())
