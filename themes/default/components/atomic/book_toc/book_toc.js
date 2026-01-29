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
    
    // Restore panel state from localStorage
    const storageKey = 'book-toc-expanded';
    const wasExpanded = localStorage.getItem(storageKey) === 'true';
    
    if (wasExpanded) {
      toggleBtn.setAttribute('aria-expanded', 'true');
      toggleBtn.style.transform = 'rotate(180deg)';
    }
    
    // Toggle button functionality for the main panel
    toggleBtn.addEventListener('click', function(e) {
      e.preventDefault();
      e.stopPropagation();
      
      const isExpanded = this.getAttribute('aria-expanded') === 'true';
      const newExpanded = !isExpanded;
      
      // Toggle the expanded state
      this.setAttribute('aria-expanded', newExpanded);
      
      // Save to localStorage
      localStorage.setItem(storageKey, newExpanded.toString());
      
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
    
    // Handle TOC link clicks - do NOT collapse the panel
    const tocLinks = tocContent.querySelectorAll('a');
    tocLinks.forEach(function(link) {
      link.addEventListener('click', function(e) {
        // Check if this is a chapter link (has class toc-chapter-link)
        if (link.classList.contains('toc-chapter-link')) {
          // Find the toggle button in the same header
          const headerContainer = link.closest('.toc-item-header');
          if (headerContainer) {
            const toggleBtn = headerContainer.querySelector('.toc-toggle-btn');
            if (toggleBtn) {
              const targetId = toggleBtn.getAttribute('data-target');
              const targetHeaders = document.getElementById(targetId);
              
              if (targetHeaders) {
                // Expand the headers if not already expanded
                if (toggleBtn.getAttribute('aria-expanded') !== 'true') {
                  toggleBtn.setAttribute('aria-expanded', 'true');
                  const icon = toggleBtn.querySelector('svg');
                  if (icon) {
                    icon.style.transform = 'rotate(180deg)';
                  }
                  targetHeaders.classList.add('expanded');
                  
                  // Store the expanded chapter in localStorage
                  localStorage.setItem('book-toc-expanded-chapter', targetId);
                }
              }
            }
          }
        }
        
        // Let the link navigate normally
        // The panel state will be preserved via localStorage
      });
    });
    
    // Restore expanded chapter from localStorage
    const expandedChapterId = localStorage.getItem('book-toc-expanded-chapter');
    if (expandedChapterId) {
      const targetHeaders = document.getElementById(expandedChapterId);
      if (targetHeaders) {
        const toggleBtn = tocContent.querySelector(`[data-target="${expandedChapterId}"]`);
        if (toggleBtn) {
          toggleBtn.setAttribute('aria-expanded', 'true');
          const icon = toggleBtn.querySelector('svg');
          if (icon) {
            icon.style.transform = 'rotate(180deg)';
          }
          targetHeaders.classList.add('expanded');
        }
      }
    }
  });
});