// Article Modal Component JavaScript
document.addEventListener('DOMContentLoaded', function() {
    const articleModalComponents = document.querySelectorAll('[data-component="article_modal"]');
    
    articleModalComponents.forEach(function(component) {
        // Debug: log the props to see what's actually being passed
        const tocElement = component.querySelector('.article-toc ul');
        if (tocElement) {
            console.log('TOC element found:', tocElement.innerHTML);
        }
    });
});