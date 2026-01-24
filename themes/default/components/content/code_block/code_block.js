// Enhanced Code Block Component JavaScript

/**
 * Copy code to clipboard with enhanced feedback
 * @param {HTMLElement} button - The copy button element
 */
function copyCode(button) {
    const codeBlock = button.closest('.code-block');
    const codeElement = codeBlock.querySelector('code');
    const text = codeElement.textContent;
    
    navigator.clipboard.writeText(text).then(() => {
        const originalText = button.innerHTML;
        button.classList.add('copied');
        button.innerHTML = `
            <svg class="code-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="20 6 9 17 4 12"></polyline>
            </svg>
            <span class="copy-text">Copied!</span>
        `;
        
        // Visual feedback
        button.style.background = 'rgba(16, 185, 129, 0.2)';
        button.style.borderColor = 'rgba(16, 185, 129, 0.5)';
        
        setTimeout(() => {
            button.innerHTML = originalText;
            button.classList.remove('copied');
            button.style.background = '';
            button.style.borderColor = '';
        }, 2000);
    }).catch(err => {
        console.error('Failed to copy code:', err);
        // Show error feedback
        button.style.background = 'rgba(239, 68, 68, 0.2)';
        button.style.borderColor = 'rgba(239, 68, 68, 0.5)';
        button.innerHTML = `
            <svg class="code-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10"></circle>
                <line x1="15" y1="9" x2="9" y2="15"></line>
                <line x1="9" y1="9" x2="15" y2="15"></line>
            </svg>
            <span class="copy-text">Error</span>
        `;
        
        setTimeout(() => {
            button.innerHTML = `
                <svg class="code-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                    <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                </svg>
                <span class="copy-text">Copy</span>
            `;
            button.style.background = '';
            button.style.borderColor = '';
        }, 2000);
    });
}

/**
 * Initialize line hover effects
 */
function initializeLineHoverEffects() {
    const codeBlocks = document.querySelectorAll('.code-block');
    
    codeBlocks.forEach(block => {
        const lines = block.querySelectorAll('.line-number');
        
        lines.forEach((line, index) => {
            line.addEventListener('mouseenter', function() {
                const lineNumber = parseInt(this.getAttribute('data-line'));
                const codeLines = block.querySelectorAll('code > span');
                
                // Highlight the current line
                if (codeLines[lineNumber - 1]) {
                    codeLines[lineNumber - 1].style.background = 'rgba(59, 130, 246, 0.1)';
                }
            });
            
            line.addEventListener('mouseleave', function() {
                const lineNumber = parseInt(this.getAttribute('data-line'));
                const codeLines = block.querySelectorAll('code > span');
                
                // Remove highlight
                if (codeLines[lineNumber - 1]) {
                    codeLines[lineNumber - 1].style.background = '';
                }
            });
        });
    });
}

/**
 * Initialize keyboard shortcuts for code blocks
 */
function initializeKeyboardShortcuts() {
    document.addEventListener('keydown', function(event) {
        // Ctrl/Cmd + K to copy focused code block
        if ((event.ctrlKey || event.metaKey) && event.key === 'k') {
            const focusedElement = document.activeElement;
            const codeBlock = focusedElement.closest('.code-block');
            
            if (codeBlock) {
                const copyButton = codeBlock.querySelector('.code-copy-button');
                if (copyButton) {
                    event.preventDefault();
                    copyButton.click();
                }
            }
        }
    });
}

/**
 * Initialize code block enhancements when DOM is ready
 */
document.addEventListener('DOMContentLoaded', function() {
    initializeLineHoverEffects();
    initializeKeyboardShortcuts();
    
    // Add line numbers if needed
    const codeBlocks = document.querySelectorAll('.code-block[data-line-numbers="true"]');
    codeBlocks.forEach(block => {
        const codeElement = block.querySelector('code');
        const lines = codeElement.textContent.split('\n');
        
        // Add line numbers
        const numberedLines = lines.map((line, index) => {
            const lineNumber = index + 1;
            const isHighlighted = block.getAttribute('data-highlight-lines')?.split(',').includes(String(lineNumber));
            const highlightClass = isHighlighted ? 'line-highlight' : '';
            
            return `<span class="${highlightClass}"><span class="line-number" data-line="${lineNumber}">${lineNumber}</span>${line}</span>`;
        }).join('\n');
        
        codeElement.innerHTML = numberedLines;
    });
});

/**
 * Export functions for external use
 */
window.CodeBlockComponent = {
    copyCode,
    initializeLineHoverEffects,
    initializeKeyboardShortcuts
};