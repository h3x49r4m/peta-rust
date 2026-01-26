// Page Tags Component JavaScript
document.addEventListener('DOMContentLoaded', function() {
  const pageTagsComponents = document.querySelectorAll('[data-component="page_tags"]');
  
  pageTagsComponents.forEach(function(component) {
    // Initialize page tags component
    console.log('Page tags component initialized');
    
    // Get the page title link and ensure it has the correct URL
    const pageTitleLink = component.querySelector('.page-title-link');
    if (pageTitleLink) {
      const currentHref = pageTitleLink.getAttribute('href');
      
      // Determine the correct URL based on the page title
      const pageTitle = component.querySelector('.page-title').textContent.trim().toLowerCase();
      const pageUrls = {
        'articles': '/articles.html',
        'books': '/books.html',
        'projects': '/projects.html',
        'snippets': '/snippets.html'
      };
      
      let targetUrl = currentHref;
      
      // If current href is a book page (contains /books/), redirect to books.html
      if (currentHref && currentHref.includes('/books/')) {
        targetUrl = '/books.html';
      } else if (currentHref && currentHref.includes('/articles/')) {
        targetUrl = '/articles.html';
      } else if (currentHref && currentHref.includes('/projects/')) {
        targetUrl = '/projects.html';
      } else if (currentHref && currentHref.includes('/snippets/')) {
        targetUrl = '/snippets.html';
      } else if (pageUrls[pageTitle]) {
        targetUrl = pageUrls[pageTitle];
      } else if (!currentHref || currentHref === '#') {
        // Default to home if page is not recognized
        targetUrl = '/';
      }
      
      pageTitleLink.setAttribute('href', targetUrl);
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