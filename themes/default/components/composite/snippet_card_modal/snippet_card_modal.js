(function() {
    'use strict';
    
    // Initialize modal functionality
    function initSnippetModal() {
        const modalOverlay = document.querySelector('.snippet-modal-overlay');
        if (!modalOverlay) return;
        
        const closeBtn = modalOverlay.querySelector('.snippet-modal-close');
        const copyBtn = modalOverlay.querySelector('.snippet-modal-copy-btn');
        
        // Close modal when clicking close button
        if (closeBtn) {
            closeBtn.addEventListener('click', closeModal);
        }
        
        // Close modal when clicking overlay
        modalOverlay.addEventListener('click', function(e) {
            if (e.target === modalOverlay) {
                closeModal();
            }
        });
        
        // Close modal with Escape key
        document.addEventListener('keydown', function(e) {
            if (e.key === 'Escape' && modalOverlay.classList.contains('active')) {
                closeModal();
            }
        });
        
        // Copy functionality
        if (copyBtn) {
            copyBtn.addEventListener('click', function() {
                const content = copyBtn.getAttribute('data-copy-content');
                if (!content) return;
                
                navigator.clipboard.writeText(content).then(function() {
                    // Show success state
                    copyBtn.classList.add('copied');
                    const originalText = copyBtn.innerHTML;
                    copyBtn.innerHTML = `
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="20 6 9 17 4 12"></polyline>
                        </svg>
                        Copied!
                    `;
                    
                    // Reset after 2 seconds
                    setTimeout(function() {
                        copyBtn.classList.remove('copied');
                        copyBtn.innerHTML = originalText;
                    }, 2000);
                }).catch(function(err) {
                    console.error('Failed to copy: ', err);
                    // Show error state
                    copyBtn.style.backgroundColor = '#ef4444';
                    copyBtn.innerHTML = `
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <circle cx="12" cy="12" r="10"></circle>
                            <line x1="12" y1="8" x2="12" y2="12"></line>
                            <line x1="12" y1="16" x2="12.01" y2="16"></line>
                        </svg>
                        Failed!
                    `;
                    
                    // Reset after 2 seconds
                    setTimeout(function() {
                        copyBtn.style.backgroundColor = '';
                        copyBtn.innerHTML = originalText;
                    }, 2000);
                });
            });
        }
    }
    
    // Open modal function
    window.openSnippetModal = function(snippet) {
        const modalOverlay = document.querySelector('.snippet-modal-overlay');
        if (!modalOverlay) return;
        
        // Update modal content
        const title = modalOverlay.querySelector('.snippet-modal-title');
        const language = modalOverlay.querySelector('.snippet-modal-language');
        const body = modalOverlay.querySelector('.snippet-modal-body');
        const date = modalOverlay.querySelector('.snippet-modal-date');
        const tagsContainer = modalOverlay.querySelector('.snippet-modal-tags');
        const copyBtn = modalOverlay.querySelector('.snippet-modal-copy-btn');
        
        if (title) title.textContent = snippet.title || 'Snippet';
        if (language) {
            if (snippet.language) {
                language.textContent = snippet.language;
                language.style.display = 'inline-block';
            } else {
                language.style.display = 'none';
            }
        }
        
        if (body) {
            body.innerHTML = snippet.content || '<p>No content available</p>';
            
            // Check if content has math formulas and handle them using KaTeX
            const hasMathFormulas = snippet.content && (
                snippet.content.includes('data-latex') || 
                snippet.content.includes('class="math"') ||
                snippet.content.includes('\\(') || 
                snippet.content.includes('\\[') ||
                snippet.content.includes('$$') ||
                snippet.content.includes('\\begin{') ||
                snippet.content.includes('\\\\(') ||
                snippet.content.includes('\\\\[')
            );
            
            if (hasMathFormulas) {
                console.log('Content has math formulas, ensuring KaTeX is loaded...');
                
                // Use the same KaTeX system as main pages
                if (typeof window.mathRendererLoaded === 'undefined') {
                    window.mathRendererLoaded = false;
                    window.pendingMathRender = false;
                }
                
                function loadKaTeX() {
                    if (window.mathRendererLoaded) {
                        renderMathInElement(body, {
                            delimiters: [
                                {left: '$$', right: '$$', display: true},
                                {left: '$', right: '$', display: false},
                                {left: '\\[', right: '\\]', display: true},
                                {left: '\\(', right: '\\)', display: false}
                            ]
                        });
                        return;
                    }
                    
                    // Load KaTeX CSS if not already loaded
                    if (!document.querySelector('link[href*="katex.min.css"]')) {
                        const css = document.createElement('link');
                        css.rel = 'stylesheet';
                        css.href = 'https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css';
                        document.head.appendChild(css);
                    }
                    
                    // Load KaTeX JS if not already loaded
                    if (typeof window.katex === 'undefined') {
                        const katex = document.createElement('script');
                        katex.src = 'https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.js';
                        katex.onload = function() {
                            const autoRender = document.createElement('script');
                            autoRender.src = 'https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/contrib/auto-render.min.js';
                            autoRender.onload = function() {
                                window.mathRendererLoaded = true;
                                renderMathInElement(body, {
                                    delimiters: [
                                        {left: '$$', right: '$$', display: true},
                                        {left: '$', right: '$', display: false},
                                        {left: '\\[', right: '\\]', display: true},
                                        {left: '\\(', right: '\\)', display: false}
                                    ]
                                });
                            };
                            document.body.appendChild(autoRender);
                        };
                        document.body.appendChild(katex);
                    } else {
                        // KaTeX already loaded, use auto-render
                        if (typeof renderMathInElement !== 'undefined') {
                            renderMathInElement(body, {
                                delimiters: [
                                    {left: '$$', right: '$$', display: true},
                                    {left: '$', right: '$', display: false},
                                    {left: '\\[', right: '\\]', display: true},
                                    {left: '\\(', right: '\\)', display: false}
                                ]
                            });
                        }
                    }
                }
                
                loadKaTeX();
            }
        }
        
        if (date) {
            if (snippet.date) {
                date.textContent = snippet.date;
                date.style.display = 'inline';
            } else {
                date.style.display = 'none';
            }
        }
        if (tagsContainer && snippet.tags) {
            tagsContainer.innerHTML = snippet.tags.map(tag => 
                `<span class="snippet-modal-tag">${tag}</span>`
            ).join('');
            tagsContainer.style.display = 'flex';
        } else if (tagsContainer) {
            tagsContainer.style.display = 'none';
        }
        if (copyBtn) {
            copyBtn.setAttribute('data-copy-content', snippet.content || '');
        }
        
        // Show modal
        modalOverlay.style.display = 'flex';
        // Force reflow
        modalOverlay.offsetHeight;
        modalOverlay.classList.add('active');
        
        // Prevent body scroll
        document.body.style.overflow = 'hidden';
    };
    
    // Close modal function
    function closeModal() {
        const modalOverlay = document.querySelector('.snippet-modal-overlay');
        if (!modalOverlay) return;
        
        modalOverlay.classList.remove('active');
        
        setTimeout(function() {
            modalOverlay.style.display = 'none';
            // Restore body scroll
            document.body.style.overflow = '';
        }, 300);
    }
    
    // Make closeModal globally available
    window.closeSnippetModal = closeModal;
    
    // Initialize on DOM content loaded
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', initSnippetModal);
    } else {
        initSnippetModal();
    }
})();