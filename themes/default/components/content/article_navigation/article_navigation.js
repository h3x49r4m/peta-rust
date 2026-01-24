class ArticleNavigationComponent {
  constructor(element) {
    this.element = element;
    this.prevLink = element.querySelector('.article-navigation-link--prev');
    this.nextLink = element.querySelector('.article-navigation-link--next');
    this.progressBar = element.querySelector('[data-progress-bar]');
    this.progressText = element.querySelector('.article-navigation-progress-text');
    this.style = element.dataset.style;
    
    this.state = {
      scrollProgress: 0,
      isReading: false
    };
    
    this.init();
  }
  
  init() {
    this.setupEventListeners();
    this.setupScrollTracking();
    this.setupKeyboardNavigation();
  }
  
  setupEventListeners() {
    // Link click events
    [this.prevLink, this.nextLink].forEach(link => {
      if (link) {
        link.addEventListener('click', (e) => this.handleLinkClick(e, link));
        link.addEventListener('mouseenter', () => this.handleLinkHover(link, true));
        link.addEventListener('mouseleave', () => this.handleLinkHover(link, false));
      }
    });
    
    // Custom events
    this.element.addEventListener('navigation_update', (e) => this.handleNavigationUpdate(e));
    this.element.addEventListener('progress_update', (e) => this.handleProgressUpdate(e));
  }
  
  setupScrollTracking() {
    let scrollTimeout;
    
    const handleScroll = () => {
      const article = document.querySelector('article');
      if (!article) return;
      
      const articleHeight = article.offsetHeight;
      const articleTop = article.offsetTop;
      const windowHeight = window.innerHeight;
      const scrolled = window.scrollY;
      
      // Calculate reading progress
      let progress = 0;
      
      if (scrolled + windowHeight >= articleTop + articleHeight) {
        // Article fully read
        progress = 100;
        this.state.isReading = false;
      } else if (scrolled > articleTop) {
        // Currently reading
        progress = Math.min(((scrolled - articleTop) / articleHeight) * 100, 99);
        this.state.isReading = true;
      }
      
      this.updateProgress(progress);
      
      // Debounce scroll events
      clearTimeout(scrollTimeout);
      scrollTimeout = setTimeout(() => {
        if (progress === 100) {
          this.state.isReading = false;
        }
      }, 1000);
    };
    
    window.addEventListener('scroll', handleScroll);
    
    // Initial progress calculation
    handleScroll();
  }
  
  setupKeyboardNavigation() {
    document.addEventListener('keydown', (e) => {
      switch (e.key) {
        case 'ArrowLeft':
          if (this.prevLink && !e.target.matches('input, textarea')) {
            e.preventDefault();
          this.prevLink.click();
        }
          break;
          
        case 'ArrowRight':
          if (this.nextLink && !e.target.matches('input, textarea')) {
            e.preventDefault();
          this.nextLink.click();
        }
          break;
          
        case 'Home':
          if (!e.target.matches('input, textarea')) {
            e.preventDefault();
          window.scrollTo({ top: 0, behavior: 'smooth' });
          }
          break;
      }
    });
  }
  
  handleLinkClick(event, link) {
    // Add loading state
    link.classList.add('loading');
    
    // Track navigation event
    const direction = link.dataset.direction;
    const title = link.querySelector('.article-navigation-text').textContent.trim();
    
    this.element.dispatchEvent(new CustomEvent('navigation_click', {
      detail: { direction, title, url: link.href }
    }));
    
    // Remove loading state after navigation
    setTimeout(() => {
      link.classList.remove('loading');
    }, 500);
  }
  
  handleLinkHover(link, isHovering) {
    if (isHovering) {
      link.style.transform = 'translateY(-2px) scale(1.02)';
    } else {
      link.style.transform = '';
    }
  }
  
  handleNavigationUpdate(event) {
    const { previous_article, next_article } = event.detail;
    
    // Update previous link
    if (previous_article && this.prevLink) {
      this.prevLink.href = previous_article.url;
      const prevText = this.prevLink.querySelector('.article-navigation-text');
      if (prevText) {
        prevText.textContent = previous_article.title || 'Previous';
      }
      this.prevLink.disabled = !previous_article.url;
    }
    
    // Update next link
    if (next_article && this.nextLink) {
      this.nextLink.href = next_article.url;
      const nextText = this.nextLink.querySelector('.article-navigation-text');
      if (nextText) {
        nextText.textContent = next_article.title || 'Next';
      }
      this.nextLink.disabled = !next_article.url;
    }
  }
  
  handleProgressUpdate(event) {
    const { progress } = event.detail;
    this.updateProgress(progress);
  }
  
  updateProgress(percentage) {
    this.state.scrollProgress = percentage;
    
    if (this.progressBar) {
      this.progressBar.style.width = `${percentage}%`;
    }
    
    if (this.progressText) {
      if (percentage === 100) {
        this.progressText.textContent = 'Article completed';
      } else if (percentage > 0) {
        this.progressText.textContent = `${Math.round(percentage)}% read`;
      } else {
        this.progressText.textContent = 'Start reading';
      }
    }
    
    // Dispatch progress event
    this.element.dispatchEvent(new CustomEvent('progress_changed', {
      detail: { progress: percentage }
    }));
  }
  
  // Public methods
  updateNavigation(previousArticle, nextArticle) {
    this.element.dispatchEvent(new CustomEvent('navigation_update', {
      detail: { previous_article, next_article }
    }));
  }
  
  setStyle(style) {
    this.style = style;
    this.element.dataset.style = style;
    
    // Update style class
    this.element.className = `article-navigation article-navigation--${style}`;
  }
  
  setLoading(direction, loading = true) {
    const link = direction === 'prev' ? this.prevLink : this.nextLink;
    if (link) {
      if (loading) {
        link.classList.add('loading');
      } else {
        link.classList.remove('loading');
      }
    }
  }
  
  getProgress() {
    return this.state.scrollProgress;
  }
  
  isReading() {
    return this.state.isReading;
  }
}

// Initialize article_navigation components
window.PETA_COMPONENTS = window.PETA_COMPONENTS || {};
window.PETA_COMPONENTS.article_navigation = (element, props = {}) => {
  return new ArticleNavigationComponent(element);
};

// Auto-initialize
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('[data-component="article_navigation"]').forEach(element => {
    new ArticleNavigationComponent(element);
  });
});