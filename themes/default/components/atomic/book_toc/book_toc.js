// Book TOC Component JavaScript
document.addEventListener('DOMContentLoaded', function() {
  const bookTocComponents = document.querySelectorAll('[data-component="book_toc"]');
  
  bookTocComponents.forEach(function(component) {
    const button = component.querySelector('.book-toc-button');
    const popup = component.querySelector('.book-toc-popup');
    const closeButton = component.querySelector('.book-toc-close');
    const tocContent = component.querySelector('.book-toc-content');
    
    if (!button || !popup || !closeButton || !tocContent) {
      console.error('Book TOC component elements not found');
      return;
    }
    
    // Extract TOC from page content
    const pageContent = document.querySelector('.content-div');
    if (pageContent) {
      const tocTree = pageContent.querySelector('.toc-tree');
      if (tocTree) {
        // Clone the TOC tree
        const tocClone = tocTree.cloneNode(true);
        // Update links to open book modal
        const links = tocClone.querySelectorAll('a');
        links.forEach(function(link) {
          link.setAttribute('data-chapter-link', 'true');
        });
        tocContent.innerHTML = '';
        tocContent.appendChild(tocClone);
      }
    }
    
    // Toggle popup on button click
    button.addEventListener('click', function(e) {
      e.stopPropagation();
      popup.classList.toggle('active');
    });
    
    // Close popup on close button click
    closeButton.addEventListener('click', function(e) {
      e.stopPropagation();
      popup.classList.remove('active');
    });
    
    // Close popup when clicking outside
    document.addEventListener('click', function(e) {
      if (!component.contains(e.target)) {
        popup.classList.remove('active');
      }
    });
    
    // Close popup on Escape key
    document.addEventListener('keydown', function(e) {
      if (e.key === 'Escape') {
        popup.classList.remove('active');
      }
    });
    
    // Handle TOC link clicks - open book modal and load chapter
    const tocLinks = popup.querySelectorAll('.book-toc-content a[data-chapter-link]');
    tocLinks.forEach(function(link, index) {
      link.addEventListener('click', function(e) {
        e.preventDefault();
        
        // Close the TOC popup
        popup.classList.remove('active');
        
        // Open book modal with the selected chapter
        const bookModal = document.querySelector('[data-component="book_modal"]');
        if (bookModal) {
          // Try to find the BookModal instance
          const modalInstances = Object.values(window).filter(obj => 
            obj && obj.constructor && obj.constructor.name === 'BookModal'
          );
          
          if (modalInstances.length > 0) {
            modalInstances[0].openWithChapter(index);
          } else {
            console.error('BookModal instance not found');
          }
        }
      });
    });
  });
});