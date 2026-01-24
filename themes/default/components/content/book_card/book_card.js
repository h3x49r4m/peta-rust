class BookCardComponent {
  constructor(element) {
    this.element = element;
    this.coverImage = element.querySelector('.book-card-cover-image');
    this.titleLink = element.querySelector('.book-card-title-link');
    this.actionButtons = element.querySelectorAll('.book-card-action');
    this.progressBar = element.querySelector('.book-card-progress-bar');
    this.size = element.dataset.size;
    this.variant = element.dataset.variant;
    
    this.state = {
      isLoaded: false,
      isLoading: false,
      progress: this.getInitialProgress()
    };
    
    this.init();
  }
  
  init() {
    this.setupEventListeners();
    this.setupImageLoading();
    this.setupProgressTracking();
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
    
    // Hover effects
    this.element.addEventListener('mouseenter', () => this.handleHover(true));
    this.element.addEventListener('mouseleave', () => this.handleHover(false));
    
    // Custom events
    this.element.addEventListener('book_update', (e) => this.handleBookUpdate(e));
    this.element.addEventListener('progress_update', (e) => this.handleProgressUpdate(e));
  }
  
  setupImageLoading() {
    if (!this.coverImage) return;
    
    if (this.coverImage.complete) {
      this.handleImageLoad();
    } else {
      this.coverImage.addEventListener('load', () => this.handleImageLoad());
      this.coverImage.addEventListener('error', () => this.handleImageError());
    }
  }
  
  setupProgressTracking() {
    if (!this.progressBar) return;
    
    // Animate initial progress
    setTimeout(() => {
      this.progressBar.style.width = `${this.state.progress}%`;
    }, 100);
  }
  
  handleTitleClick(event) {
    // Track book title clicks
    const title = this.titleLink.textContent.trim();
    const url = this.titleLink.href;
    
    this.element.dispatchEvent(new CustomEvent('book_title_click', {
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
    event.preventDefault();
    
    const action = button.classList.contains('book-card-action--primary') ? 'primary' : 'secondary';
    const title = this.element.querySelector('.book-card-title').textContent.trim();
    
    // Track action button clicks
    this.element.dispatchEvent(new CustomEvent('book_action_click', {
      detail: { action, title, button: button.textContent.trim() }
    }));
    
    // Add loading state
    button.classList.add('loading');
    button.disabled = true;
    
    // Simulate action
    setTimeout(() => {
      button.classList.remove('loading');
      button.disabled = false;
      
      // Navigate if it's a primary action with href
      if (action === 'primary' && button.href) {
        window.location.href = button.href;
      }
    }, 800);
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
    this.coverImage.style.animation = 'fadeIn 0.5s ease';
    
    this.element.dispatchEvent(new CustomEvent('book_image_loaded'));
  }
  
  handleImageError() {
    this.element.classList.add('image-error');
    
    // Create fallback
    const fallback = document.createElement('div');
    fallback.className = 'book-card-cover-fallback';
    fallback.innerHTML = `
      <div class="book-card-cover-fallback-icon">
        ${this.getBookIcon()}
      </div>
    `;
    
    if (this.coverImage.parentElement) {
      this.coverImage.parentElement.appendChild(fallback);
      this.coverImage.style.display = 'none';
    }
  }
  
  handleBookUpdate(event) {
    const { book } = event.detail;
    
    // Update book information
    this.updateBookInfo(book);
  }
  
  handleProgressUpdate(event) {
    const { progress } = event.detail;
    this.updateProgress(progress);
  }
  
  updateBookInfo(book) {
    // Update title
    const titleElement = this.element.querySelector('.book-card-title-link');
    if (titleElement && book.title) {
      titleElement.textContent = book.title;
      titleElement.href = book.url || '#';
    }
    
    // Update subtitle
    const subtitleElement = this.element.querySelector('.book-card-subtitle');
    if (subtitleElement && book.subtitle) {
      subtitleElement.textContent = book.subtitle;
    }
    
    // Update description
    const descriptionElement = this.element.querySelector('.book-card-description p');
    if (descriptionElement && book.description) {
      descriptionElement.textContent = this.truncateText(book.description, 150);
    }
    
    // Update progress if available
    if (book.progress !== undefined) {
      this.updateProgress(book.progress);
    }
  }
  
  updateProgress(percentage) {
    this.state.progress = Math.max(0, Math.min(100, percentage));
    
    if (this.progressBar) {
      this.progressBar.style.width = `${this.state.progress}%`;
    }
    
    // Update progress indicator in meta
    const progressElement = this.element.querySelector('.book-card-progress-indicator');
    if (progressElement) {
      progressElement.textContent = `${Math.round(this.state.progress)}% complete`;
    }
    
    // Dispatch progress event
    this.element.dispatchEvent(new CustomEvent('book_progress_changed', {
      detail: { progress: this.state.progress }
    }));
  }
  
  setLoading(isLoading) {
    this.state.isLoading = isLoading;
    
    if (isLoading) {
      this.element.classList.add('loading');
    } else {
      this.element.classList.remove('loading');
    }
  }
  
  getInitialProgress() {
    if (!this.progressBar) return 0;
    
    const width = this.progressBar.style.width || '0%';
    return parseInt(width.replace('%', '')) || 0;
  }
  
  truncateText(text, maxLength) {
    if (text.length <= maxLength) return text;
    return text.substring(0, maxLength).trim() + '...';
  }
  
  getBookIcon() {
    return `
      <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"></path>
        <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"></path>
      </svg>
    `;
  }
  
  // Public methods
  setProgress(percentage) {
    this.updateProgress(percentage);
  }
  
  getProgress() {
    return this.state.progress;
  }
  
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
    this.setupProgressTracking();
  }
}

// Initialize book_card components
window.PETA_COMPONENTS = window.PETA_COMPONENTS || {};
window.PETA_COMPONENTS.book_card = (element, props = {}) => {
  return new BookCardComponent(element);
};

// Auto-initialize
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('[data-component="book_card"]').forEach(element => {
    new BookCardComponent(element);
  });
});