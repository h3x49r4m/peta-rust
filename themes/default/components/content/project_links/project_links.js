class ProjectLinksComponent {
  constructor(element) {
    this.element = element;
    this.links = element.querySelectorAll('.project-links-link');
    
    this.init();
  }
  
  init() {
    this.setupEventListeners();
  }
  
  setupEventListeners() {
    this.links.forEach(link => {
      link.addEventListener('click', () => this.handleLinkClick(link));
    });
  }
  
  handleLinkClick(link) {
    const type = Array.from(link.classList).find(c => c.startsWith('project-links-link--'))?.replace('project-links-link--', '');
    
    this.element.dispatchEvent(new CustomEvent('project_link_click', {
      detail: { 
        type,
        url: link.href,
        text: link.textContent.trim()
      }
    }));
  }
}

// Initialize project_links components
window.PETA_COMPONENTS = window.PETA_COMPONENTS || {};
window.PETA_COMPONENTS.project_links = (element, props = {}) => {
  return new ProjectLinksComponent(element);
};

// Auto-initialize
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('[data-component="project_links"]').forEach(element => {
    new ProjectLinksComponent(element);
  });
});