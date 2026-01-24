class LanguageFilterComponent {
  constructor(element) {
    this.element = element;
    this.layout = element.dataset.layout;
    this.variant = element.dataset.variant;
    this.select = element.querySelector('[data-select]');
    this.list = element.querySelector('[data-list]');
    this.items = element.querySelectorAll('[data-language]');
    this.clearButton = element.querySelector('[data-clear]');
    
    this.state = {
      selectedLanguage: this.getSelectedLanguage(),
      languages: this.getLanguagesData(),
      isLoading: false
    };
    
    this.init();
  }
  
  init() {
    this.setupEventListeners();
    this.setupKeyboardNavigation();
    this.updateActiveState();
  }
  
  setupEventListeners() {
    // Select change event
    if (this.select) {
      this.select.addEventListener('change', (e) => this.handleSelectChange(e));
    }
    
    // Item click events
    this.items.forEach(item => {
      item.addEventListener('click', () => this.handleItemClick(item));
    });
    
    // Clear button click
    if (this.clearButton) {
      this.clearButton.addEventListener('click', () => this.handleClearClick());
    }
    
    // Custom events
    this.element.addEventListener('filter_update', (e) => this.handleFilterUpdate(e));
    this.element.addEventListener('languages_update', (e) => this.handleLanguagesUpdate(e));
  }
  
  setupKeyboardNavigation() {
    this.element.addEventListener('keydown', (e) => {
      const items = Array.from(this.items);
      const currentIndex = items.findIndex(item => 
        item.dataset.language === this.state.selectedLanguage
      );
      
      switch (e.key) {
        case 'ArrowRight':
        case 'ArrowDown':
          e.preventDefault();
          const nextIndex = (currentIndex + 1) % items.length;
          this.selectLanguage(items[nextIndex].dataset.language);
          break;
          
        case 'ArrowLeft':
        case 'ArrowUp':
          e.preventDefault();
          const prevIndex = (currentIndex - 1 + items.length) % items.length;
          this.selectLanguage(items[prevIndex].dataset.language);
          break;
          
        case 'Home':
          e.preventDefault();
          this.selectLanguage('');
          break;
          
        case 'End':
          e.preventDefault();
          if (items.length > 0) {
            this.selectLanguage(items[items.length - 1].dataset.language);
          }
          break;
          
        case 'Escape':
          e.preventDefault();
          this.clearFilter();
          break;
      }
    });
  }
  
  handleSelectChange(event) {
    const language = event.target.value;
    this.selectLanguage(language);
  }
  
  handleItemClick(item) {
    const language = item.dataset.language;
    this.selectLanguage(language);
    
    // Track item click
    this.element.dispatchEvent(new CustomEvent('filter_item_click', {
      detail: { 
        language,
        name: item.querySelector('.language-filter-name')?.textContent?.trim(),
        count: parseInt(item.dataset.count) || 0
      }
    }));
  }
  
  handleClearClick() {
    this.clearFilter();
    
    // Track clear click
    this.element.dispatchEvent(new CustomEvent('filter_clear_click'));
  }
  
  handleFilterUpdate(event) {
    const { selectedLanguage } = event.detail;
    this.selectLanguage(selectedLanguage);
  }
  
  handleLanguagesUpdate(event) {
    const { languages } = event.detail;
    this.updateLanguages(languages);
  }
  
  selectLanguage(language) {
    if (this.state.selectedLanguage === language) return;
    
    this.state.selectedLanguage = language;
    this.updateActiveState();
    
    // Update select element
    if (this.select) {
      this.select.value = language;
    }
    
    // Update active filter display
    this.updateActiveDisplay();
    
    // Dispatch filter change event
    this.element.dispatchEvent(new CustomEvent('filter_change', {
      detail: { 
        language,
        languageData: this.getLanguageData(language)
      }
    }));
  }
  
  clearFilter() {
    this.selectLanguage('');
  }
  
  updateActiveState() {
    // Update item active states
    this.items.forEach(item => {
      const itemLanguage = item.dataset.language;
      const isActive = itemLanguage === this.state.selectedLanguage;
      
      if (isActive) {
        item.classList.add('language-filter-item--active');
      } else {
        item.classList.remove('language-filter-item--active');
      }
    });
  }
  
  updateActiveDisplay() {
    const activeDisplay = this.element.querySelector('.language-filter-active');
    
    if (this.state.selectedLanguage && activeDisplay) {
      const languageData = this.getLanguageData(this.state.selectedLanguage);
      const valueElement = activeDisplay.querySelector('.language-filter-active-value');
      
      if (valueElement && languageData) {
        valueElement.textContent = languageData.name;
        activeDisplay.style.display = 'flex';
      }
    } else if (activeDisplay) {
      activeDisplay.style.display = 'none';
    }
  }
  
  updateLanguages(languages) {
    this.state.languages = languages;
    
    // Update DOM based on layout
    if (this.layout === 'dropdown') {
      this.updateDropdownOptions(languages);
    } else {
      this.updateListItems(languages);
    }
    
    // Re-setup event listeners for new items
    this.items = this.element.querySelectorAll('[data-language]');
    this.setupEventListeners();
  }
  
  updateDropdownOptions(languages) {
    if (!this.select) return;
    
    // Clear existing options (except first if show_all)
    const firstOption = this.select.querySelector('option');
    this.select.innerHTML = '';
    
    if (firstOption) {
      this.select.appendChild(firstOption);
    }
    
    // Add new options
    languages.forEach(language => {
      const option = document.createElement('option');
      option.value = language.id;
      option.textContent = this.formatLanguageOption(language);
      option.selected = language.id === this.state.selectedLanguage;
      this.select.appendChild(option);
    });
  }
  
  updateListItems(languages) {
    if (!this.list) return;
    
    // Clear existing items
    this.list.innerHTML = '';
    
    // Add "All" item if needed
    const showAll = this.element.querySelector('[data-language=""]');
    if (showAll) {
      this.list.appendChild(showAll.cloneNode(true));
    }
    
    // Add language items
    languages.forEach(language => {
      const item = this.createLanguageItem(language);
      this.list.appendChild(item);
    });
  }
  
  createLanguageItem(language) {
    const item = document.createElement('button');
    item.className = 'language-filter-item';
    item.dataset.language = language.id;
    item.dataset.count = language.count || 0;
    
    if (language.id === this.state.selectedLanguage) {
      item.classList.add('language-filter-item--active');
    }
    
    let content = '';
    
    if (language.icon) {
      content += `<span class="language-filter-icon">${language.icon}</span>`;
    }
    
    content += `<span class="language-filter-name">${language.name}</span>`;
    
    const showCounts = this.element.querySelector('.language-filter-count') !== null;
    if (showCounts) {
      content += `<span class="language-filter-count">${language.count || 0}</span>`;
    }
    
    item.innerHTML = content;
    
    return item;
  }
  
  formatLanguageOption(language) {
    let text = language.name;
    
    const showCounts = this.element.querySelector('.language-filter-count') !== null;
    if (showCounts && language.count) {
      text += ` (${language.count})`;
    }
    
    return text;
  }
  
  getSelectedLanguage() {
    // Try select element first
    if (this.select) {
      return this.select.value;
    }
    
    // Check for active item
    const activeItem = this.element.querySelector('.language-filter-item--active');
    if (activeItem) {
      return activeItem.dataset.language || '';
    }
    
    // Check data attribute
    return this.element.dataset.selectedLanguage || '';
  }
  
  getLanguagesData() {
    const languages = [];
    
    this.items.forEach(item => {
      const language = item.dataset.language;
      const name = item.querySelector('.language-filter-name')?.textContent?.trim();
      const count = parseInt(item.dataset.count) || 0;
      const icon = item.querySelector('.language-filter-icon')?.textContent?.trim();
      
      if (language && name) {
        languages.push({ id: language, name, count, icon });
      }
    });
    
    return languages;
  }
  
  getLanguageData(languageId) {
    return this.state.languages.find(lang => lang.id === languageId) || null;
  }
  
  setLoading(isLoading) {
    this.state.isLoading = isLoading;
    
    if (isLoading) {
      this.element.classList.add('loading');
    } else {
      this.element.classList.remove('loading');
    }
  }
  
  // Public methods
  setLanguage(language) {
    this.selectLanguage(language);
  }
  
  getLanguage() {
    return this.state.selectedLanguage;
  }
  
  getLanguages() {
    return [...this.state.languages];
  }
  
  addLanguage(language) {
    this.state.languages.push(language);
    this.updateLanguages(this.state.languages);
  }
  
  removeLanguage(languageId) {
    this.state.languages = this.state.languages.filter(lang => lang.id !== languageId);
    this.updateLanguages(this.state.languages);
    
    // Clear filter if removed language was selected
    if (this.state.selectedLanguage === languageId) {
      this.clearFilter();
    }
  }
  
  updateLanguageCount(languageId, count) {
    const language = this.state.languages.find(lang => lang.id === languageId);
    if (language) {
      language.count = count;
      this.updateLanguages(this.state.languages);
    }
  }
  
  setLayout(layout) {
    this.layout = layout;
    this.element.dataset.layout = layout;
    this.updateLanguages(this.state.languages);
  }
  
  setVariant(variant) {
    this.variant = variant;
    this.element.dataset.variant = variant;
  }
  
  refresh() {
    // Re-initialize the component
    this.state.selectedLanguage = this.getSelectedLanguage();
    this.state.languages = this.getLanguagesData();
    this.updateActiveState();
    this.updateActiveDisplay();
  }
}

// Initialize language_filter components
window.PETA_COMPONENTS = window.PETA_COMPONENTS || {};
window.PETA_COMPONENTS.language_filter = (element, props = {}) => {
  return new LanguageFilterComponent(element);
};

// Auto-initialize
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('[data-component="language_filter"]').forEach(element => {
    new LanguageFilterComponent(element);
  });
});