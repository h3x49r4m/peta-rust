// Book TOC Component JavaScript
document.addEventListener('DOMContentLoaded', function() {
  const bookTocComponents = document.querySelectorAll('[data-component="book_toc"]');
  
  bookTocComponents.forEach(function(component) {
    const toggleBtn = component.querySelector('#book-toc-toggle');
    const tocPanel = component.querySelector('.book-toc-panel');
    const tocContent = component.querySelector('#book-toc-content');
    
    if (!toggleBtn || !tocPanel || !tocContent) {
      console.error('Book TOC toggle button, panel, or content not found');
      return;
    }
    
    // Toggle button functionality
    toggleBtn.addEventListener('click', function() {
      const isExpanded = this.getAttribute('aria-expanded') === 'true';
      
      // Toggle the expanded state
      this.setAttribute('aria-expanded', !isExpanded);
      
      // Toggle icon rotation
      if (isExpanded) {
        this.style.transform = 'rotate(0deg)';
      } else {
        this.style.transform = 'rotate(180deg)';
      }
    });
    
    // Handle TOC link clicks
    const tocLinks = tocContent.querySelectorAll('a');
    tocLinks.forEach(function(link) {
      link.addEventListener('click', function(e) {
        const href = link.getAttribute('href');
        if (href && !href.startsWith('#')) {
          // It's a chapter link, let it navigate normally
          return;
        }
      });
    });
  });
});