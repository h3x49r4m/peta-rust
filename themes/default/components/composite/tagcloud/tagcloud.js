class TagcloudComponent {
  constructor(element) {
    this.element = element;
    this.tagsContainer = element.querySelector('[data-tagcloud-tags]');
    this.tags = Array.from(element.querySelectorAll('[data-tag]'));
    this.variant = element.dataset.variant;
    
    this.state = {
      selectedTag: '',
      filteredTags: []
    };
    
    this.init();
  }
  
  init() {
    this.setupEventListeners();
    this.initializeInteractions();
  }
  
  setupEventListeners() {
    // Tag click events
    this.tags.forEach(tag => {
      tag.addEventListener('click', (e) => this.handleTagClick(e, tag));
      tag.addEventListener('mouseenter', () => this.handleTagHover(tag, true));
      tag.addEventListener('mouseleave', () => this.handleTagHover(tag, false));
    });
    
    // Custom events for external filtering
    this.element.addEventListener('filter', (e) => this.handleFilterEvent(e));
    this.element.addEventListener('select', (e) => this.handleSelectEvent(e));
  }
  
  initializeInteractions() {
    // Add hover effect for weighted variant
    if (this.variant === 'weighted') {
      this.setupWeightedInteractions();
    }
    
    // Add keyboard navigation
    this.setupKeyboardNavigation();
  }
  
  setupWeightedInteractions() {
    this.tags.forEach(tag => {
      const weight = parseInt(tag.dataset.weight);
      
      tag.addEventListener('mouseenter', () => {
        this.highlightRelatedTags(tag.dataset.tag, weight);
      });
      
      tag.addEventListener('mouseleave', () => {
        this.clearHighlight();
      });
    });
  }
  
  setupKeyboardNavigation() {
    let currentIndex = -1;
    
    this.element.addEventListener('keydown', (e) => {
      switch (e.key) {
        case 'ArrowRight':
        case 'ArrowDown':
          e.preventDefault();
          currentIndex = Math.min(currentIndex + 1, this.tags.length - 1);
          this.focusTag(currentIndex);
          break;
          
        case 'ArrowLeft':
        case 'ArrowUp':
          e.preventDefault();
          currentIndex = Math.max(currentIndex - 1, -1);
          this.focusTag(currentIndex);
          break;
          
        case 'Enter':
        case ' ':
          e.preventDefault();
          if (currentIndex >= 0 && this.tags[currentIndex]) {
            this.tags[currentIndex].click();
          }
          break;
          
        case 'Escape':
          this.clearFocus();
          currentIndex = -1;
          break;
      }
    });
  }
  
  handleTagClick(event, tag) {
    const tagName = tag.dataset.tag;
    const tagCount = parseInt(tag.dataset.count);
    
    // Update selected state
    this.state.selectedTag = tagName;
    
    // Dispatch custom event
    this.element.dispatchEvent(new CustomEvent('tagclick', {
      detail: {
        tag: tagName,
        count: tagCount,
        element: tag
      }
    }));
    
    // Add visual feedback
    this.animateTagClick(tag);
  }
  
  handleTagHover(tag, isHovering) {
    if (isHovering) {
      tag.style.transform = 'translateY(-2px) scale(1.05)';
    } else {
      tag.style.transform = '';
    }
  }
  
  handleFilterEvent(event) {
    const { filter, value } = event.detail;
    this.filterTags(filter, value);
  }
  
  handleSelectEvent(event) {
    const { tag } = event.detail;
    this.selectTag(tag);
  }
  
  highlightRelatedTags(tagName, weight) {
    const threshold = Math.max(weight - 20, 0);
    
    this.tags.forEach(tag => {
      const tagWeight = parseInt(tag.dataset.weight);
      if (tag.dataset.tag !== tagName && tagWeight >= threshold) {
        tag.classList.add('tagcloud-tag--highlighted');
      }
    });
  }
  
  clearHighlight() {
    this.tags.forEach(tag => {
      tag.classList.remove('tagcloud-tag--highlighted');
    });
  }
  
  focusTag(index) {
    if (index >= 0 && index < this.tags.length) {
      this.tags[index].focus();
    }
  }
  
  clearFocus() {
    this.element.blur();
  }
  
  filterTags(filter, value) {
    const filtered = this.tags.filter(tag => {
      const tagName = tag.dataset.tag.toLowerCase();
      const tagCount = parseInt(tag.dataset.count);
      
      switch (filter) {
        case 'name':
          return tagName.includes(value.toLowerCase());
        case 'count':
          return tagCount >= parseInt(value);
        case 'weight':
          return parseInt(tag.dataset.weight) >= parseInt(value);
        default:
          return true;
      }
    });
    
    // Show/hide tags based on filter
    this.tags.forEach(tag => {
      if (filtered.includes(tag)) {
        tag.style.display = '';
      } else {
        tag.style.display = 'none';
      }
    });
    
    this.state.filteredTags = filtered.map(tag => tag.dataset.tag);
    
    // Dispatch filter complete event
    this.element.dispatchEvent(new CustomEvent('filtercomplete', {
      detail: { filteredTags: this.state.filteredTags }
    }));
  }
  
  selectTag(tagName) {
    // Remove previous selection
    this.tags.forEach(tag => {
      tag.classList.remove('tagcloud-tag--selected');
    });
    
    // Add selection to matching tag
    const selectedTag = this.tags.find(tag => tag.dataset.tag === tagName);
    if (selectedTag) {
      selectedTag.classList.add('tagcloud-tag--selected');
      selectedTag.scrollIntoView({ behavior: 'smooth', block: 'center' });
    }
    
    this.state.selectedTag = tagName;
    
    // Dispatch selection event
    this.element.dispatchEvent(new CustomEvent('tagselect', {
      detail: { tag: tagName, element: selectedTag }
    }));
  }
  
  animateTagClick(tag) {
    tag.style.transform = 'scale(0.95)';
    tag.style.opacity = '0.7';
    
    setTimeout(() => {
      tag.style.transform = '';
      tag.style.opacity = '';
    }, 150);
  }
  
  // Public methods
  getSelectedTag() {
    return this.state.selectedTag;
  }
  
  getFilteredTags() {
    return this.state.filteredTags;
  }
  
  getTags() {
    return this.tags.map(tag => ({
      name: tag.dataset.tag,
      count: parseInt(tag.dataset.count),
      weight: parseInt(tag.dataset.weight),
      element: tag
    }));
  }
}

// Initialize tagcloud components
window.PETA_COMPONENTS = window.PETA_COMPONENTS || {};
window.PETA_COMPONENTS.tagcloud = (element, props = {}) => {
  return new TagcloudComponent(element);
};

// Auto-initialize
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('[data-component="tagcloud"]').forEach(element => {
    new TagcloudComponent(element);
  });
});