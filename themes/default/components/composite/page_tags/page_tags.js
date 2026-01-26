// Page Tags Component JavaScript
document.addEventListener('DOMContentLoaded', function() {
  const pageTagsComponents = document.querySelectorAll('[data-component="page_tags"]');
  
  pageTagsComponents.forEach(function(component) {
    // Initialize page tags component
    console.log('Page tags component initialized');
    
    // Get the page title link and ensure it has the correct URL
    const pageTitleLink = component.querySelector('.page-title-link');
    if (pageTitleLink) {
      // If no href is set, determine it based on the current page
      if (!pageTitleLink.getAttribute('href') || pageTitleLink.getAttribute('href') === '#') {
        const pageTitle = component.querySelector('.page-title').textContent.trim().toLowerCase();
        const pageUrls = {
          'articles': '/articles.html',
          'books': '/books.html',
          'projects': '/projects.html',
          'snippets': '/snippets.html'
        };
        
        if (pageUrls[pageTitle]) {
          pageTitleLink.setAttribute('href', pageUrls[pageTitle]);
        } else {
          // Default to home if page is not recognized
          pageTitleLink.setAttribute('href', '/');
        }
      }
    }
    
    // Add any interactive behavior here
    component.addEventListener('click', function(e) {
      // Handle clicks within the component
      if (e.target.classList.contains('tag-cloud-tag')) {
        console.log('Tag clicked:', e.target.textContent);
      }
    });
  });
});