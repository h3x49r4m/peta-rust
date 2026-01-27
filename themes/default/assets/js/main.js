// Math rendering for dynamically added content
function renderMathFormulas() {
    // Check if math rendering is available
    if (typeof window.mathRendererLoaded !== 'undefined') {
        // Math formulas will be rendered by the global math renderer
    }
}

// Start observing the document body for dynamic content changes
const observer = new MutationObserver(function(mutations) {
    mutations.forEach(function(mutation) {
        mutation.addedNodes.forEach(node => {
            if (node.nodeType === Node.ELEMENT_NODE) {
                const mathElements = node.querySelectorAll ? 
                    node.querySelectorAll('[data-latex]') : [];
                
                if (mathElements.length > 0) {
                    // Trigger math rendering for new content
                    if (typeof window.mathRendererLoaded !== 'undefined') {
                        setTimeout(() => {
                            renderMathFormulas();
                        }, 50);
                    }
                }
            }
        });
    });
});

observer.observe(document.body, {
    childList: true,
    subtree: true
});
