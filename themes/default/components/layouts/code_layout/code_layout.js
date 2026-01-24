// Code Layout Component JavaScript

/**
 * Initialize code block tabs functionality
 */
function initializeCodeBlockTabs() {
    const tabGroups = document.querySelectorAll('.code-block-group-tabs');
    
    tabGroups.forEach(tabGroup => {
        const tabs = tabGroup.querySelectorAll('.code-block-tab');
        const codeBlocks = tabGroup.parentElement.querySelectorAll('.code-block');
        
        tabs.forEach((tab, index) => {
            tab.addEventListener('click', () => {
                // Remove active class from all tabs
                tabs.forEach(t => t.classList.remove('active'));
                
                // Hide all code blocks
                codeBlocks.forEach(block => block.style.display = 'none');
                
                // Activate clicked tab
                tab.classList.add('active');
                
                // Show corresponding code block
                if (codeBlocks[index]) {
                    codeBlocks[index].style.display = 'block';
                }
            });
        });
        
        // Activate first tab by default
        if (tabs.length > 0) {
            tabs[0].click();
        }
    });
}

/**
 * Initialize smooth scrolling for TOC links
 */
function initializeTOCScrolling() {
    const tocLinks = document.querySelectorAll('.table-of-contents a');
    
    tocLinks.forEach(link => {
        link.addEventListener('click', (e) => {
            e.preventDefault();
            
            const targetId = link.getAttribute('href');
            const targetElement = document.querySelector(targetId);
            
            if (targetElement) {
                const headerHeight = document.querySelector('.navbar').offsetHeight;
                const targetPosition = targetElement.offsetTop - headerHeight - 20;
                
                window.scrollTo({
                    top: targetPosition,
                    behavior: 'smooth'
                });
                
                // Update active state
                tocLinks.forEach(l => l.classList.remove('active'));
                link.classList.add('active');
            }
        });
    });
}

/**
 * Initialize scroll spy for TOC
 */
function initializeScrollSpy() {
    const sections = document.querySelectorAll('h2[id], h3[id], h4[id]');
    const tocLinks = document.querySelectorAll('.table-of-contents a');
    
    function updateActiveTOC() {
        const scrollPosition = window.scrollY + 100;
        
        let currentSection = '';
        sections.forEach(section => {
            const sectionTop = section.offsetTop;
            const sectionHeight = section.offsetHeight;
            
            if (scrollPosition >= sectionTop && scrollPosition < sectionTop + sectionHeight) {
                currentSection = section.getAttribute('id');
            }
        });
        
        tocLinks.forEach(link => {
            link.classList.remove('active');
            if (link.getAttribute('href') === `#${currentSection}`) {
                link.classList.add('active');
            }
        });
    }
    
    window.addEventListener('scroll', updateActiveTOC);
    updateActiveTOC(); // Initial call
}

/**
 * Initialize code example navigation
 */
function initializeCodeExampleNavigation() {
    const exampleLinks = document.querySelectorAll('.code-example-item a');
    
    exampleLinks.forEach(link => {
        link.addEventListener('click', (e) => {
            e.preventDefault();
            
            const targetId = link.getAttribute('href');
            const targetElement = document.querySelector(targetId);
            
            if (targetElement) {
                const headerHeight = document.querySelector('.navbar').offsetHeight;
                const targetPosition = targetElement.offsetTop - headerHeight - 20;
                
                window.scrollTo({
                    top: targetPosition,
                    behavior: 'smooth'
                });
                
                // Highlight the target code block
                targetElement.scrollIntoView({ behavior: 'smooth', block: 'center' });
                targetElement.classList.add('highlighted');
                
                setTimeout(() => {
                    targetElement.classList.remove('highlighted');
                }, 2000);
            }
        });
    });
}

/**
 * Initialize code block annotations
 */
function initializeCodeBlockAnnotations() {
    const annotations = document.querySelectorAll('.code-block-annotation');
    
    annotations.forEach(annotation => {
        // Add close button
        const closeButton = document.createElement('button');
        closeButton.innerHTML = '×';
        closeButton.className = 'annotation-close';
        closeButton.style.cssText = `
            position: absolute;
            top: 0.5rem;
            right: 0.5rem;
            background: none;
            border: none;
            font-size: 1.2rem;
            cursor: pointer;
            color: inherit;
            opacity: 0.7;
        `;
        
        annotation.style.position = 'relative';
        annotation.appendChild(closeButton);
        
        closeButton.addEventListener('click', () => {
            annotation.style.display = 'none';
        });
    });
}

/**
 * Initialize all code layout features
 */
function initializeCodeLayout() {
    initializeCodeBlockTabs();
    initializeTOCScrolling();
    initializeScrollSpy();
    initializeCodeExampleNavigation();
    initializeCodeBlockAnnotations();
}

/**
 * Initialize when DOM is ready
 */
document.addEventListener('DOMContentLoaded', initializeCodeLayout);

/**
 * Re-initialize when content is dynamically loaded
 */
window.CodeLayoutComponent = {
    initialize: initializeCodeLayout,
    initializeCodeBlockTabs,
    initializeTOCScrolling,
    initializeScrollSpy,
    initializeCodeExampleNavigation,
    initializeCodeBlockAnnotations
};