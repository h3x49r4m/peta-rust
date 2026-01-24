class SnippetCardComponent {
  constructor(element) {
    this.element = element;
    this.size = element.dataset.size;
    this.titleLink = element.querySelector('.snippet-card-title-link');
    this.copyButton = element.querySelector('[data-copy]');
    this.codeContainer = element.querySelector('.snippet-card-code');
    this.actionButtons = element.querySelectorAll('.snippet-card-action');
    
    this.state = {
      isLoading: false,
      copySuccess: false,
      expandedCode: false
    };
    
    this.init();
  }
  
  init() {
    this.setupEventListeners();
    this.setupCodeHighlighting();
    this.setupCopyFunctionality();
  }
  
  setupEventListeners() {
    // Title link clicks
    if (this.titleLink) {
      this.titleLink.addEventListener('click', (e) => this.handleTitleClick(e));
    }
    
    // Action button clicks
    this.actionButtons.forEach(button => {
      button.addEventListener('click', (e) => this.handleActionClick(e, button));
    });
    
    // Copy button specific handling
    if (this.copyButton) {
      this.copyButton.addEventListener('click', (e) => this.handleCopyClick(e));
    }
    
    // Code container interactions
    if (this.codeContainer) {
      this.codeContainer.addEventListener('click', () => this.handleCodeClick());
    }
    
    // Custom events
    this.element.addEventListener('snippet_update', (e) => this.handleSnippetUpdate(e));
  }
  
  setupCodeHighlighting() {
    if (!this.codeContainer) return;
    
    // Apply syntax highlighting if Prism.js or similar is available
    if (typeof Prism !== 'undefined') {
      Prism.highlightElement(this.codeContainer);
    }
    
    // Add line numbers if needed
    this.addLineNumbers();
  }
  
  setupCopyFunctionality() {
    if (!this.copyButton) return;
    
    // Check if Clipboard API is available
    if (!navigator.clipboard) {
      this.copyButton.style.display = 'none';
    }
  }
  
  handleTitleClick(event) {
    // Track snippet title clicks
    const title = this.titleLink.textContent.trim();
    const url = this.titleLink.href;
    
    this.element.dispatchEvent(new CustomEvent('snippet_title_click', {
      detail: { title, url }
    }));
    
    // Add loading state
    this.setLoading(true);
    
    // Remove loading state after navigation
    setTimeout(() => {
      this.setLoading(false);
    }, 500);
  }
  
  handleActionClick(event, button) {
    const action = button.classList.contains('snippet-card-action--primary') ? 'primary' : 'secondary';
    const title = this.element.querySelector('.snippet-card-title').textContent.trim();
    
    // Track action button clicks
    this.element.dispatchEvent(new CustomEvent('snippet_action_click', {
      detail: { action, title, button: button.textContent.trim() }
    }));
    
    // Add loading state
    button.classList.add('loading');
    button.disabled = true;
    
    // Simulate action
    setTimeout(() => {
      button.classList.remove('loading');
      button.disabled = false;
      
      // Navigate if it's a primary action with href
      if (action === 'primary' && button.href) {
        window.location.href = button.href;
      }
    }, 800);
  }
  
  handleCopyClick(event) {
    event.preventDefault();
    event.stopPropagation();
    
    if (this.state.copySuccess) return;
    
    const copyUrl = this.copyButton.dataset.url;
    const codeText = this.codeContainer?.textContent;
    const textToCopy = copyUrl || codeText;
    
    if (!textToCopy) return;
    
    // Copy to clipboard
    navigator.clipboard.writeText(textToCopy).then(() => {
      this.setCopySuccess(true);
      
      // Track copy action
      this.element.dispatchEvent(new CustomEvent('snippet_copy', {
        detail: { 
          type: copyUrl ? 'url' : 'code',
          content: textToCopy.substring(0, 50) + (textToCopy.length > 50 ? '...' : '')
        }
      }));
      
      // Reset copy success state after 2 seconds
      setTimeout(() => {
        this.setCopySuccess(false);
      }, 2000);
    }).catch(err => {
      console.error('Failed to copy text: ', err);
      
      // Fallback for older browsers
      this.fallbackCopyToClipboard(textToCopy);
    });
  }
  
  handleCodeClick() {
    // Track code container clicks
    this.element.dispatchEvent(new CustomEvent('snippet_code_click', {
      detail: { 
        language: this.getLanguage(),
        lines: this.getLineCount()
      }
    }));
    
    // Toggle code expansion if there are many lines
    const previewMore = this.element.querySelector('.snippet-card-preview-more');
    if (previewMore) {
      this.toggleCodeExpansion();
    }
  }
  
  handleSnippetUpdate(event) {
    const { snippet } = event.detail;
    this.updateSnippetInfo(snippet);
  }
  
  setCopySuccess(success) {
    this.state.copySuccess = success;
    
    if (success) {
      this.element.dataset.copySuccess = 'true';
      this.copyButton.classList.add('copied');
      
      // Update button text
      const originalText = this.copyButton.textContent;
      this.copyButton.textContent = 'Copied!';
      
      setTimeout(() => {
        this.copyButton.textContent = originalText;
      }, 2000);
    } else {
      delete this.element.dataset.copySuccess;
      this.copyButton.classList.remove('copied');
    }
  }
  
  setLoading(isLoading) {
    this.state.isLoading = isLoading;
    
    if (isLoading) {
      this.element.classList.add('loading');
    } else {
      this.element.classList.remove('loading');
    }
  }
  
  toggleCodeExpansion() {
    this.state.expandedCode = !this.state.expandedCode;
    
    const previewMore = this.element.querySelector('.snippet-card-preview-more');
    const code = this.codeContainer?.textContent;
    
    if (this.state.expandedCode && code) {
      // Show full code
      this.codeContainer.textContent = code;
      previewMore.style.display = 'none';
      
      // Re-apply syntax highlighting
      if (typeof Prism !== 'undefined') {
        Prism.highlightElement(this.codeContainer);
      }
    } else {
      // Restore truncated preview
      location.reload(); // Simple way to reset to original state
    }
    
    // Track expansion toggle
    this.element.dispatchEvent(new CustomEvent('snippet_code_toggle', {
      detail: { expanded: this.state.expandedCode }
    }));
  }
  
  updateSnippetInfo(snippet) {
    // Update title
    const titleElement = this.element.querySelector('.snippet-card-title-link');
    if (titleElement && snippet.title) {
      titleElement.textContent = snippet.title;
      titleElement.href = snippet.url || '#';
    }
    
    // Update description
    const descriptionElement = this.element.querySelector('.snippet-card-description');
    if (descriptionElement && snippet.description) {
      descriptionElement.textContent = this.truncateText(snippet.description, 120);
    }
    
    // Update language
    const languageElements = this.element.querySelectorAll('.snippet-card-language-badge, .snippet-card-code-language');
    languageElements.forEach(element => {
      if (snippet.language) {
        element.textContent = snippet.language;
      }
    });
    
    // Update code
    if (this.codeContainer && snippet.code) {
      this.codeContainer.textContent = snippet.code;
      this.setupCodeHighlighting();
    }
  }
  
  addLineNumbers() {
    if (!this.codeContainer) return;
    
    const lines = this.codeContainer.textContent.split('\n');
    const lineNumbers = lines.map((_, index) => index + 1).join('\n');
    
    // Create line numbers container
    const lineNumbersContainer = document.createElement('span');
    lineNumbersContainer.className = 'line-numbers';
    lineNumbersContainer.textContent = lineNumbers;
    lineNumbersContainer.style.cssText = `
      display: inline-block;
      width: 30px;
      margin-right: 10px;
      color: var(--code-comment);
      user-select: none;
      text-align: right;
    `;
    
    // Wrap code with line numbers
    const wrapper = document.createElement('div');
    wrapper.style.display = 'flex';
    wrapper.appendChild(lineNumbersContainer);
    wrapper.appendChild(this.codeContainer);
    
    this.codeContainer.parentNode.replaceChild(wrapper, this.codeContainer);
  }
  
  getLanguage() {
    const languageElement = this.element.querySelector('.snippet-card-language-badge');
    return languageElement ? languageElement.textContent.trim() : 'unknown';
  }
  
  getLineCount() {
    if (!this.codeContainer) return 0;
    return this.codeContainer.textContent.split('\n').length;
  }
  
  truncateText(text, maxLength) {
    if (text.length <= maxLength) return text;
    return text.substring(0, maxLength).trim() + '...';
  }
  
  fallbackCopyToClipboard(text) {
    const textArea = document.createElement('textarea');
    textArea.value = text;
    textArea.style.position = 'fixed';
    textArea.style.left = '-999999px';
    textArea.style.top = '-999999px';
    document.body.appendChild(textArea);
    textArea.focus();
    textArea.select();
    
    try {
      document.execCommand('copy');
      this.setCopySuccess(true);
    } catch (err) {
      console.error('Fallback: Oops, unable to copy', err);
    }
    
    document.body.removeChild(textArea);
  }
  
  // Public methods
  setLanguage(language) {
    const languageElements = this.element.querySelectorAll('.snippet-card-language-badge, .snippet-card-code-language');
    languageElements.forEach(element => {
      element.textContent = language;
    });
    
    // Update code class for syntax highlighting
    if (this.codeContainer) {
      this.codeContainer.className = `language-${language}`;
      this.setupCodeHighlighting();
    }
  }
  
  setSize(size) {
    this.size = size;
    this.element.dataset.size = size;
  }
  
  refresh() {
    // Re-initialize the component
    this.setupCodeHighlighting();
    this.setupCopyFunctionality();
  }
}

// Initialize snippet_card components
window.PETA_COMPONENTS = window.PETA_COMPONENTS || {};
window.PETA_COMPONENTS.snippet_card = (element, props = {}) => {
  return new SnippetCardComponent(element);
};

// Auto-initialize
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('[data-component="snippet_card"]').forEach(element => {
    new SnippetCardComponent(element);
  });
});