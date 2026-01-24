class ArticleTocComponent {
  constructor(element) {
    this.element = element;
    this.content = element.querySelector('[data-toc-content]');
    this.toggle = element.querySelector('[data-toc-toggle]');
    this.links = element.querySelectorAll('[data-toc-link]');
    this.isSticky = element.dataset.sticky !== undefined;
    this.isCollapsible = element.dataset.collapsible !== undefined;
    
    this.state = {
      collapsed: false,
      activeItem: null,
      progress: 0
    };
    
    this.init();
  }
  
  init() {
    this.setupEventListeners();
    this.setupIntersectionObserver();
    this.setupProgressIndicator();
    this.initializeCollapsible();
  }
  
  setupEventListeners() {
    // Toggle collapsible state
    if (this.toggle) {
      this.toggle.addEventListener('click', () => this.toggleCollapsible());
    }
    
    // Link clicks with smooth scrolling
    this.links.forEach(link => {
      link.addEventListener('click', (e) => this.handleLinkClick(e, link));
    });
    
    // Keyboard navigation
    this.element.addEventListener('keydown', (e) => this.handleKeydown(e));
    
    // Window resize for sticky behavior
    window.addEventListener('resize', () => this.handleResize());
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
  
  setupProgressIndicator() {
    this.links.forEach(link => {
      const progress = document.createElement('div');
      progress.className = 'article-toc-progress';
      link.style.position = 'relative';
      link.appendChild(progress);
    });
  }
  
  initializeCollapsible() {
    if (!this.isCollapsible) return;
    
    // Set initial state
    if (this.state.collapsed) {
      this.content.classList.add('collapsed');
      this.element.dataset.collapsed = 'true';
    }
  }
  
  toggleCollapsible() {
    this.state.collapsed = !this.state.collapsed;
    
    if (this.state.collapsed) {
      this.content.classList.add('collapsed');
      this.element.dataset.collapsed = 'true';
    } else {
      this.content.classList.remove('collapsed');
      this.element.dataset.collapsed = 'false';
    }
    
    // Dispatch toggle event
    this.element.dispatchEvent(new CustomEvent('toc_toggle', {
      detail: { collapsed: this.state.collapsed }
    }));
  }
  
  handleLinkClick(event, link) {
    event.preventDefault();
    
    const targetId = link.dataset.target;
    const targetElement = document.getElementById(targetId);
    
    if (targetElement) {
      // Smooth scroll to target
      targetElement.scrollIntoView({
        behavior: 'smooth',
        block: 'start'
      });
      
      // Update active state
      this.setActiveItem(link);
      
      // Update URL hash
      history.replaceState(null, '', `#${targetId}`);
      
      // Track TOC interaction
      this.trackEvent('toc_click', {
        target: targetId,
        title: link.textContent.trim()
      });
    }
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
        
      case 'Home':
        event.preventDefault();
        window.scrollTo({ top: 0, behavior: 'smooth' });
        break;
        
      case 'Escape':
        this.blur();
        break;
    }
  }
  
  handleResize() {
    // Recalculate sticky behavior on resize
    if (this.isSticky) {
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
    // Remove active class from all links
    this.links.forEach(l => l.classList.remove('active'));
    
    // Add active class to current link
    link.classList.add('active');
    this.state.activeItem = link;
    
    // Update progress indicator
    this.updateProgress(link);
    
    // Update toggle icon rotation for nested items
    this.updateToggleIcons(link);
    
    // Dispatch active change event
    this.element.dispatchEvent(new CustomEvent('toc_active_change', {
      detail: { 
        target: link.dataset.target,
        title: link.textContent.trim()
      }
    }));
  }
  
  updateProgress(link) {
    const progress = link.querySelector('.article-toc-progress');
    if (progress) {
      const linkIndex = Array.from(this.links).indexOf(link);
      const totalLinks = this.links.length;
      const percentage = (linkIndex + 1) / totalLinks * 100;
      progress.style.width = `${percentage}%`;
    }
  }
  
  updateToggleIcons(link) {
    const toggleIcon = link.querySelector('.article-toc-toggle-icon');
    if (toggleIcon) {
      const hasActiveChildren = link.parentElement.querySelector('.article-toc-children .active');
      if (hasActiveChildren) {
        toggleIcon.style.transform = 'rotate(90deg)';
      } else {
        toggleIcon.style.transform = '';
      }
    }
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
  
  blur() {
    this.element.blur();
    this.state.activeItem = null;
    
    // Remove active class from all links
    this.links.forEach(l => {
      l.classList.remove('active');
      const progress = l.querySelector('.article-toc-progress');
      if (progress) {
        progress.style.width = '0%';
      }
    });
  }
  
  // Public methods
  scrollToSection(targetId) {
    const link = this.getLinkForSection(document.getElementById(targetId));
    if (link) {
      link.click();
    }
  }
  
  expand() {
    if (this.state.collapsed) {
      this.toggleCollapsible();
    }
  }
  
  collapse() {
    if (!this.state.collapsed) {
      this.toggleCollapsible();
    }
  }
  
  refresh() {
    // Re-initialize links and observers
    this.links = this.element.querySelectorAll('[data-toc-link]');
    this.setupIntersectionObserver();
    this.setupProgressIndicator();
  }
  
  trackEvent(action, data = {}) {
    this.element.dispatchEvent(new CustomEvent(`toc_${action}`, {
      detail: data
    }));
  }
}

// Initialize article_toc components
window.PETA_COMPONENTS = window.PETA_COMPONENTS || {};
window.PETA_COMPONENTS.article_toc = (element, props = {}) => {
  return new ArticleTocComponent(element);
};

// Auto-initialize
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('[data-component="article_toc"]').forEach(element => {
    new ArticleTocComponent(element);
  });
});