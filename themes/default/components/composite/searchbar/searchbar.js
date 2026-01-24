class SearchbarComponent {
  constructor(element) {
    this.element = element;
    this.input = element.querySelector('[data-search-input]');
    this.button = element.querySelector('[data-search-button]');
    this.results = element.querySelector('[data-search-results]');
    this.autocomplete = element.querySelector('[data-search-autocomplete]');
    this.filters = element.querySelector('[data-search-filters]');
    
    this.state = {
      query: '',
      focused: false,
      loading: false,
      selectedIndex: -1
    };
    
    this.init();
  }
  
  init() {
    this.setupEventListeners();
    this.setupAutocomplete();
  }
  
  setupEventListeners() {
    // Input events
    this.input.addEventListener('focus', () => this.handleFocus());
    this.input.addEventListener('blur', () => this.handleBlur());
    this.input.addEventListener('input', (e) => this.handleInput(e));
    this.input.addEventListener('keydown', (e) => this.handleKeydown(e));
    
    // Button events
    if (this.button) {
      this.button.addEventListener('click', () => this.handleSearch());
    }
    
    // Click outside to close autocomplete
    document.addEventListener('click', (e) => {
      if (!this.element.contains(e.target)) {
        this.hideAutocomplete();
      }
    });
  }
  
  setupAutocomplete() {
    if (!this.autocomplete) return;
    
    // Initialize search index
    this.searchIndex = window.PETA_SEARCH_INDEX || [];
  }
  
  handleFocus() {
    this.state.focused = true;
    this.element.classList.add('searchbar--focused');
    
    if (this.state.query) {
      this.showAutocomplete();
    }
  }
  
  handleBlur() {
    this.state.focused = false;
    this.element.classList.remove('searchbar--focused');
    
    // Delay hiding autocomplete to allow click on results
    setTimeout(() => this.hideAutocomplete(), 200);
  }
  
  handleInput(e) {
    this.state.query = e.target.value;
    
    if (this.state.query) {
      this.showAutocomplete();
    } else {
      this.hideAutocomplete();
    }
  }
  
  handleKeydown(e) {
    if (!this.autocomplete || !this.state.query) return;
    
    const results = this.results.querySelectorAll('.searchbar-result');
    
    switch (e.key) {
      case 'ArrowDown':
        e.preventDefault();
        this.state.selectedIndex = Math.min(this.state.selectedIndex + 1, results.length - 1);
        this.updateSelection(results);
        break;
        
      case 'ArrowUp':
        e.preventDefault();
        this.state.selectedIndex = Math.max(this.state.selectedIndex - 1, -1);
        this.updateSelection(results);
        break;
        
      case 'Enter':
        e.preventDefault();
        if (this.state.selectedIndex >= 0 && results[this.state.selectedIndex]) {
          results[this.state.selectedIndex].click();
        } else {
          this.handleSearch();
        }
        break;
        
      case 'Escape':
        this.hideAutocomplete();
        this.input.blur();
        break;
    }
  }
  
  handleSearch() {
    const query = this.state.query.trim();
    if (!query) return;
    
    this.state.loading = true;
    this.updateLoadingState();
    
    // Trigger search event
    this.element.dispatchEvent(new CustomEvent('search', {
      detail: { query }
    }));
    
    // Simulate search completion
    setTimeout(() => {
      this.state.loading = false;
      this.updateLoadingState();
      this.hideAutocomplete();
    }, 500);
  }
  
  showAutocomplete() {
    if (!this.autocomplete || !this.searchIndex.length) return;
    
    const results = this.searchResults(this.state.query);
    
    if (results.length === 0) {
      this.hideAutocomplete();
      return;
    }
    
    this.renderResults(results);
    this.autocomplete.classList.add('searchbar-autocomplete--visible');
    this.state.selectedIndex = -1;
  }
  
  hideAutocomplete() {
    if (this.autocomplete) {
      this.autocomplete.classList.remove('searchbar-autocomplete--visible');
    }
    this.state.selectedIndex = -1;
  }
  
  searchResults(query) {
    const queryLower = query.toLowerCase();
    const maxResults = 8;
    
    return this.searchIndex
      .filter(item => 
        item.title.toLowerCase().includes(queryLower) ||
        item.content.toLowerCase().includes(queryLower) ||
        item.tags.some(tag => tag.toLowerCase().includes(queryLower))
      )
      .slice(0, maxResults)
      .map(item => ({
        ...item,
        highlightedTitle: this.highlightText(item.title, query),
        highlightedContent: this.highlightText(item.excerpt || item.content, query)
      }));
  }
  
  highlightText(text, query) {
    if (!query) return text;
    
    const regex = new RegExp(`(${this.escapeRegex(query)})`, 'gi');
    return text.replace(regex, '<span class="searchbar-result-highlight">$1</span>');
  }
  
  escapeRegex(string) {
    return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  }
  
  renderResults(results) {
    this.results.innerHTML = results.map((result, index) => `
      <div class="searchbar-result" data-index="${index}" data-url="${result.url}">
        <div class="searchbar-result-content">
          <div class="searchbar-result-title">${result.highlightedTitle}</div>
          <div class="searchbar-result-type">${result.content_type}</div>
        </div>
      </div>
    `).join('');
    
    // Add click handlers
    this.results.querySelectorAll('.searchbar-result').forEach(result => {
      result.addEventListener('click', () => {
        const url = result.dataset.url;
        if (url) {
          window.location.href = url;
        }
      });
    });
  }
  
  updateSelection(results) {
    results.forEach((result, index) => {
      if (index === this.state.selectedIndex) {
        result.classList.add('searchbar-result--selected');
      } else {
        result.classList.remove('searchbar-result--selected');
      }
    });
  }
  
  updateLoadingState() {
    if (this.state.loading) {
      this.element.classList.add('searchbar--loading');
    } else {
      this.element.classList.remove('searchbar--loading');
    }
  }
}

// Initialize searchbar components
window.PETA_COMPONENTS = window.PETA_COMPONENTS || {};
window.PETA_COMPONENTS.searchbar = (element, props = {}) => {
  return new SearchbarComponent(element);
};

// Auto-initialize
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('[data-component="searchbar"]').forEach(element => {
    new SearchbarComponent(element);
  });
});