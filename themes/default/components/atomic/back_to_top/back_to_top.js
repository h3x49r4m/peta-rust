(function() {
  'use strict';
  
  // Configuration
  const SCROLL_THRESHOLD = 300; // Show button after scrolling 300px
  const SCROLL_DEBOUNCE = 100; // Debounce scroll events by 100ms
  
  let backToTopButton = null;
  let lastScrollTop = 0;
  let scrollTimeout = null;
  let ticking = false;
  
  // Initialize the back to top button
  function initBackToTop() {
    backToTopButton = document.querySelector('[data-component="back_to_top"]');
    
    if (!backToTopButton) {
      console.warn('Back to top button not found');
      return;
    }
    
    // Add click event listener
    backToTopButton.addEventListener('click', scrollToTop);
    
    // Add keyboard support (Enter and Space keys)
    backToTopButton.addEventListener('keydown', function(e) {
      if (e.key === 'Enter' || e.key === ' ') {
        e.preventDefault();
        scrollToTop();
      }
    });
    
    // Add scroll event listener with requestAnimationFrame
    window.addEventListener('scroll', handleScroll, { passive: true });
    
    // Check initial scroll position
    updateButtonVisibility();
  }
  
  // Handle scroll event with performance optimization
  function handleScroll() {
    if (!ticking) {
      window.requestAnimationFrame(function() {
        updateButtonVisibility();
        ticking = false;
      });
      
      ticking = true;
    }
  }
  
  // Update button visibility based on scroll position
  function updateButtonVisibility() {
    if (!backToTopButton) return;
    
    const scrollTop = window.pageYOffset || document.documentElement.scrollTop;
    
    if (scrollTop > SCROLL_THRESHOLD) {
      backToTopButton.classList.add('visible');
    } else {
      backToTopButton.classList.remove('visible');
    }
    
    lastScrollTop = scrollTop;
  }
  
  // Scroll to top smoothly
  function scrollToTop() {
    const scrollOptions = {
      top: 0,
      behavior: 'smooth'
    };
    
    // Check if smooth scrolling is supported
    if ('scrollBehavior' in document.documentElement.style) {
      window.scrollTo(scrollOptions);
    } else {
      // Fallback for older browsers
      const currentScroll = window.pageYOffset || document.documentElement.scrollTop;
      const step = currentScroll / 30;
      
      function animateScroll() {
        const newScroll = window.pageYOffset || document.documentElement.scrollTop;
        
        if (newScroll > 0) {
          window.scrollTo(0, newScroll - step);
          requestAnimationFrame(animateScroll);
        }
      }
      
      animateScroll();
    }
    
    // Focus back to top of page for accessibility
    setTimeout(function() {
      document.body.focus();
    }, 500);
  }
  
  // Initialize when DOM is ready
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initBackToTop);
  } else {
    initBackToTop();
  }
  
  // Clean up on page unload
  window.addEventListener('beforeunload', function() {
    if (backToTopButton) {
      backToTopButton.removeEventListener('click', scrollToTop);
    }
    window.removeEventListener('scroll', handleScroll);
  });
})();