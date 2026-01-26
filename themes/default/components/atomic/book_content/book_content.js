// Book Content Component JavaScript
document.addEventListener('DOMContentLoaded', function() {
  const bookContentComponents = document.querySelectorAll('[data-component="book_content"]');
  
  bookContentComponents.forEach(function(component) {
    const toggleBtn = component.querySelector('#book-toc-toggle');
    const tocSidebar = component.querySelector('#book-toc-sidebar');
    const tocContent = component.querySelector('.book-toc-content');
    const bookContentWrapper = component.querySelector('.book-content-wrapper');
    
    // Toggle TOC sidebar
    if (toggleBtn && tocSidebar) {
      toggleBtn.addEventListener('click', function() {
        tocSidebar.classList.toggle('active');
        this.classList.toggle('active');
      });
      
      // Close sidebar when clicking outside
      document.addEventListener('click', function(e) {
        if (!tocSidebar.contains(e.target) && !toggleBtn.contains(e.target)) {
          tocSidebar.classList.remove('active');
          toggleBtn.classList.remove('active');
        }
      });
    }
    
    // Extract and populate TOC from content
    if (tocContent && bookContentWrapper) {
      const headings = bookContentWrapper.querySelectorAll('h2, h3, h4, h5, h6');
      const tocList = document.createElement('ul');
      let currentSubList = null;
      
      headings.forEach(function(heading) {
        const id = heading.getAttribute('id');
        if (!id) return;
        
        const link = document.createElement('a');
        link.href = `#${id}`;
        link.textContent = heading.textContent;
        link.addEventListener('click', function(e) {
          e.preventDefault();
          const targetElement = document.getElementById(id);
          if (targetElement) {
            targetElement.scrollIntoView({
              behavior: 'smooth',
              block: 'start'
            });
            history.pushState(null, null, `#${id}`);
            
            // Update active state
            tocContent.querySelectorAll('a').forEach(a => a.classList.remove('active'));
            link.classList.add('active');
            
            // Close sidebar on mobile
            if (window.innerWidth <= 1024) {
              tocSidebar.classList.remove('active');
              toggleBtn.classList.remove('active');
            }
          }
        });
        
        const listItem = document.createElement('li');
        listItem.appendChild(link);
        
        // Handle nested structure
        if (heading.tagName === 'H3' || heading.tagName === 'H4') {
          if (!currentSubList) {
            currentSubList = document.createElement('ul');
            tocList.appendChild(currentSubList);
          }
          currentSubList.appendChild(listItem);
        } else {
          currentSubList = null;
          tocList.appendChild(listItem);
        }
      });
      
      tocContent.appendChild(tocList);
    }
    
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
          
          // Update active TOC link
          if (tocContent) {
            tocContent.querySelectorAll('a').forEach(a => a.classList.remove('active'));
            const activeLink = tocContent.querySelector(`a[href="#${targetId}"]`);
            if (activeLink) {
              activeLink.classList.add('active');
            }
          }
        }
      });
    });
    
    // Update active TOC link on scroll
          if (tocContent) {
            const headings = bookContentWrapper.querySelectorAll('h2, h3, h4, h5, h6');
            
            function updateActiveTOC() {
              let currentHeading = null;
              headings.forEach(function(heading) {
                const rect = heading.getBoundingClientRect();
                if (rect.top <= 150) {
                  currentHeading = heading;
                }
              });
              
              if (currentHeading) {
                const id = currentHeading.getAttribute('id');
                tocContent.querySelectorAll('a').forEach(a => a.classList.remove('active'));
                const activeLink = tocContent.querySelector(`a[href="#${id}"]`);
                if (activeLink) {
                  activeLink.classList.add('active');
                }
              }
            }
            
            window.addEventListener('scroll', updateActiveTOC);
          }    
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