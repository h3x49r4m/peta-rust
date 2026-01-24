class ChapterNavigationComponent {
  constructor(element) {
    this.element = element;
    this.layout = element.dataset.layout;
    this.collapsible = element.dataset.collapsible === 'true';
    this.content = element.querySelector('[data-content]');
    this.toggle = element.querySelector('[data-toggle]');
    this.list = element.querySelector('[data-list]');
    this.links = element.querySelectorAll('[data-chapter-link]');
    this.sectionLinks = element.querySelectorAll('[data-section-link]');
    this.prevBtn = element.querySelector('.chapter-navigation-nav-btn--prev');
    this.nextBtn = element.querySelector('.chapter-navigation-nav-btn--next');
    this.progressBar = element.querySelector('.chapter-navigation-progress-fill');
    
    this.state = {
      currentChapter: this.getCurrentChapter(),
      isCollapsed: false,
      isAnimating: false
    };
    
    this.init();
  }
  
  init() {
    this.setupEventListeners();
    this.setupKeyboardNavigation();
    this.updateActiveChapter();
    this.updateProgress();
  }
  
  setupEventListeners() {
    // Toggle collapsible state
    if (this.toggle && this.collapsible) {
      this.toggle.addEventListener('click', () => this.toggleCollapsed());
    }
    
    // Chapter link clicks
    this.links.forEach(link => {
      link.addEventListener('click', (e) => this.handleChapterClick(e, link));
    });
    
    // Section link clicks
    this.sectionLinks.forEach(link => {
      link.addEventListener('click', (e) => this.handleSectionClick(e, link));
    });
    
    // Previous/Next navigation
    if (this.prevBtn) {
      this.prevBtn.addEventListener('click', (e) => this.handleNavClick(e, 'prev'));
    }
    
    if (this.nextBtn) {
      this.nextBtn.addEventListener('click', (e) => this.handleNavClick(e, 'next'));
    }
    
    // Custom events
    this.element.addEventListener('chapter_update', (e) => this.handleChapterUpdate(e));
    this.element.addEventListener('progress_update', (e) => this.handleProgressUpdate(e));
  }
  
  setupKeyboardNavigation() {
    this.element.addEventListener('keydown', (e) => {
      switch (e.key) {
        case 'ArrowUp':
          e.preventDefault();
          this.navigateChapter('prev');
          break;
        case 'ArrowDown':
          e.preventDefault();
          this.navigateChapter('next');
          break;
        case 'Home':
          e.preventDefault();
          this.goToChapter(0);
          break;
        case 'End':
          e.preventDefault();
          this.goToChapter(this.links.length - 1);
          break;
        case 'Enter':
        case ' ':
          if (e.target === this.element || e.target.classList.contains('chapter-navigation-title')) {
            e.preventDefault();
            if (this.collapsible) {
              this.toggleCollapsed();
            }
          }
          break;
      }
    });
  }
  
  toggleCollapsed() {
    if (this.state.isAnimating) return;
    
    this.state.isAnimating = true;
    this.state.isCollapsed = !this.state.isCollapsed;
    
    if (this.state.isCollapsed) {
      this.element.dataset.collapsed = 'true';
      this.content.style.display = 'none';
    } else {
      delete this.element.dataset.collapsed;
      this.content.style.display = 'flex';
    }
    
    // Animate toggle icon
    if (this.toggle) {
      this.toggle.style.transform = this.state.isCollapsed ? 'rotate(-90deg)' : '';
    }
    
    setTimeout(() => {
      this.state.isAnimating = false;
    }, 200);
    
    // Dispatch toggle event
    this.element.dispatchEvent(new CustomEvent('navigation_toggle', {
      detail: { collapsed: this.state.isCollapsed }
    }));
  }
  
  handleChapterClick(event, link) {
    const chapterIndex = parseInt(link.closest('[data-chapter]').dataset.chapter);
    const chapterTitle = link.querySelector('.chapter-navigation-title').textContent.trim();
    
    // Track chapter click
    this.element.dispatchEvent(new CustomEvent('chapter_click', {
      detail: { 
        index: chapterIndex, 
        title: chapterTitle,
        url: link.href
      }
    }));
    
    // Add loading state
    link.classList.add('loading');
    
    // Update current chapter
    this.setCurrentChapter(chapterIndex);
    
    // Remove loading state after navigation
    setTimeout(() => {
      link.classList.remove('loading');
    }, 500);
  }
  
  handleSectionClick(event, link) {
    const sectionTitle = link.querySelector('.chapter-navigation-section-title').textContent.trim();
    
    // Track section click
    this.element.dispatchEvent(new CustomEvent('section_click', {
      detail: { 
        title: sectionTitle,
        url: link.href
      }
    }));
    
    // Add loading state
    link.classList.add('loading');
    
    // Remove loading state after navigation
    setTimeout(() => {
      link.classList.remove('loading');
    }, 500);
  }
  
  handleNavClick(event, direction) {
    event.preventDefault();
    this.navigateChapter(direction);
  }
  
  handleChapterUpdate(event) {
    const { currentChapter } = event.detail;
    this.setCurrentChapter(currentChapter);
  }
  
  handleProgressUpdate(event) {
    const { progress } = event.detail;
    this.updateProgress(progress);
  }
  
  navigateChapter(direction) {
    const newIndex = direction === 'prev' 
      ? Math.max(0, this.state.currentChapter - 1)
      : Math.min(this.links.length - 1, this.state.currentChapter + 1);
    
    this.goToChapter(newIndex);
  }
  
  goToChapter(index) {
    if (index < 0 || index >= this.links.length) return;
    
    const link = this.links[index];
    if (link && link.href) {
      // Add loading state
      link.classList.add('loading');
      
      // Navigate to chapter
      window.location.href = link.href;
    }
  }
  
  setCurrentChapter(index) {
    this.state.currentChapter = index;
    this.updateActiveChapter();
    this.updateProgress();
    this.updateNavigationButtons();
  }
  
  updateActiveChapter() {
    // Remove active class from all items
    this.links.forEach(link => {
      link.closest('.chapter-navigation-item').classList.remove('chapter-navigation-item--active');
    });
    
    // Add active class to current chapter
    if (this.links[this.state.currentChapter]) {
      this.links[this.state.currentChapter]
        .closest('.chapter-navigation-item')
        .classList.add('chapter-navigation-item--active');
    }
    
    // Expand sections for current chapter in accordion layout
    if (this.layout === 'accordion') {
      this.updateSectionsExpansion();
    }
  }
  
  updateSectionsExpansion() {
    const sections = this.element.querySelectorAll('[data-sections]');
    
    sections.forEach((section, index) => {
      if (index === this.state.currentChapter) {
        section.classList.add('chapter-navigation-sections--expanded');
      } else {
        section.classList.remove('chapter-navigation-sections--expanded');
      }
    });
  }
  
  updateProgress(progress = null) {
    if (!this.progressBar) return;
    
    if (progress === null) {
      // Calculate progress from current chapter
      const totalChapters = this.links.length;
      progress = (this.state.currentChapter / Math.max(1, totalChapters - 1)) * 100;
    }
    
    this.progressBar.style.width = `${Math.max(0, Math.min(100, progress))}%`;
    
    // Update progress text
    const progressText = this.element.querySelector('.chapter-navigation-progress-text');
    if (progressText) {
      progressText.textContent = `Chapter ${this.state.currentChapter + 1} of ${this.links.length}`;
    }
    
    // Dispatch progress event
    this.element.dispatchEvent(new CustomEvent('chapter_progress_changed', {
      detail: { 
        chapter: this.state.currentChapter + 1,
        total: this.links.length,
        progress
      }
    }));
  }
  
  updateNavigationButtons() {
    if (this.prevBtn) {
      this.prevBtn.disabled = this.state.currentChapter === 0;
    }
    
    if (this.nextBtn) {
      this.nextBtn.disabled = this.state.currentChapter >= this.links.length - 1;
    }
  }
  
  getCurrentChapter() {
    // Try to get current chapter from URL or data attribute
    const urlPath = window.location.pathname;
    
    for (let i = 0; i < this.links.length; i++) {
      const link = this.links[i];
      if (link.href && link.href.includes(urlPath)) {
        return i;
      }
    }
    
    // Check for active class as fallback
    const activeItem = this.element.querySelector('.chapter-navigation-item--active');
    if (activeItem) {
      return parseInt(activeItem.dataset.chapter) || 0;
    }
    
    return 0;
  }
  
  // Public methods
  setLayout(layout) {
    this.layout = layout;
    this.element.dataset.layout = layout;
    
    // Re-initialize based on new layout
    if (layout === 'accordion') {
      this.updateSectionsExpansion();
    }
  }
  
  expand() {
    if (this.state.isCollapsed) {
      this.toggleCollapsed();
    }
  }
  
  collapse() {
    if (!this.state.isCollapsed) {
      this.toggleCollapsed();
    }
  }
  
  refresh() {
    // Re-initialize the component
    this.state.currentChapter = this.getCurrentChapter();
    this.updateActiveChapter();
    this.updateProgress();
    this.updateNavigationButtons();
  }
}

// Initialize chapter_navigation components
window.PETA_COMPONENTS = window.PETA_COMPONENTS || {};
window.PETA_COMPONENTS.chapter_navigation = (element, props = {}) => {
  return new ChapterNavigationComponent(element);
};

// Auto-initialize
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('[data-component="chapter_navigation"]').forEach(element => {
    new ChapterNavigationComponent(element);
  });
});