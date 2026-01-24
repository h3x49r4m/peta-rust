document.addEventListener('DOMContentLoaded', function() {
  const navbar = document.querySelector('[data-component="navbar"]');
  if (!navbar) return;
  
  // Add mobile menu toggle if needed
  const navbarList = navbar.querySelector('.navbar-list');
  if (navbarList) {
    // Check if we're on mobile
    if (window.innerWidth <= 768) {
      navbarList.classList.add('mobile-menu');
    }
    
    // Handle window resize
    window.addEventListener('resize', function() {
      if (window.innerWidth <= 768) {
        navbarList.classList.add('mobile-menu');
      } else {
        navbarList.classList.remove('mobile-menu');
      }
    });
  }
  
  // Handle active state for navigation items
  const navLinks = navbar.querySelectorAll('.navbar-link');
  navLinks.forEach(link => {
    // Remove active class from all links
    link.addEventListener('click', function(e) {
      navLinks.forEach(l => l.classList.remove('active'));
      this.classList.add('active');
    });
  });
});