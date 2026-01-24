// Projects Layout JavaScript
class ProjectsLayout {
  constructor(element) {
    this.element = element;
    this.init();
  }
  
  init() {
    this.setupStatusFilter();
    this.setupProjectCards();
    this.setupTechTags();
  }
  
  setupStatusFilter() {
    const statusFilter = this.element.querySelector('#status-filter');
    if (statusFilter) {
      statusFilter.addEventListener('change', (e) => {
        const status = e.target.value;
        const url = new URL(window.location);
        if (status) {
          url.searchParams.set('status', status);
        } else {
          url.searchParams.delete('status');
        }
        window.location = url.toString();
      });
    }
  }
  
  setupProjectCards() {
    const cards = this.element.querySelectorAll('.project-card');
    cards.forEach(card => {
      card.addEventListener('mouseenter', () => {
        card.classList.add('project-card--hover');
      });
      card.addEventListener('mouseleave', () => {
        card.classList.remove('project-card--hover');
      });
    });
  }
  
  setupTechTags() {
    const techTags = this.element.querySelectorAll('.tech-tag');
    techTags.forEach(tag => {
      tag.addEventListener('click', () => {
        const tech = tag.textContent;
        const url = new URL(window.location);
        url.searchParams.set('tech', tech);
        window.location = url.toString();
      });
      tag.style.cursor = 'pointer';
    });
  }
}

// Initialize projects layout
document.addEventListener('DOMContentLoaded', () => {
  const projectsLayout = document.querySelector('.projects-layout');
  if (projectsLayout) {
    new ProjectsLayout(projectsLayout);
  }
});