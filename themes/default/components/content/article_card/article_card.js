class ArticleCardComponent {
  constructor(element) {
    this.element = element;
    this.category = element.dataset.category;
    this.state = {
      loading: false,
      bookmarked: false
    };
    
    this.init();
  }
  
  init() {
    // Click handler for entire card
    this.element.addEventListener('click', (e) => this.handleClick(e));
    
    // Setup bookmark functionality
    this.setupBookmark();
    
    // Setup reading progress
    this.setupReadingProgress();
    
    // Dispatch ready event
    this.element.dispatchEvent(new CustomEvent('article-card:ready', {
      detail: { category: this.category }
    }));
  }
  
  handleClick(event) {
    // Don't trigger card click if clicking on links or buttons inside
    if (event.target.closest('a, button')) {
      return;
    }
    
    const link = this.element.querySelector('.article-card-title a');
    if (link) {
      this.element.dispatchEvent(new CustomEvent('article-card:click', {
        detail: { 
          url: link.href,
          title: link.textContent,
          category: this.category
        }
      }));
    }
  }
  
  setupBookmark() {
    const bookmarkButton = this.element.querySelector('.article-card-bookmark');
    
    if (bookmarkButton) {
      bookmarkButton.addEventListener('click', (e) => {
        e.stopPropagation();
        this.toggleBookmark();
      });
      
      // Check bookmark state from localStorage
      const articleUrl = this.element.querySelector('.article-card-title a')?.href;
      if (articleUrl) {
        const bookmarks = JSON.parse(localStorage.getItem('bookmarks') || '[]');
        this.state.bookmarked = bookmarks.includes(articleUrl);
        this.updateBookmarkButton();
      }
    }
  }
  
  toggleBookmark() {
    const articleUrl = this.element.querySelector('.article-card-title a')?.href;
    if (!articleUrl) return;
    
    let bookmarks = JSON.parse(localStorage.getItem('bookmarks') || '[]');
    
    if (this.state.bookmarked) {
      // Remove bookmark
      bookmarks = bookmarks.filter(url => url !== articleUrl);
    } else {
      // Add bookmark
      bookmarks.push(articleUrl);
    }
    
    localStorage.setItem('bookmarks', JSON.stringify(bookmarks));
    this.state.bookmarked = !this.state.bookmarked;
    this.updateBookmarkButton();
    
    this.element.dispatchEvent(new CustomEvent('article-card:bookmark-changed', {
      detail: { 
        url: articleUrl,
        bookmarked: this.state.bookmarked
      }
    }));
  }
  
  updateBookmarkButton() {
    const bookmarkButton = this.element.querySelector('.article-card-bookmark');
    if (bookmarkButton) {
      if (this.state.bookmarked) {
        bookmarkButton.classList.add('bookmarked');
        bookmarkButton.setAttribute('aria-label', 'Remove bookmark');
      } else {
        bookmarkButton.classList.remove('bookmarked');
        bookmarkButton.setAttribute('aria-label', 'Add bookmark');
      }
    }
  }
  
  setupReadingProgress() {
    // Track reading progress for this article
    const articleUrl = this.element.querySelector('.article-card-title a')?.href;
    if (!articleUrl) return;
    
    const readingProgress = JSON.parse(localStorage.getItem('reading-progress') || '{}');
    const progress = readingProgress[articleUrl] || 0;
    
    if (progress > 0) {
      this.addProgressIndicator(progress);
    }
  }
  
  addProgressIndicator(progress) {
    const indicator = document.createElement('div');
    indicator.className = 'article-card-progress';
    indicator.innerHTML = `
      <div class="progress-bar">
        <div class="progress-fill" style="width: ${progress}%"></div>
      </div>
      <span class="progress-text">${Math.round(progress)}% read</span>
    `;
    
    this.element.appendChild(indicator);
  }
  
  // Public API
  setLoading(loading) {
    this.state.loading = loading;
    if (loading) {
      this.element.classList.add('article-card-loading');
      this.element.style.pointerEvents = 'none';
    } else {
      this.element.classList.remove('article-card-loading');
      this.element.style.pointerEvents = '';
    }
    
    this.element.dispatchEvent(new CustomEvent('article-card:loading', {
      detail: { loading }
    }));
  }
  
  setCategory(category) {
    this.category = category;
    this.element.dataset.category = category;
    
    // Update category badge
    const categoryBadge = this.element.querySelector('.article-card-category');
    if (categoryBadge) {
      categoryBadge.textContent = category;
    }
  }
  
  updateReadingProgress(progress) {
    const articleUrl = this.element.querySelector('.article-card-title a')?.href;
    if (!articleUrl) return;
    
    const readingProgress = JSON.parse(localStorage.getItem('reading-progress') || '{}');
    readingProgress[articleUrl] = progress;
    localStorage.setItem('reading-progress', JSON.stringify(readingProgress));
    
    // Update or add progress indicator
    let indicator = this.element.querySelector('.article-card-progress');
    if (!indicator && progress > 0) {
      this.addProgressIndicator(progress);
    } else if (indicator) {
      const fill = indicator.querySelector('.progress-fill');
      const text = indicator.querySelector('.progress-text');
      fill.style.width = `${progress}%`;
      text.textContent = `${Math.round(progress)}% read`;
    }
  }
  
  getCategory() {
    return this.category;
  }
  
  isBookmarked() {
    return this.state.bookmarked;
  }
  
  getTitle() {
    return this.element.querySelector('.article-card-title')?.textContent?.trim();
  }
  
  getUrl() {
    return this.element.querySelector('.article-card-title a')?.href;
  }
}

// Auto-initialize article cards
document.addEventListener('DOMContentLoaded', () => {
  const articleCards = document.querySelectorAll('.article-card');
  articleCards.forEach(element => {
    new ArticleCardComponent(element);
  });
});

// Export for manual initialization
window.ArticleCardComponent = ArticleCardComponent;