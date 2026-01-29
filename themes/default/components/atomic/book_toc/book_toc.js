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
    
    // Toggle button functionality for the main panel
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
    
    // Handle chapter header toggle buttons
    const headerToggleBtns = tocContent.querySelectorAll('.toc-toggle-btn');
    headerToggleBtns.forEach(function(btn) {
      btn.addEventListener('click', function(e) {
        e.preventDefault();
        e.stopPropagation();
        
        const targetId = this.getAttribute('data-target');
        const targetHeaders = document.getElementById(targetId);
        
        if (!targetHeaders) {
          console.error('Target headers container not found:', targetId);
          return;
        }
        
        const isExpanded = this.getAttribute('aria-expanded') === 'true';
        
        // Toggle the expanded state
        this.setAttribute('aria-expanded', !isExpanded);
        
        // Toggle icon rotation
        const icon = this.querySelector('svg');
        if (icon) {
          if (!isExpanded) {
            icon.style.transform = 'rotate(180deg)';
          } else {
            icon.style.transform = 'rotate(0deg)';
          }
        }
        
        // Toggle headers visibility
        if (!isExpanded) {
          targetHeaders.classList.add('expanded');
        } else {
          targetHeaders.classList.remove('expanded');
        }
      });
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