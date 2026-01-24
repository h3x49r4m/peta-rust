document.addEventListener('DOMContentLoaded', function() {
  const header = document.querySelector('[data-component="header"]');
  if (!header) return;
  
  // Handle scroll effect for sticky header
  let lastScrollTop = 0;
  const headerHeight = header.offsetHeight;
  
  window.addEventListener('scroll', function() {
    const scrollTop = window.pageYOffset || document.documentElement.scrollTop;
    
    if (scrollTop > headerHeight) {
      header.classList.add('scrolled');
    } else {
      header.classList.remove('scrolled');
    }
    
    lastScrollTop = scrollTop;
  });
  
  // Handle mobile menu toggle
  const siteName = header.querySelector('.site-name');
  const navbar = header.querySelector('[data-component="navbar"]');
  
  if (siteName && navbar && window.innerWidth <= 768) {
    // Add mobile menu toggle button
    const menuToggle = document.createElement('button');
    menuToggle.className = 'mobile-menu-toggle';
    menuToggle.innerHTML = '☰';
    menuToggle.setAttribute('aria-label', 'Toggle navigation menu');
    
    siteName.parentNode.insertBefore(menuToggle, siteName.nextSibling);
    
    menuToggle.addEventListener('click', function() {
      navbar.classList.toggle('mobile-open');
      menuToggle.innerHTML = navbar.classList.contains('mobile-open') ? '✕' : '☰';
    });
  }
  
  // Handle window resize
  window.addEventListener('resize', function() {
    const mobileToggle = header.querySelector('.mobile-menu-toggle');
    if (mobileToggle && window.innerWidth > 768) {
      mobileToggle.remove();
      if (navbar) {
        navbar.classList.remove('mobile-open');
      }
    }
  });
});