class ProjectModalComponent {
  constructor(element) {
    this.element = element;
    this.overlay = element.querySelector('[data-overlay]');
    this.dialog = element.querySelector('[data-dialog]');
    this.closeButton = element.querySelector('[data-close]');
    
    this.state = {
      isOpen: false
    };
    
    this.init();
  }
  
  init() {
    this.setupEventListeners();
  }
  
  setupEventListeners() {
    // Close button
    if (this.closeButton) {
      this.closeButton.addEventListener('click', () => this.close());
    }
    
    // Overlay click
    this.overlay.addEventListener('click', () => this.close());
    
    // Escape key
    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape' && this.state.isOpen) {
        this.close();
      }
    });
  }
  
  open() {
    this.state.isOpen = true;
    this.element.dataset.open = 'true';
    document.body.style.overflow = 'hidden';
    
    this.element.dispatchEvent(new CustomEvent('modal_open'));
  }
  
  close() {
    this.state.isOpen = false;
    this.element.dataset.open = 'false';
    document.body.style.overflow = '';
    
    this.element.dispatchEvent(new CustomEvent('modal_close'));
  }
  
  toggle() {
    if (this.state.isOpen) {
      this.close();
    } else {
      this.open();
    }
  }
}

// Initialize project_modal components
window.PETA_COMPONENTS = window.PETA_COMPONENTS || {};
window.PETA_COMPONENTS.project_modal = (element, props = {}) => {
  return new ProjectModalComponent(element);
};

// Auto-initialize
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('[data-component="project_modal"]').forEach(element => {
    new ProjectModalComponent(element);
  });
});