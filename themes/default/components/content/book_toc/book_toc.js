class BookTocComponent {
  constructor(element) {
    this.element = element;
    this.sticky = element.dataset.sticky === 'true';
    this.collapsible = element.dataset.collapsible === 'true';
    this.content = element.querySelector('[data-content]');
    this.toggle = element.querySelector('[data-toggle]');
    this.list = element.querySelector('[data-list]');
    this.links = element.querySelectorAll('[data-toc-link]');
    this.expandButtons = element.querySelectorAll('[data-expand]');
    this.progressBar = element.querySelector('.book-toc-progress-fill');
    
    this.state = {
      isCollapsed: false,
      activeItem: null,
      expandedItems: new Set()
    };
    
    this.init();
  }
  
  init() {
    this.setupEventListeners();
    this.setupIntersectionObserver();
    this.initializeExpandedItems();
    this.updateActiveFromUrl();
  }
  
  setupEventListeners() {
    // Toggle collapsible state
    if (this.toggle && this.collapsible) {
      this.toggle.addEventListener('click', () => this.toggleCollapsed());
    }
    
    // TOC link clicks
    this.links.forEach(link => {
      link.addEventListener('click', (e) => this.handleLinkClick(e, link));
    });
    
    // Expand button clicks
    this.expandButtons.forEach(button => {
      button.addEventListener('click', (e) => this.handleExpandClick(e, button));
    });
    
    // Keyboard navigation
    this.element.addEventListener('keydown', (e) => this.handleKeydown(e));
    
    // Custom events
    this.element.addEventListener('toc_update', (e) => this.handleTocUpdate(e));
    this.element.addEventListener('progress_update', (e) => this.handleProgressUpdate(e));
    
    // Window resize for sticky behavior
    if (this.sticky) {
      window.addEventListener('resize', () => this.handleResize());
    }
  }
  
  setupIntersectionObserver() {
    // Observe all sections to update active TOC item
    const sections = this.getSections();
    
    const observer = new IntersectionObserver((entries) => {
      entries.forEach(entry => {
        const link = this.getLinkForSection(entry.target);
        if (entry.isIntersecting && link) {
          this.setActiveItem(link);
        }
      });
    }, {
      rootMargin: '-20% 0px -60% 0px'
    });
    
    sections.forEach(section => observer.observe(section));
  }
  
  initializeExpandedItems() {
    // Initialize expanded items from active state
    this.links.forEach(link => {
      const item = link.closest('.book-toc-item');
      const isActive = item.classList.contains('book-toc-item--active');
      const hasChildren = item.querySelector('[data-children]');
      
      if (hasChildren && (isActive || item.classList.contains('book-toc-children--expanded'))) {
        this.state.expandedItems.add(item);
      }
    });
  }
  
  updateActiveFromUrl() {
    // Set active item based on current URL
    const currentPath = window.location.pathname + window.location.hash;
    
    this.links.forEach(link => {
      if (link.href === currentPath || link.href.includes(window.location.pathname)) {
        this.setActiveItem(link);
        
        // Expand parent items
        let parent = link.closest('.book-toc-children');
        while (parent) {
          const parentItem = parent.closest('.book-toc-item');
          if (parentItem) {
            this.expandItem(parentItem);
          }
          parent = parentItem?.closest('.book-toc-children');
        }
      }
    });
  }
  
  toggleCollapsed() {
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
    
    // Dispatch toggle event
    this.element.dispatchEvent(new CustomEvent('toc_toggle', {
      detail: { collapsed: this.state.isCollapsed }
    }));
  }
  
  handleLinkClick(event, link) {
    const targetId = link.dataset.target;
    const targetElement = document.getElementById(targetId);
    
    if (targetElement) {
      event.preventDefault();
      
      // Smooth scroll to target
      targetElement.scrollIntoView({
        behavior: 'smooth',
        block: 'start'
      });
      
      // Update URL hash
      history.replaceState(null, '', `#${targetId}`);
      
      // Track TOC interaction
      this.trackEvent('toc_click', {
        target: targetId,
        title: link.querySelector('.book-toc-text').textContent.trim(),
        level: link.closest('.book-toc-item').dataset.level
      });
    }
  }
  
  handleExpandClick(event, button) {
    event.preventDefault();
    event.stopPropagation();
    
    const item = button.closest('.book-toc-item');
    this.toggleExpanded(item);
  }
  
  handleKeydown(event) {
    const links = Array.from(this.links);
    const currentIndex = links.indexOf(this.state.activeItem);
    
    switch (event.key) {
      case 'ArrowDown':
        event.preventDefault();
        const nextIndex = Math.min(currentIndex + 1, links.length - 1);
        this.setActiveItem(links[nextIndex]);
        links[nextIndex].click();
        break;
        
      case 'ArrowUp':
        event.preventDefault();
        const prevIndex = Math.max(currentIndex - 1, 0);
        this.setActiveItem(links[prevIndex]);
        links[prevIndex].click();
        break;
        
      case 'ArrowRight':
        if (this.state.activeItem) {
          const currentItem = this.state.activeItem.closest('.book-toc-item');
          const expandBtn = currentItem.querySelector('.book-toc-expand');
          if (expandBtn) {
            event.preventDefault();
            this.expandItem(currentItem);
          }
        }
        break;
        
      case 'ArrowLeft':
        if (this.state.activeItem) {
          const currentItem = this.state.activeItem.closest('.book-toc-item');
          const expandBtn = currentItem.querySelector('.book-toc-expand');
          if (expandBtn) {
            event.preventDefault();
            this.collapseItem(currentItem);
          }
        }
        break;
        
      case 'Home':
        event.preventDefault();
        if (links[0]) links[0].click();
        break;
        
      case 'End':
        event.preventDefault();
        if (links[links.length - 1]) links[links.length - 1].click();
        break;
        
      case 'Escape':
        this.blur();
        break;
    }
  }
  
  handleTocUpdate(event) {
    const { book } = event.detail;
    this.updateBookInfo(book);
  }
  
  handleProgressUpdate(event) {
    const { progress } = event.detail;
    this.updateProgress(progress);
  }
  
  handleResize() {
    // Recalculate sticky behavior on resize
    if (this.sticky) {
      const tocHeight = this.element.offsetHeight;
      const viewportHeight = window.innerHeight;
      const navbarHeight = 64; // Approximate navbar height
      
      if (tocHeight > viewportHeight - navbarHeight - 100) {
        this.element.style.maxHeight = `${viewportHeight - navbarHeight - 100}px`;
      } else {
        this.element.style.maxHeight = '';
      }
    }
  }
  
  setActiveItem(link) {
    // Remove active class from all items
    this.links.forEach(l => {
      l.closest('.book-toc-item').classList.remove('book-toc-item--active');
    });
    
    // Add active class to current item
    const item = link.closest('.book-toc-item');
    item.classList.add('book-toc-item--active');
    this.state.activeItem = link;
    
    // Expand parent items
    let parent = item.closest('.book-toc-children');
    while (parent) {
      const parentItem = parent.closest('.book-toc-item');
      if (parentItem) {
        this.expandItem(parentItem);
      }
      parent = parentItem?.closest('.book-toc-children');
    }
    
    // Dispatch active change event
    this.element.dispatchEvent(new CustomEvent('toc_active_change', {
      detail: { 
        target: link.dataset.target,
        title: link.querySelector('.book-toc-text').textContent.trim(),
        level: item.dataset.level
      }
    }));
  }
  
  toggleExpanded(item) {
    if (this.state.expandedItems.has(item)) {
      this.collapseItem(item);
    } else {
      this.expandItem(item);
    }
  }
  
  expandItem(item) {
    const children = item.querySelector('[data-children]');
    if (children) {
      children.classList.add('book-toc-children--expanded');
      this.state.expandedItems.add(item);
      
      // Update expand button icon
      const expandBtn = item.querySelector('.book-toc-expand');
      if (expandBtn) {
        expandBtn.style.transform = 'rotate(90deg)';
      }
    }
  }
  
  collapseItem(item) {
    const children = item.querySelector('[data-children]');
    if (children) {
      children.classList.remove('book-toc-children--expanded');
      this.state.expandedItems.delete(item);
      
      // Update expand button icon
      const expandBtn = item.querySelector('.book-toc-expand');
      if (expandBtn) {
        expandBtn.style.transform = '';
      }
    }
  }
  
  updateBookInfo(book) {
    // Update progress
    if (book.progress_percentage !== undefined) {
      this.updateProgress(book.progress_percentage);
    }
    
    // Update title and subtitle
    const titleElement = this.element.querySelector('.book-toc-title');
    if (titleElement && book.title) {
      titleElement.textContent = book.title;
    }
    
    const subtitleElement = this.element.querySelector('.book-toc-subtitle');
    if (subtitleElement && book.subtitle) {
      subtitleElement.textContent = book.subtitle;
    }
  }
  
  updateProgress(percentage) {
    if (this.progressBar) {
      this.progressBar.style.width = `${Math.max(0, Math.min(100, percentage))}%`;
    }
    
    // Update progress text
    const progressText = this.element.querySelector('.book-toc-progress-text');
    if (progressText) {
      progressText.textContent = `${Math.round(percentage)}% complete`;
    }
  }
  
  blur() {
    this.element.blur();
    this.state.activeItem = null;
    
    // Remove active class from all items
    this.links.forEach(l => {
      l.closest('.book-toc-item').classList.remove('book-toc-item--active');
    });
  }
  
  getSections() {
    return this.links
      .map(link => {
        const targetId = link.dataset.target;
        return document.getElementById(targetId);
      })
      .filter(section => section !== null);
  }
  
  getLinkForSection(section) {
    const sectionId = section.id;
    return this.links.find(link => link.dataset.target === sectionId);
  }
  
  trackEvent(action, data = {}) {
    this.element.dispatchEvent(new CustomEvent(`toc_${action}`, {
      detail: data
    }));
  }
  
  // Public methods
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
  
  expandAll() {
    this.element.querySelectorAll('.book-toc-item').forEach(item => {
      this.expandItem(item);
    });
  }
  
  collapseAll() {
    this.element.querySelectorAll('.book-toc-item').forEach(item => {
      this.collapseItem(item);
    });
  }
  
  refresh() {
    // Re-initialize links and observers
    this.links = this.element.querySelectorAll('[data-toc-link]');
    this.expandButtons = this.element.querySelectorAll('[data-expand]');
    this.setupIntersectionObserver();
    this.initializeExpandedItems();
    this.updateActiveFromUrl();
  }
}

// Initialize book_toc components
window.PETA_COMPONENTS = window.PETA_COMPONENTS || {};
window.PETA_COMPONENTS.book_toc = (element, props = {}) => {
  return new BookTocComponent(element);
};

// Auto-initialize
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('[data-component="book_toc"]').forEach(element => {
    new BookTocComponent(element);
  });
});