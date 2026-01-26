// Book TOC Component JavaScript
document.addEventListener('DOMContentLoaded', function() {
  const bookTocComponents = document.querySelectorAll('[data-component="book_toc"]');
  
  bookTocComponents.forEach(function(component) {
    const tocContent = component.querySelector('.book-toc-content');
    if (!tocContent) {
      console.error('Book TOC content element not found');
      return;
    }
    
    // Find parent book modal
    const bookModal = component.closest('[data-component="book_modal"]');
    if (!bookModal) {
      console.error('Book modal parent not found');
      return;
    }
    
    // Find book content in modal
    const bookContent = bookModal.querySelector('.book-modal-content');
    if (!bookContent) {
      console.error('Book content not found in modal');
      return;
    }
    
    // Extract TOC from book content
    const tocTree = bookContent.querySelector('.toc-tree');
    if (tocTree) {
      // Clone the TOC tree
      const tocClone = tocTree.cloneNode(true);
      // Update links to navigate to chapters
      const links = tocClone.querySelectorAll('a');
      links.forEach(function(link) {
        link.setAttribute('data-chapter-link', 'true');
      });
      tocContent.innerHTML = '';
      tocContent.appendChild(tocClone);
    }
    
    // Handle TOC link clicks - redirect to chapter page
    const tocLinks = tocContent.querySelectorAll('.book-toc-content a[data-chapter-link]');
    tocLinks.forEach(function(link) {
      link.addEventListener('click', function(e) {
        e.preventDefault();
        
        // Get the chapter URL
        const chapterUrl = link.getAttribute('href');
        
        // Navigate to the chapter page
        if (chapterUrl) {
          // Get current book path
          const currentPath = window.location.pathname;
          const bookPath = currentPath.substring(0, currentPath.lastIndexOf('/'));
          
          // Navigate to chapter
          window.location.href = bookPath + '/' + chapterUrl;
        }
      });
    });
  });
});