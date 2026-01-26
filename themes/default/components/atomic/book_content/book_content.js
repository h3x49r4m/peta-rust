// Book Content Component JavaScript
document.addEventListener('DOMContentLoaded', function() {
  const bookContentComponents = document.querySelectorAll('[data-component="book_content"]');
  
  bookContentComponents.forEach(function(component) {
    const bookContentWrapper = component.querySelector('.book-content-wrapper');
    
    // Handle internal links within book content
    const links = component.querySelectorAll('a[href^="#"]');
    links.forEach(function(link) {
      link.addEventListener('click', function(e) {
        e.preventDefault();
        const targetId = this.getAttribute('href').substring(1);
        const targetElement = component.querySelector(`[id="${targetId}"]`);
        
        if (targetElement) {
          targetElement.scrollIntoView({
            behavior: 'smooth',
            block: 'start'
          });
          
          // Update URL hash
          history.pushState(null, null, `#${targetId}`);
        }
      });
    });
    
    // Smooth scroll to hash on page load
    if (window.location.hash) {
      const targetId = window.location.hash.substring(1);
      const targetElement = component.querySelector(`[id="${targetId}"]`);
      if (targetElement) {
        setTimeout(function() {
          targetElement.scrollIntoView({
            behavior: 'smooth',
            block: 'start'
          });
        }, 100);
      }
    }
  });
});