class BadgeComponent {
  constructor(element) {
    this.element = element;
    this.init();
  }
  
  init() {
    // Add hover effects
    this.element.addEventListener('mouseenter', () => {
      this.element.classList.add('badge-hover');
    });
    
    this.element.addEventListener('mouseleave', () => {
      this.element.classList.remove('badge-hover');
    });
    
    // Dispatch ready event
    this.element.dispatchEvent(new CustomEvent('badge:ready'));
  }
  
  // Public API
  updateCount(count) {
    this.element.textContent = count;
  }
  
  setVariant(variant) {
    const variants = ['primary', 'secondary', 'success', 'danger', 'warning', 'info', 'light', 'dark'];
    variants.forEach(v => this.element.classList.remove(`badge-${v}`));
    this.element.classList.add(`badge-${variant}`);
  }
}

// Auto-initialize badge components
document.addEventListener('DOMContentLoaded', () => {
  const badgeElements = document.querySelectorAll('.badge');
  badgeElements.forEach(element => {
    new BadgeComponent(element);
  });
});

// Export for manual initialization
window.BadgeComponent = BadgeComponent;