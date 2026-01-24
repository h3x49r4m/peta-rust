// Navigation Hook - Provides navigation functionality and state management
class useNavigation {
  constructor() {
    this.currentPath = window.location.pathname;
    this.previousPath = null;
    this.navigationHistory = [];
    this.scrollPositions = new Map();
    this.isInitialized = false;
    this.init();
  }
  
  init() {
    this.setupEventListeners();
    this.restoreScrollPosition();
    this.isInitialized = true;
  }
  
  setupEventListeners() {
    // Track navigation changes
    window.addEventListener('popstate', (e) => {
      this.previousPath = this.currentPath;
      this.currentPath = window.location.pathname;
      this.restoreScrollPosition();
      this.emit('navigationChange', {
        currentPath: this.currentPath,
        previousPath: this.previousPath,
        direction: 'back'
      });
    });
    
    // Save scroll position before navigation
    window.addEventListener('beforeunload', () => {
      this.saveScrollPosition();
    });
    
    // Track scroll positions for SPA-like navigation
    window.addEventListener('scroll', this.debounce(() => {
      this.saveScrollPosition();
    }, 100));
    
    // Handle navigation clicks
    document.addEventListener('click', (e) => {
      const link = e.target.closest('a[href]');
      if (link && this.shouldHandleNavigation(link)) {
        e.preventDefault();
        this.navigateTo(link.href);
      }
    });
  }
  
  shouldHandleNavigation(link) {
    const href = link.getAttribute('href');
    
    // Don't handle external links
    if (href.startsWith('http://') || href.startsWith('https://')) {
      return false;
    }
    
    // Don't handle anchor links
    if (href.startsWith('#')) {
      return false;
    }
    
    // Don't handle mailto links
    if (href.startsWith('mailto:')) {
      return false;
    }
    
    // Don't handle file downloads
    if (href.includes('.') && !href.includes('.html')) {
      return false;
    }
    
    // Only handle same-origin links
    return href.startsWith('/') || href.startsWith('./');
  }
  
  navigateTo(path, options = {}) {
    const {
      replace = false,
      saveScroll = true,
      state = {}
    } = options;
    
    if (saveScroll) {
      this.saveScrollPosition();
    }
    
    this.previousPath = this.currentPath;
    this.currentPath = path;
    
    // Update browser history
    if (replace) {
      window.history.replaceState(state, '', path);
    } else {
      window.history.pushState(state, '', path);
    }
    
    // Add to navigation history
    this.navigationHistory.push({
      path,
      timestamp: Date.now(),
      state
    });
    
    // Load new page content (for SPA-like behavior)
    if (!options.external) {
      this.loadPageContent(path);
    }
    
    this.emit('navigationChange', {
      currentPath: this.currentPath,
      previousPath: this.previousPath,
      direction: replace ? 'replace' : 'forward'
    });
  }
  
  async loadPageContent(path) {
    try {
      // Show loading state
      this.emit('pageLoadStart', { path });
      
      // Fetch page content
      const response = await fetch(path);
      if (!response.ok) {
        throw new Error(`Failed to load page: ${response.statusText}`);
      }
      
      const html = await response.text();
      
      // Parse HTML and extract content
      const parser = new DOMParser();
      const doc = parser.parseFromString(html, 'text/html');
      
      // Update page content
      const mainContent = doc.querySelector('main, .main-content, #content');
      const currentMain = document.querySelector('main, .main-content, #content');
      
      if (mainContent && currentMain) {
        currentMain.innerHTML = mainContent.innerHTML;
      }
      
      // Update page title
      document.title = doc.title;
      
      // Update meta tags
      this.updateMetaTags(doc);
      
      // Scroll to top
      window.scrollTo(0, 0);
      
      // Reinitialize components
      this.reinitializeComponents();
      
      this.emit('pageLoadComplete', { path });
    } catch (error) {
      console.error('Failed to load page:', error);
      this.emit('pageLoadError', { path, error });
    }
  }
  
  updateMetaTags(doc) {
    // Update description
    const description = doc.querySelector('meta[name="description"]');
    if (description) {
      let currentDesc = document.querySelector('meta[name="description"]');
      if (!currentDesc) {
        currentDesc = document.createElement('meta');
        currentDesc.name = 'description';
        document.head.appendChild(currentDesc);
      }
      currentDesc.content = description.content;
    }
    
    // Update other meta tags as needed
    const keywords = doc.querySelector('meta[name="keywords"]');
    if (keywords) {
      let currentKeywords = document.querySelector('meta[name="keywords"]');
      if (!currentKeywords) {
        currentKeywords = document.createElement('meta');
        currentKeywords.name = 'keywords';
        document.head.appendChild(currentKeywords);
      }
      currentKeywords.content = keywords.content;
    }
  }
  
  reinitializeComponents() {
    // Reinitialize any components that need to be refreshed
    // This would be called after page content is updated
    this.emit('componentsReinitialize');
  }
  
  saveScrollPosition() {
    const scrollY = window.pageYOffset;
    this.scrollPositions.set(this.currentPath, scrollY);
  }
  
  restoreScrollPosition() {
    const scrollY = this.scrollPositions.get(this.currentPath) || 0;
    window.scrollTo(0, scrollY);
  }
  
  goBack() {
    if (this.navigationHistory.length > 1) {
      window.history.back();
    }
  }
  
  goForward() {
    window.history.forward();
  }
  
  canGoBack() {
    return this.navigationHistory.length > 1;
  }
  
  canGoForward() {
    return window.history.length > 1;
  }
  
  getCurrentPath() {
    return this.currentPath;
  }
  
  getPreviousPath() {
    return this.previousPath;
  }
  
  getNavigationHistory() {
    return [...this.navigationHistory];
  }
  
  // Event system
  emit(event, data) {
    const customEvent = new CustomEvent(`navigation:${event}`, { detail: data });
    document.dispatchEvent(customEvent);
  }
  
  on(event, callback) {
    document.addEventListener(`navigation:${event}`, callback);
  }
  
  off(event, callback) {
    document.removeEventListener(`navigation:${event}`, callback);
  }
  
  // Utility function for debouncing
  debounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
      const later = () => {
        clearTimeout(timeout);
        func(...args);
      };
      clearTimeout(timeout);
      timeout = setTimeout(later, wait);
    };
  }
}

// Export for use in components
window.useNavigation = useNavigation;

// Auto-initialize navigation hook
const navigationHook = new useNavigation();