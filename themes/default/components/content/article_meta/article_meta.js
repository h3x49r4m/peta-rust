class ArticleMetaComponent {
  constructor(element) {
    this.element = element;
    this.dateElement = element.querySelector('.article-meta-date');
    this.tagsContainer = element.querySelector('.article-meta-tags-list');
    this.readingTimeElement = element.querySelector('.article-meta-reading-time');
    
    this.init();
  }
  
  init() {
    this.setupEventListeners();
    this.initializeReadingTime();
    this.setupTagInteractions();
  }
  
  setupEventListeners() {
    // Add click tracking for metadata
    this.element.addEventListener('click', (e) => {
      if (e.target.closest('.article-meta-date')) {
        this.trackEvent('date_click');
      } else if (e.target.closest('.article-meta-author')) {
        this.trackEvent('author_click');
      }
    });
    
    // Add hover effects for tags
    const tags = this.element.querySelectorAll('.article-meta-tags .badge');
    tags.forEach(tag => {
      tag.addEventListener('mouseenter', () => this.handleTagHover(tag, true));
      tag.addEventListener('mouseleave', () => this.handleTagHover(tag, false));
    });
  }
  
  initializeReadingTime() {
    if (!this.readingTimeElement) return;
    
    const readingTime = parseInt(this.readingTimeElement.textContent);
    if (readingTime > 0) {
      // Add reading progress indicator
      this.readingTimeElement.setAttribute('title', `${readingTime} minute${readingTime > 1 ? 's' : ''} read`);
      
      // Estimate reading progress based on scroll
      this.setupReadingProgress();
    }
  }
  
  setupReadingProgress() {
    let startTime = Date.now();
    let hasStartedReading = false;
    
    const observer = new IntersectionObserver((entries) => {
      entries.forEach(entry => {
        if (entry.isIntersecting && !hasStartedReading) {
          hasStartedReading = true;
          startTime = Date.now();
        }
      });
    });
    
    // Observe the main content area
    const content = document.querySelector('article');
    if (content) {
      observer.observe(content);
    }
    
    // Track reading time
    window.addEventListener('beforeunload', () => {
      if (hasStartedReading) {
        const readingDuration = Math.floor((Date.now() - startTime) / 1000);
        this.trackEvent('reading_time', { duration: readingDuration });
      }
    });
  }
  
  setupTagInteractions() {
    if (!this.tagsContainer) return;
    
    const tags = this.tagsContainer.querySelectorAll('.badge');
    tags.forEach(tag => {
      tag.addEventListener('click', (e) => {
        e.preventDefault();
        const tagName = this.extractTagName(tag);
        this.trackEvent('tag_click', { tag: tagName });
        
        // Add visual feedback
        this.animateTagClick(tag);
      });
    });
  }
  
  handleTagHover(tag, isHovering) {
    if (isHovering) {
      tag.style.transform = 'translateY(-1px) scale(1.05)';
    } else {
      tag.style.transform = '';
    }
  }
  
  extractTagName(tagElement) {
    const link = tagElement.querySelector('a');
    return link ? link.textContent.trim() : '';
  }
  
  animateTagClick(tag) {
    tag.style.transform = 'scale(0.95)';
    tag.style.opacity = '0.7';
    
    setTimeout(() => {
      tag.style.transform = '';
      tag.style.opacity = '';
    }, 150);
  }
  
  trackEvent(action, data = {}) {
    // Dispatch custom event for analytics
    this.element.dispatchEvent(new CustomEvent('article_meta_interaction', {
      detail: { action, ...data }
    }));
  }
  
  // Public methods
  updateReadingTime(minutes) {
    if (!this.readingTimeElement) return;
    
    this.readingTimeElement.innerHTML = `
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="12" cy="12" r="10"></circle>
        <polyline points="12 6 12 12 12 18"></polyline>
      </svg>
      ${minutes} min read
    `;
    
    this.readingTimeElement.setAttribute('title', `${minutes} minute${minutes > 1 ? 's' : ''} read`);
  }
  
  updateDate(dateString, format = "MMM DD, YYYY") {
    if (!this.dateElement) return;
    
    const formattedDate = this.formatDate(dateString, format);
    this.dateElement.innerHTML = `
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect>
        <line x1="16" y1="2" x2="16" y2="6"></line>
        <line x1="8" y1="2" x2="8" y2="6"></line>
        <line x1="3" y1="10" x2="21" y2="10"></line>
      </svg>
      ${formattedDate}
    `;
    
    this.dateElement.setAttribute('datetime', dateString);
  }
  
  formatDate(dateString, format) {
    try {
      const date = new Date(dateString);
      return date.toLocaleDateString('en-US', { 
        year: 'numeric', 
        month: 'short', 
        day: 'numeric' 
      });
    } catch (error) {
      return dateString;
    }
  }
  
  updateAuthor(author) {
    const authorElement = this.element.querySelector('.article-meta-author');
    if (authorElement) {
      authorElement.innerHTML = `
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
          <circle cx="12" cy="13" r="4"></circle>
        </svg>
        by ${author}
      `;
    }
  }
  
  updateTags(tags) {
    if (!this.tagsContainer) return;
    
    this.tagsContainer.innerHTML = tags.map(tag => `
      <span class="badge badge-outline badge-sm">
        <a href="/tags/${tag}">${tag}</a>
      </span>
    `).join('');
    
    // Reinitialize tag interactions
    this.setupTagInteractions();
  }
}

// Initialize article_meta components
window.PETA_COMPONENTS = window.PETA_COMPONENTS || {};
window.PETA_COMPONENTS.article_meta = (element, props = {}) => {
  return new ArticleMetaComponent(element);
};

// Auto-initialize
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('[data-component="article_meta"]').forEach(element => {
    new ArticleMetaComponent(element);
  });
});