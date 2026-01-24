class ProjectTechComponent {
  constructor(element) {
    this.element = element;
    this.layout = element.dataset.layout;
    this.list = element.querySelector('[data-list]');
    this.items = element.querySelectorAll('.project-tech-item');
    
    this.init();
  }
  
  init() {
    this.setupEventListeners();
  }
  
  setupEventListeners() {
    this.items.forEach(item => {
      item.addEventListener('click', () => this.handleItemClick(item));
    });
  }
  
  handleItemClick(item) {
    const name = item.querySelector('.project-tech-name').textContent.trim();
    const category = item.dataset.category;
    
    this.element.dispatchEvent(new CustomEvent('tech_click', {
      detail: { name, category }
    }));
  }
  
  setLayout(layout) {
    this.layout = layout;
    this.element.dataset.layout = layout;
  }
}

// Initialize project_tech components
window.PETA_COMPONENTS = window.PETA_COMPONENTS || {};
window.PETA_COMPONENTS.project_tech = (element, props = {}) => {
  return new ProjectTechComponent(element);
};

// Auto-initialize
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('[data-component="project_tech"]').forEach(element => {
    new ProjectTechComponent(element);
  });
});