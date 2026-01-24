class ProjectCardComponent {
  constructor(element) {
    this.element = element;
    this.size = element.dataset.size;
    this.variant = element.dataset.variant;
    this.image = element.querySelector('.project-card-image-img');
    this.titleLink = element.querySelector('.project-card-title-link');
    this.actionButtons = element.querySelectorAll('.project-card-action');
    this.overlayLink = element.querySelector('.project-card-overlay-link');
    
    this.state = {
      isLoaded: false,
      isLoading: false
    };
    
    this.init();
  }
  
  init() {
    this.setupEventListeners();
    this.setupImageLoading();
  }
  
  setupEventListeners() {
    // Title link clicks
    if (this.titleLink) {
      this.titleLink.addEventListener('click', (e) => this.handleTitleClick(e));
    }
    
    // Action button clicks
    this.actionButtons.forEach(button => {
      button.addEventListener('click', (e) => this.handleActionClick(e, button));
    });
    
    // Overlay link clicks
    if (this.overlayLink) {
      this.overlayLink.addEventListener('click', (e) => this.handleOverlayClick(e));
    }
    
    // Hover effects
    this.element.addEventListener('mouseenter', () => this.handleHover(true));
    this.element.addEventListener('mouseleave', () => this.handleHover(false));
    
    // Custom events
    this.element.addEventListener('project_update', (e) => this.handleProjectUpdate(e));
  }
  
  setupImageLoading() {
    if (!this.image) return;
    
    if (this.image.complete) {
      this.handleImageLoad();
    } else {
      this.image.addEventListener('load', () => this.handleImageLoad());
      this.image.addEventListener('error', () => this.handleImageError());
    }
  }
  
  handleTitleClick(event) {
    // Track project title clicks
    const title = this.titleLink.textContent.trim();
    const url = this.titleLink.href;
    
    this.element.dispatchEvent(new CustomEvent('project_title_click', {
      detail: { title, url }
    }));
    
    // Add loading state
    this.setLoading(true);
    
    // Remove loading state after navigation
    setTimeout(() => {
      this.setLoading(false);
    }, 500);
  }
  
  handleActionClick(event, button) {
    const action = button.classList.contains('project-card-action--primary') ? 'primary' : 'secondary';
    const title = this.element.querySelector('.project-card-title').textContent.trim();
    
    // Track action button clicks
    this.element.dispatchEvent(new CustomEvent('project_action_click', {
      detail: { 
        action, 
        title, 
        button: button.textContent.trim(),
        url: button.href
      }
    }));
    
    // Add loading state
    button.classList.add('loading');
    button.disabled = true;
    
    // Simulate action
    setTimeout(() => {
      button.classList.remove('loading');
      button.disabled = false;
      
      // Navigate if it's an external link
      if (button.href) {
        window.open(button.href, '_blank', 'noopener,noreferrer');
      }
    }, 800);
  }
  
  handleOverlayClick(event) {
    const title = this.element.querySelector('.project-card-title').textContent.trim();
    const url = this.overlayLink.href;
    
    // Track overlay clicks
    this.element.dispatchEvent(new CustomEvent('project_overlay_click', {
      detail: { title, url }
    }));
    
    // Add loading state
    this.overlayLink.classList.add('loading');
    
    // Remove loading state after navigation
    setTimeout(() => {
      this.overlayLink.classList.remove('loading');
    }, 500);
  }
  
  handleHover(isHovering) {
    if (isHovering) {
      this.element.style.transform = 'translateY(-4px) scale(1.02)';
    } else {
      this.element.style.transform = '';
    }
  }
  
  handleImageLoad() {
    this.state.isLoaded = true;
    this.element.classList.add('loaded');
    
    // Add subtle animation
    this.image.style.animation = 'fadeIn 0.5s ease';
    
    this.element.dispatchEvent(new CustomEvent('project_image_loaded'));
  }
  
  handleImageError() {
    this.element.classList.add('image-error');
    
    // Create fallback
    const fallback = document.createElement('div');
    fallback.className = 'project-card-image-fallback';
    fallback.innerHTML = `
      <div class="project-card-image-fallback-icon">
        ${this.getProjectIcon()}
      </div>
      <div class="project-card-image-fallback-text">No Preview</div>
    `;
    
    if (this.image.parentElement) {
      this.image.parentElement.appendChild(fallback);
      this.image.style.display = 'none';
    }
  }
  
  handleProjectUpdate(event) {
    const { project } = event.detail;
    this.updateProjectInfo(project);
  }
  
  updateProjectInfo(project) {
    // Update title
    const titleElement = this.element.querySelector('.project-card-title-link');
    if (titleElement && project.title) {
      titleElement.textContent = project.title;
      titleElement.href = project.url || '#';
    }
    
    // Update subtitle
    const subtitleElement = this.element.querySelector('.project-card-subtitle');
    if (subtitleElement && project.subtitle) {
      subtitleElement.textContent = project.subtitle;
    }
    
    // Update description
    const descriptionElement = this.element.querySelector('.project-card-description p');
    if (descriptionElement && project.description) {
      descriptionElement.textContent = this.truncateText(project.description, 150);
    }
    
    // Update status
    const statusElement = this.element.querySelector('.project-card-status-badge');
    if (statusElement && project.status) {
      statusElement.textContent = project.status;
      statusElement.className = `project-card-status-badge project-card-status-badge--${project.status}`;
    }
    
    // Update stats
    this.updateStats(project);
    
    // Update tags
    this.updateTags(project);
  }
  
  updateStats(project) {
    // Update stars
    const starsElement = this.element.querySelector('.project-card-stat');
    if (starsElement && project.stars) {
      starsElement.innerHTML = `
        {% component "icon" with name="star" library="feather" size="xs" %}
        {% endslot %}
        ${project.stars}
      `;
    }
    
    // Update forks
    const forksElement = this.element.querySelectorAll('.project-card-stat')[1];
    if (forksElement && project.forks) {
      forksElement.innerHTML = `
        {% component "icon" with name="git-branch" library="feather" size="xs" %}
        {% endslot %}
        ${project.forks}
      `;
    }
    
    // Update language
    const languageElements = this.element.querySelectorAll('.project-card-stat');
    languageElements.forEach(element => {
      const icon = element.querySelector('svg');
      if (icon && icon.getAttribute('data-name') === 'code' && project.language) {
        element.lastChild.textContent = project.language;
      }
    });
  }
  
  updateTags(project) {
    const tagsContainer = this.element.querySelector('.project-card-tags');
    if (tagsContainer && project.tags) {
      tagsContainer.innerHTML = '';
      
      project.tags.slice(0, 3).forEach(tag => {
        const tagElement = document.createElement('span');
        tagElement.className = 'project-card-tag';
        tagElement.textContent = tag;
        tagsContainer.appendChild(tagElement);
      });
      
      if (project.tags.length > 3) {
        const moreElement = document.createElement('span');
        moreElement.className = 'project-card-tag project-card-tag--more';
        moreElement.textContent = `+${project.tags.length - 3}`;
        tagsContainer.appendChild(moreElement);
      }
    }
  }
  
  setLoading(isLoading) {
    this.state.isLoading = isLoading;
    
    if (isLoading) {
      this.element.classList.add('loading');
    } else {
      this.element.classList.remove('loading');
    }
  }
  
  truncateText(text, maxLength) {
    if (text.length <= maxLength) return text;
    return text.substring(0, maxLength).trim() + '...';
  }
  
  getProjectIcon() {
    return `
      <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <rect x="2" y="3" width="20" height="14" rx="2" ry="2"></rect>
        <line x1="8" y1="21" x2="16" y2="21"></line>
        <line x1="12" y1="17" x2="12" y2="21"></line>
      </svg>
    `;
  }
  
  // Public methods
  setSize(size) {
    this.size = size;
    this.element.dataset.size = size;
  }
  
  setVariant(variant) {
    this.variant = variant;
    this.element.dataset.variant = variant;
  }
  
  refresh() {
    // Re-initialize the component
    this.setupImageLoading();
  }
}

// Initialize project_card components
window.PETA_COMPONENTS = window.PETA_COMPONENTS || {};
window.PETA_COMPONENTS.project_card = (element, props = {}) => {
  return new ProjectCardComponent(element);
};

// Auto-initialize
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('[data-component="project_card"]').forEach(element => {
    new ProjectCardComponent(element);
  });
});