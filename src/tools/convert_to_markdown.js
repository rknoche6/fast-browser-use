// JavaScript code to convert HTML to markdown and extract page content
// Returns JSON string with title and markdown content
JSON.stringify((function() {
    // Simple HTML to Markdown converter
    function htmlToMarkdown(element) {
        if (!element) return '';
        
        const tagName = element.tagName ? element.tagName.toLowerCase() : '';
        
        // Skip unwanted elements
        if (['script', 'style', 'noscript', 'meta', 'link', 'iframe'].includes(tagName)) {
            return '';
        }
        
        // Handle text nodes
        if (element.nodeType === Node.TEXT_NODE) {
            const text = element.textContent.trim();
            return text ? text + ' ' : '';
        }
        
        let markdown = '';
        
        // Process based on tag type
        switch (tagName) {
            case 'h1':
                markdown = '\n# ' + getTextContent(element) + '\n\n';
                break;
            case 'h2':
                markdown = '\n## ' + getTextContent(element) + '\n\n';
                break;
            case 'h3':
                markdown = '\n### ' + getTextContent(element) + '\n\n';
                break;
            case 'h4':
                markdown = '\n#### ' + getTextContent(element) + '\n\n';
                break;
            case 'h5':
                markdown = '\n##### ' + getTextContent(element) + '\n\n';
                break;
            case 'h6':
                markdown = '\n###### ' + getTextContent(element) + '\n\n';
                break;
            case 'p':
                markdown = getTextContent(element) + '\n\n';
                break;
            case 'br':
                markdown = '\n';
                break;
            case 'hr':
                markdown = '\n---\n\n';
                break;
            case 'strong':
            case 'b':
                markdown = '**' + getTextContent(element) + '**';
                break;
            case 'em':
            case 'i':
                markdown = '*' + getTextContent(element) + '*';
                break;
            case 'code':
                if (element.parentElement && element.parentElement.tagName.toLowerCase() === 'pre') {
                    return ''; // Handled by pre tag
                }
                markdown = '`' + getTextContent(element) + '`';
                break;
            case 'pre':
                const codeElement = element.querySelector('code');
                const code = codeElement ? codeElement.textContent : element.textContent;
                markdown = '\n```\n' + code + '\n```\n\n';
                break;
            case 'a':
                const href = element.getAttribute('href') || '';
                const linkText = getTextContent(element);
                markdown = '[' + linkText + '](' + href + ')';
                break;
            case 'img':
                const src = element.getAttribute('src') || '';
                const alt = element.getAttribute('alt') || '';
                markdown = '![' + alt + '](' + src + ')';
                break;
            case 'ul':
            case 'ol':
                const listItems = Array.from(element.children);
                const isOrdered = tagName === 'ol';
                listItems.forEach((li, index) => {
                    if (li.tagName.toLowerCase() === 'li') {
                        const prefix = isOrdered ? (index + 1) + '. ' : '- ';
                        markdown += prefix + getTextContent(li) + '\n';
                    }
                });
                markdown += '\n';
                break;
            case 'li':
                // Handled by parent ul/ol
                return '';
            case 'blockquote':
                const lines = getTextContent(element).split('\n');
                markdown = lines.map(line => '> ' + line).join('\n') + '\n\n';
                break;
            case 'table':
                markdown = convertTable(element) + '\n\n';
                break;
            case 'div':
            case 'article':
            case 'section':
            case 'main':
            case 'body':
                // Process children
                for (let child of element.childNodes) {
                    markdown += htmlToMarkdown(child);
                }
                break;
            default:
                // For other elements, just process children
                for (let child of element.childNodes) {
                    markdown += htmlToMarkdown(child);
                }
        }
        
        return markdown;
    }
    
    function getTextContent(element) {
        if (element.nodeType === Node.TEXT_NODE) {
            return element.textContent.trim();
        }
        
        let text = '';
        for (let child of element.childNodes) {
            if (child.nodeType === Node.TEXT_NODE) {
                text += child.textContent;
            } else if (child.nodeType === Node.ELEMENT_NODE) {
                const childTag = child.tagName.toLowerCase();
                if (!['script', 'style', 'noscript'].includes(childTag)) {
                    text += getTextContent(child);
                }
            }
        }
        return text.trim();
    }
    
    function convertTable(table) {
        const rows = Array.from(table.querySelectorAll('tr'));
        if (rows.length === 0) return '';
        
        let markdown = '';
        let hasHeader = table.querySelector('th') !== null;
        
        rows.forEach((row, rowIndex) => {
            const cells = Array.from(row.querySelectorAll('th, td'));
            const cellTexts = cells.map(cell => getTextContent(cell));
            markdown += '| ' + cellTexts.join(' | ') + ' |\n';
            
            // Add header separator after first row if it has th elements
            if (rowIndex === 0 && hasHeader) {
                markdown += '| ' + cells.map(() => '---').join(' | ') + ' |\n';
            }
        });
        
        return markdown;
    }
    
    // Get page title
    const title = document.title || '';
    
    // Get main content - try to find article/main content, fallback to body
    let contentElement = document.querySelector('article') 
        || document.querySelector('main') 
        || document.querySelector('[role="main"]')
        || document.body;
    
    // Convert to markdown
    let content = htmlToMarkdown(contentElement);
    
    // Clean up extra whitespace
    content = content.replace(/\n{3,}/g, '\n\n').trim();
    
    return {
        title: title,
        content: content,
        url: window.location.href
    };
})())
