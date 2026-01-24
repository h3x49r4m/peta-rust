class ArticleRelatedComponent {
  constructor(element) {
    this.element = element;
    this.layout = element.dataset.layout;
    this.columns = parseInt(element.dataset.columns) || 2;
    this.carousel = element.querySelector('[data-carousel]');
    this.carouselTrack = element.querySelector('[data-carousel-track]');
    this.carouselList = element.querySelector('[data-carousel-list]');
    this.prevBtn = element.querySelector('[data-carousel-prev]');
    this.nextBtn = element.querySelector('[data-carousel-next]');
    this.moreBtn = element.querySelector('[data-more-btn]');
    this.grid = element.querySelector('[data-grid]');
    
    this.state = {
      currentIndex: 0,
      isAnimating: false,
      touchStartX: 0,
      touchEndX: 0,
      isLoading: false
    };
    
    this.init();
  }
  
  init() {
    if (this.layout === 'carousel') {
      this.setupCarousel();
    }
    
    this.setupEventListeners();
    this.setupTouchSupport();
    this.setupLoadingStates();
  }
  
  setupCarousel() {
    if (!this.carousel || !this.carouselList) return;
    
    const items = this.carouselList.querySelectorAll('.article-related-carousel-item');
    if (items.length === 0) return;
    
    this.state.totalItems = items.length;
    this.updateCarouselButtons();
  }
  
  setupEventListeners() {
    // Carousel navigation
    if (this.prevBtn) {
      this.prevBtn.addEventListener('click', () => this.navigateCarousel('prev'));
    }
    
    if (this.nextBtn) {
      this.nextBtn.addEventListener('click', () => this.navigateCarousel('next'));
    }
    
    // More button
    if (this.moreBtn) {
      this.moreBtn.addEventListener('click', () => this.handleMoreClick());
    }
    
    // Keyboard navigation
    this.element.addEventListener('keydown', (e) => this.handleKeydown(e));
    
    // Window resize
    window.addEventListener('resize', () => this.handleResize());
    
    // Custom events
    this.element.addEventListener('related_update', (e) => this.handleRelatedUpdate(e));
  }
  
  setupTouchSupport() {
    if (!this.carousel) return;
    
    this.carousel.addEventListener('touchstart', (e) => this.handleTouchStart(e), { passive: true });
    this.carousel.addEventListener('touchmove', (e) => this.handleTouchMove(e), { passive: true });
    this.carousel.addEventListener('touchend', (e) => this.handleTouchEnd(e));
  }
  
  setupLoadingStates() {
    // Add loading state to related articles
    this.element.addEventListener('loading', () => {
      this.state.isLoading = true;
      this.element.classList.add('loading');
    });
    
    this.element.addEventListener('loaded', () => {
      this.state.isLoading = false;
      this.element.classList.remove('loading');
    });
  }
  
  navigateCarousel(direction) {
    if (this.state.isAnimating || !this.carouselList) return;
    
    const items = this.carouselList.querySelectorAll('.article-related-carousel-item');
    if (items.length === 0) return;
    
    this.state.isAnimating = true;
    
    if (direction === 'prev') {
      this.state.currentIndex = Math.max(0, this.state.currentIndex - 1);
    } else {
      this.state.currentIndex = Math.min(items.length - 1, this.state.currentIndex + 1);
    }
    
    this.updateCarouselPosition();
    this.updateCarouselButtons();
    
    setTimeout(() => {
      this.state.isAnimating = false;
    }, 300);
  }
  
  updateCarouselPosition() {
    if (!this.carouselList) return;
    
    const itemWidth = this.carouselList.querySelector('.article-related-carousel-item').offsetWidth;
    const gap = 24; // CSS gap value
    const offset = this.state.currentIndex * (itemWidth + gap);
    
    this.carouselList.style.transform = `translateX(-${offset}px)`;
    this.carouselList.dataset.sliding = 'true';
    
    setTimeout(() => {
      delete this.carouselList.dataset.sliding;
    }, 500);
  }
  
  updateCarouselButtons() {
    if (!this.prevBtn || !this.nextBtn) return;
    
    const items = this.carouselList.querySelectorAll('.article-related-carousel-item');
    const maxIndex = Math.max(0, items.length - 1);
    
    this.prevBtn.disabled = this.state.currentIndex === 0;
    this.nextBtn.disabled = this.state.currentIndex >= maxIndex;
  }
  
  handleKeydown(event) {
    if (this.layout !== 'carousel') return;
    
    switch (event.key) {
      case 'ArrowLeft':
        event.preventDefault();
        this.navigateCarousel('prev');
        break;
      case 'ArrowRight':
        event.preventDefault();
        this.navigateCarousel('next');
        break;
    }
  }
  
  handleResize() {
    if (this.layout === 'carousel') {
      this.updateCarouselPosition();
      this.updateCarouselButtons();
    }
  }
  
  handleTouchStart(event) {
    this.state.touchStartX = event.touches[0].clientX;
    this.carousel.dataset.touch = 'true';
  }
  
  handleTouchMove(event) {
    if (!this.state.touchStartX) return;
    
    this.state.touchEndX = event.touches[0].clientX;
  }
  
  handleTouchEnd(event) {
    if (!this.state.touchStartX || !this.state.touchEndX) return;
    
    const diff = this.state.touchStartX - this.state.touchEndX;
    const threshold = 50;
    
    if (diff > threshold) {
      this.navigateCarousel('next');
    } else if (diff < -threshold) {
      this.navigateCarousel('prev');
    }
    
    this.state.touchStartX = 0;
    this.state.touchEndX = 0;
    delete this.carousel.dataset.touch;
  }
  
  handleMoreClick() {
    // Track more button clicks
    this.element.dispatchEvent(new CustomEvent('related_more_click', {
      detail: { action: 'view_all' }
    }));
    
    // Add loading state
    this.moreBtn.classList.add('loading');
    this.moreBtn.disabled = true;
    
    // Simulate loading more articles
    setTimeout(() => {
      this.moreBtn.classList.remove('loading');
      this.moreBtn.disabled = false;
    }, 1000);
  }
  
  handleRelatedUpdate(event) {
    const { related_articles } = event.detail;
    
    // Update the component with new articles
    this.updateArticles(related_articles);
  }
  
  updateArticles(articles) {
    this.element.dispatchEvent(new CustomEvent('loading'));
    
    // Simulate updating articles
    setTimeout(() => {
      // In a real implementation, this would update the DOM
      // with new article cards based on the articles array
      
      if (this.layout === 'carousel') {
        this.state.currentIndex = 0;
        this.setupCarousel();
      }
      
      this.element.dispatchEvent(new CustomEvent('loaded'));
      this.element.dispatchEvent(new CustomEvent('articles_updated', {
        detail: { articles }
      }));
    }, 500);
  }
  
  // Public methods
  setLayout(layout) {
    this.layout = layout;
    this.element.dataset.layout = layout;
    
    // Re-initialize based on new layout
    if (layout === 'carousel') {
      this.setupCarousel();
    }
  }
  
  setColumns(columns) {
    this.columns = columns;
    this.element.dataset.columns = columns;
    
    if (this.grid) {
      this.grid.style.gridTemplateColumns = `repeat(${columns}, 1fr)`;
    }
  }
  
  nextSlide() {
    if (this.layout === 'carousel') {
      this.navigateCarousel('next');
    }
  }
  
  prevSlide() {
    if (this.layout === 'carousel') {
      this.navigateCarousel('prev');
    }
  }
  
  goToSlide(index) {
    if (this.layout === 'carousel' && !this.state.isAnimating) {
      const items = this.carouselList.querySelectorAll('.article-related-carousel-item');
      this.state.currentIndex = Math.max(0, Math.min(index, items.length - 1));
      this.updateCarouselPosition();
      this.updateCarouselButtons();
    }
  }
  
  refresh() {
    // Re-initialize the component
    if (this.layout === 'carousel') {
      this.setupCarousel();
    }
  }
}

// Initialize article_related components
window.PETA_COMPONENTS = window.PETA_COMPONENTS || {};
window.PETA_COMPONENTS.article_related = (element, props = {}) => {
  return new ArticleRelatedComponent(element);
};

// Auto-initialize
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('[data-component="article_related"]').forEach(element => {
    new ArticleRelatedComponent(element);
  });
});