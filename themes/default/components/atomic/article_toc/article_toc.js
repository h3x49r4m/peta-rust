// Article TOC Component JavaScript
document.addEventListener('DOMContentLoaded', function() {
  const articleTocComponents = document.querySelectorAll('[data-component="article_toc"]');
  
  articleTocComponents.forEach(function(component) {
    // Initialize article TOC component
    console.log('Article TOC component initialized');
    
    // Add smooth scrolling to TOC links
    const tocLinks = component.querySelectorAll('a[href^="#"]');
    tocLinks.forEach(function(link) {
      link.addEventListener('click', function(e) {
        e.preventDefault();
        const targetId = this.getAttribute('href').substring(1);
        const targetElement = document.getElementById(targetId);
        
        if (targetElement) {
          // Smooth scroll to target
          targetElement.scrollIntoView({
            behavior: 'smooth',
            block: 'start'
          });
          
          // Update active state
          tocLinks.forEach(l => l.classList.remove('active'));
          this.classList.add('active');
          
          // Update URL without jumping
          history.pushState(null, null, '#' + targetId);
        }
      });
    });
    
    // Highlight current section on scroll
    function updateActiveTocItem() {
      const sections = document.querySelectorAll('h1[id], h2[id], h3[id], h4[id], h5[id], h6[id]');
      const scrollPosition = window.scrollY + 100;
      
      let currentSection = '';
      sections.forEach(function(section) {
        const sectionTop = section.offsetTop;
        if (scrollPosition >= sectionTop) {
          currentSection = section.getAttribute('id');
        }
      });
      
      if (currentSection) {
        tocLinks.forEach(function(link) {
          link.classList.remove('active');
          if (link.getAttribute('href') === '#' + currentSection) {
            link.classList.add('active');
          }
        });
      }
    }
    
    // Throttled scroll handler
    let scrollTimer = null;
    window.addEventListener('scroll', function() {
      if (scrollTimer) clearTimeout(scrollTimer);
      scrollTimer = setTimeout(updateActiveTocItem, 100);
    });
    
    // Initial check for active item
    updateActiveTocItem();
  });
});