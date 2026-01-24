// Copy code functionality with enhanced feedback
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
        
        setTimeout(() => {
            button.style.background = '';
            button.borderColor = '';
        }, 2000);
    });
}

// Add line hover effect
document.addEventListener('DOMContentLoaded', function() {
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
});

// Component initialization
document.addEventListener('DOMContentLoaded', function() {
    // Header component functionality
    const header = document.querySelector('[data-component="header"]');
    if (header) {
        // Handle scroll effect for sticky header
        let lastScrollTop = 0;
        const headerHeight = header.offsetHeight;
        
        window.addEventListener('scroll', function() {
            const scrollTop = window.pageYOffset || document.documentElement.scrollTop;
            
            if (scrollTop > headerHeight) {
                header.classList.add('scrolled');
            } else {
                header.classList.remove('scrolled');
            }
            
            lastScrollTop = scrollTop;
        });
        
        // Handle mobile menu toggle
        const siteName = header.querySelector('.site-name');
        const navbar = header.querySelector('[data-component="navbar"]');
        
        if (siteName && navbar && window.innerWidth <= 768) {
            // Add mobile menu toggle button
            const menuToggle = document.createElement('button');
            menuToggle.className = 'mobile-menu-toggle';
            menuToggle.innerHTML = '☰';
            menuToggle.setAttribute('aria-label', 'Toggle navigation menu');
            menuToggle.style.cssText = 'background: none; border: none; font-size: 1.2rem; cursor: pointer; margin-left: 1rem;';
            
            siteName.parentNode.insertBefore(menuToggle, siteName.nextSibling);
            
            menuToggle.addEventListener('click', function() {
                navbar.classList.toggle('mobile-open');
                menuToggle.innerHTML = navbar.classList.contains('mobile-open') ? '✕' : '☰';
            });
        }
        
        // Handle window resize
        window.addEventListener('resize', function() {
            const mobileToggle = header.querySelector('.mobile-menu-toggle');
            if (mobileToggle && window.innerWidth > 768) {
                mobileToggle.remove();
                if (navbar) {
                    navbar.classList.remove('mobile-open');
                }
            }
        });
    }
    
    // Navbar component functionality
    const navbar = document.querySelector('[data-component="navbar"]');
    if (navbar) {
        // Handle active state for navigation items
        const navLinks = navbar.querySelectorAll('.navbar-link');
        
        // Set active link based on current URL
        const currentPath = window.location.pathname;
        navLinks.forEach(link => {
            const linkPath = new URL(link.href).pathname;
            if (linkPath === currentPath || (currentPath === '/' && linkPath === '/index.html')) {
                link.classList.add('active');
            }
            
            // Handle click events
            link.addEventListener('click', function(e) {
                navLinks.forEach(l => l.classList.remove('active'));
                this.classList.add('active');
            });
        });
    }
});

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