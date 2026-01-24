class BookProgressComponent {
  constructor(element) {
    this.element = element;
    this.compact = element.dataset.compact === 'true';
    this.progressBar = element.querySelector('.book-progress-fill');
    this.percentageText = element.querySelector('.book-progress-percentage');
    this.timelineItems = element.querySelectorAll('.book-progress-timeline-item');
    this.actionButtons = element.querySelectorAll('.book-progress-action');
    
    this.state = {
      progress: this.getCurrentProgress(),
      isLoading: false,
      status: this.getProgressStatus()
    };
    
    this.init();
  }
  
  init() {
    this.setupEventListeners();
    this.setupTimelineInteractions();
    this.initializeProgress();
  }
  
  setupEventListeners() {
    // Action button clicks
    this.actionButtons.forEach(button => {
      button.addEventListener('click', (e) => this.handleActionClick(e, button));
    });
    
    // Timeline item clicks
    this.timelineItems.forEach(item => {
      item.addEventListener('click', () => this.handleTimelineClick(item));
    });
    
    // Custom events
    this.element.addEventListener('progress_update', (e) => this.handleProgressUpdate(e));
    this.element.addEventListener('book_update', (e) => this.handleBookUpdate(e));
    
    // Keyboard navigation
    this.element.addEventListener('keydown', (e) => this.handleKeydown(e));
  }
  
  setupTimelineInteractions() {
    // Add tooltips to timeline items
    this.timelineItems.forEach(item => {
      const chapterIndex = parseInt(item.dataset.chapter);
      const chapterTitle = item.title;
      
      item.addEventListener('mouseenter', () => {
        this.showTooltip(item, chapterTitle);
      });
      
      item.addEventListener('mouseleave', () => {
        this.hideTooltip(item);
      });
    });
  }
  
  initializeProgress() {
    // Animate initial progress
    setTimeout(() => {
      this.animateProgress(this.state.progress);
    }, 300);
    
    // Set initial status
    this.updateStatus(this.state.status);
  }
  
  handleActionClick(event, button) {
    const action = button.dataset.action;
    
    // Track action clicks
    this.element.dispatchEvent(new CustomEvent('progress_action_click', {
      detail: { action }
    }));
    
    switch (action) {
      case 'continue':
        this.continueReading();
        break;
      case 'reset':
        this.resetProgress();
        break;
      case 'mark_complete':
        this.markAsComplete();
        break;
    }
    
    // Add loading state
    button.classList.add('loading');
    button.disabled = true;
    
    // Remove loading state after action
    setTimeout(() => {
      button.classList.remove('loading');
      button.disabled = false;
    }, 1000);
  }
  
  handleTimelineClick(item) {
    const chapterIndex = parseInt(item.dataset.chapter);
    
    // Track timeline clicks
    this.element.dispatchEvent(new CustomEvent('timeline_click', {
      detail: { chapterIndex }
    }));
    
    // Navigate to chapter (in a real implementation, this would navigate to the chapter)
    console.log(`Navigate to chapter ${chapterIndex + 1}`);
  }
  
  handleProgressUpdate(event) {
    const { progress } = event.detail;
    this.updateProgress(progress);
  }
  
  handleBookUpdate(event) {
    const { book } = event.detail;
    this.updateBookInfo(book);
  }
  
  handleKeydown(event) {
    switch (event.key) {
      case 'ArrowRight':
        event.preventDefault();
        this.incrementProgress(5);
        break;
      case 'ArrowLeft':
        event.preventDefault();
        this.incrementProgress(-5);
        break;
      case 'Home':
        event.preventDefault();
        this.updateProgress(0);
        break;
      case 'End':
        event.preventDefault();
        this.updateProgress(100);
        break;
    }
  }
  
  continueReading() {
    // Find current chapter and navigate to it
    const currentChapter = this.element.querySelector('.book-progress-current-title');
    if (currentChapter) {
      const chapterTitle = currentChapter.textContent.trim();
      
      // Track continue reading action
      this.element.dispatchEvent(new CustomEvent('continue_reading', {
        detail: { chapterTitle }
      }));
      
      // In a real implementation, this would navigate to the current chapter
      console.log(`Continue reading: ${chapterTitle}`);
    }
  }
  
  resetProgress() {
    // Confirm reset
    if (confirm('Are you sure you want to reset your reading progress? This cannot be undone.')) {
      this.updateProgress(0);
      
      // Track reset action
      this.element.dispatchEvent(new CustomEvent('progress_reset'));
      
      // Update UI
      this.updateTimelineItems(0);
    }
  }
  
  markAsComplete() {
    this.updateProgress(100);
    this.updateStatus('completed');
    
    // Track completion
    this.element.dispatchEvent(new CustomEvent('book_completed'));
  }
  
  updateProgress(percentage) {
    this.state.progress = Math.max(0, Math.min(100, percentage));
    
    // Update progress bar
    if (this.progressBar) {
      this.progressBar.style.width = `${this.state.progress}%`;
    }
    
    // Update percentage text
    if (this.percentageText) {
      this.percentageText.textContent = `${Math.round(this.state.progress)}%`;
    }
    
    // Update timeline items
    this.updateTimelineItems(this.state.progress);
    
    // Update status
    this.state.status = this.getProgressStatus();
    this.updateStatus(this.state.status);
    
    // Dispatch progress change event
    this.element.dispatchEvent(new CustomEvent('progress_changed', {
      detail: { progress: this.state.progress, status: this.state.status }
    }));
  }
  
  animateProgress(targetProgress) {
    const startProgress = parseFloat(this.progressBar.style.width) || 0;
    const duration = 1000; // 1 second animation
    const startTime = performance.now();
    
    const animate = (currentTime) => {
      const elapsed = currentTime - startTime;
      const progress = Math.min(elapsed / duration, 1);
      
      // Easing function
      const easeOutQuart = 1 - Math.pow(1 - progress, 4);
      const currentProgress = startProgress + (targetProgress - startProgress) * easeOutQuart;
      
      this.progressBar.style.width = `${currentProgress}%`;
      this.percentageText.textContent = `${Math.round(currentProgress)}%`;
      
      if (progress < 1) {
        requestAnimationFrame(animate);
      }
    };
    
    requestAnimationFrame(animate);
  }
  
  updateTimelineItems(progress) {
    const totalChapters = this.timelineItems.length;
    const completedChapters = Math.floor((progress / 100) * totalChapters);
    
    this.timelineItems.forEach((item, index) => {
      item.classList.remove('book-progress-timeline-item--completed', 'book-progress-timeline-item--current');
      
      if (index < completedChapters) {
        item.classList.add('book-progress-timeline-item--completed');
      } else if (index === completedChapters) {
        item.classList.add('book-progress-timeline-item--current');
      }
    });
  }
  
  updateStatus(status) {
    this.element.dataset.status = status;
    
    // Update status-specific styling
    if (status === 'completed') {
      this.element.classList.add('book-progress--completed');
    } else {
      this.element.classList.remove('book-progress--completed');
    }
  }
  
  updateBookInfo(book) {
    // Update progress percentage
    if (book.progress_percentage !== undefined) {
      this.updateProgress(book.progress_percentage);
    }
    
    // Update current chapter
    const currentTitleElement = this.element.querySelector('.book-progress-current-title');
    if (currentTitleElement && book.current_chapter?.title) {
      currentTitleElement.textContent = book.current_chapter.title;
    }
    
    // Update last read date
    const lastReadElement = this.element.querySelector('.book-progress-last-read-date');
    if (lastReadElement && book.last_read) {
      lastReadElement.textContent = new Date(book.last_read).toLocaleDateString();
    }
    
    // Update statistics
    this.updateStatistics(book);
  }
  
  updateStatistics(book) {
    // Update completed chapters
    const completedChaptersElement = this.element.querySelector('.book-progress-stat-value');
    if (completedChaptersElement && book.completed_chapters !== undefined) {
      completedChaptersElement.textContent = book.completed_chapters;
    }
    
    // Update reading time
    const readingTimeElements = this.element.querySelectorAll('.book-progress-stat-value');
    readingTimeElements.forEach(element => {
      if (element.nextElementSibling?.textContent === 'Reading time' && book.reading_time) {
        element.textContent = book.reading_time;
      }
    });
  }
  
  incrementProgress(amount) {
    const newProgress = Math.max(0, Math.min(100, this.state.progress + amount));
    this.updateProgress(newProgress);
  }
  
  getCurrentProgress() {
    if (this.progressBar) {
      const width = this.progressBar.style.width || '0%';
      return parseFloat(width.replace('%', '')) || 0;
    }
    return 0;
  }
  
  getProgressStatus() {
    if (this.state.progress >= 100) return 'completed';
    if (this.state.progress >= 75) return 'near_complete';
    if (this.state.progress >= 50) return 'halfway';
    if (this.state.progress >= 25) return 'started';
    return 'not_started';
  }
  
  showTooltip(item, text) {
    // Create tooltip element
    const tooltip = document.createElement('div');
    tooltip.className = 'book-progress-tooltip';
    tooltip.textContent = text;
    tooltip.style.cssText = `
      position: absolute;
      bottom: 100%;
      left: 50%;
      transform: translateX(-50%);
      background: var(--background-primary);
      color: var(--text-primary);
      padding: 4px 8px;
      border-radius: 4px;
      font-size: 12px;
      white-space: nowrap;
      z-index: 1000;
      box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
      margin-bottom: 4px;
    `;
    
    item.appendChild(tooltip);
  }
  
  hideTooltip(item) {
    const tooltip = item.querySelector('.book-progress-tooltip');
    if (tooltip) {
      tooltip.remove();
    }
  }
  
  // Public methods
  setProgress(percentage) {
    this.updateProgress(percentage);
  }
  
  getProgress() {
    return this.state.progress;
  }
  
  getStatus() {
    return this.state.status;
  }
  
  setLoading(isLoading) {
    this.state.isLoading = isLoading;
    
    if (isLoading) {
      this.element.classList.add('loading');
    } else {
      this.element.classList.remove('loading');
    }
  }
  
  refresh() {
    // Re-initialize the component
    this.state.progress = this.getCurrentProgress();
    this.state.status = this.getProgressStatus();
    this.initializeProgress();
  }
}

// Initialize book_progress components
window.PETA_COMPONENTS = window.PETA_COMPONENTS || {};
window.PETA_COMPONENTS.book_progress = (element, props = {}) => {
  return new BookProgressComponent(element);
};

// Auto-initialize
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('[data-component="book_progress"]').forEach(element => {
    new BookProgressComponent(element);
  });
});