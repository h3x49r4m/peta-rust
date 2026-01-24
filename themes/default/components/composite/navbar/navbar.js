class NavbarComponent {
  constructor(element) {
    this.element = element;
    this.menu = element.querySelector('[data-navbar-menu]');
    this.toggle = element.querySelector('[data-navbar-toggle]');
    this.overlay = element.querySelector('[data-navbar-overlay]');
    this.themeToggle = element.querySelector('[data-theme-toggle]');
    
    this.state = {
      mobileOpen: false,
      scrolled: false,
      theme: 'light'
    };
    
    this.init();
  }
  
  init() {
    this.setupEventListeners();
    this.setupScrollHandler();
    this.setupThemeToggle();
  }
  
  setupEventListeners() {
    // Mobile menu toggle
    if (this.toggle) {
      this.toggle.addEventListener('click', () => this.toggleMobileMenu());
    }
    
    // Close menu when clicking overlay
    if (this.overlay) {
      this.overlay.addEventListener('click', () => this.closeMobileMenu());
    }
    
    // Close menu on escape key
    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape' && this.state.mobileOpen) {
        this.closeMobileMenu();
      }
    });
    
    // Handle resize
    window.addEventListener('resize', () => {
      if (window.innerWidth > 768 && this.state.mobileOpen) {
        this.closeMobileMenu();
      }
    });
  }
  
  setupScrollHandler() {
    if (this.element.dataset.transparent) {
      let lastScrollY = 0;
      
      window.addEventListener('scroll', () => {
        const scrollY = window.scrollY;
        
        if (scrollY > 50) {
          this.element.classList.add('navbar--scrolled');
          this.state.scrolled = true;
        } else {
          this.element.classList.remove('navbar--scrolled');
          this.state.scrolled = false;
        }
        
        lastScrollY = scrollY;
      });
    }
  }
  
  setupThemeToggle() {
    if (!this.themeToggle) return;
    
    // Initialize theme from localStorage or system preference
    const savedTheme = localStorage.getItem('theme') || 'light';
    this.setTheme(savedTheme);
    
    this.themeToggle.addEventListener('click', () => {
      const newTheme = this.state.theme === 'light' ? 'dark' : 'light';
      this.setTheme(newTheme);
    });
  }
  
  setTheme(theme) {
    this.state.theme = theme;
    
    if (theme === 'dark') {
      document.body.classList.add('dark-mode');
      this.updateThemeIcon('sun');
    } else {
      document.body.classList.remove('dark-mode');
      this.updateThemeIcon('moon');
    }
    
    localStorage.setItem('theme', theme);
    
    // Dispatch theme change event
    this.element.dispatchEvent(new CustomEvent('themechange', {
      detail: { theme }
    }));
  }
  
  updateThemeIcon(icon) {
    if (!this.themeToggle) return;
    
    // Update icon - in a real implementation, this would replace the icon
    this.themeToggle.innerHTML = `
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        ${icon === 'sun' ? this.getSunIcon() : this.getMoonIcon()}
      </svg>
    `;
  }
  
  getSunIcon() {
    return `
      <circle cx="12" cy="12" r="5"></circle>
      <line x1="12" y1="1" x2="12" y2="3"></line>
      <line x1="12" y1="21" x2="12" y2="23"></line>
      <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
      <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
      <line x1="1" y1="12" x2="3" y2="12"></line>
      <line x1="21" y1="12" x2="23" y2="12"></line>
      <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
      <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
    `;
  }
  
  getMoonIcon() {
    return `
      <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
    `;
  }
  
  toggleMobileMenu() {
    this.state.mobileOpen = !this.state.mobileOpen;
    
    if (this.state.mobileOpen) {
      this.openMobileMenu();
    } else {
      this.closeMobileMenu();
    }
  }
  
  openMobileMenu() {
    this.element.classList.add('navbar--mobile-open');
    document.body.style.overflow = 'hidden';
  }
  
  closeMobileMenu() {
    this.element.classList.remove('navbar--mobile-open');
    document.body.style.overflow = '';
  }
  
  // Public methods
  close() {
    this.closeMobileMenu();
  }
  
  setTransparent(transparent) {
    if (transparent) {
      this.element.dataset.transparent = '';
    } else {
      delete this.element.dataset.transparent;
    }
  }
}

// Initialize navbar components
window.PETA_COMPONENTS = window.PETA_COMPONENTS || {};
window.PETA_COMPONENTS.navbar = (element, props = {}) => {
  return new NavbarComponent(element);
};

// Auto-initialize
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('[data-component="navbar"]').forEach(element => {
    new NavbarComponent(element);
  });
});