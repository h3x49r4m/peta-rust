class SnippetEmbedComponent {
  constructor(element) {
    this.element = element;
    this.theme = element.dataset.theme;
    this.source = element.dataset.source;
    this.content = element.querySelector('.snippet-embed-content');
    this.codeContainer = element.querySelector('[data-code-container]');
    this.codeElement = element.querySelector('[data-code]');
    this.lineNumbers = element.querySelector('[data-line-numbers]');
    this.expandButton = element.querySelector('[data-expand]');
    this.actionButtons = element.querySelectorAll('[data-action]');
    
    this.state = {
      isFullscreen: false,
      isExpanded: false,
      copySuccess: false,
      isLoading: false
    };
    
    this.init();
  }
  
  init() {
    this.setupEventListeners();
    this.setupCodeHighlighting();
    this.setupKeyboardShortcuts();
    this.initializeTheme();
  }
  
  setupEventListeners() {
    // Action button clicks
    this.actionButtons.forEach(button => {
      button.addEventListener('click', (e) => this.handleActionClick(e, button));
    });
    
    // Expand button click
    if (this.expandButton) {
      this.expandButton.addEventListener('click', () => this.toggleExpansion());
    }
    
    // Code container interactions
    if (this.codeElement) {
      this.codeElement.addEventListener('click', () => this.handleCodeClick());
      this.codeElement.addEventListener('scroll', () => this.syncLineNumbers());
    }
    
    // Window resize for fullscreen
    window.addEventListener('resize', () => this.handleResize());
    
    // Escape key for fullscreen
    document.addEventListener('keydown', (e) => this.handleKeydown(e));
    
    // Custom events
    this.element.addEventListener('snippet_update', (e) => this.handleSnippetUpdate(e));
    this.element.addEventListener('theme_change', (e) => this.handleThemeChange(e));
  }
  
  setupCodeHighlighting() {
    if (!this.codeElement) return;
    
    // Apply syntax highlighting if Prism.js or similar is available
    if (typeof Prism !== 'undefined') {
      Prism.highlightElement(this.codeElement);
    }
    
    // Apply highlight.js if available
    if (typeof hljs !== 'undefined') {
      hljs.highlightElement(this.codeElement);
    }
    
    // Add line number hover effects
    this.setupLineNumberInteractions();
  }
  
  setupKeyboardShortcuts() {
    // Keyboard shortcuts are handled in handleKeydown
    this.shortcuts = {
      'ctrl+c': () => this.copyCode(),
      'cmd+c': () => this.copyCode(),
      'ctrl+f': () => this.toggleFullscreen(),
      'cmd+f': () => this.toggleFullscreen(),
      'escape': () => this.exitFullscreen()
    };
  }
  
  initializeTheme() {
    if (this.theme === 'auto') {
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      this.setTheme(prefersDark ? 'dark' : 'light');
    }
  }
  
  setupLineNumberInteractions() {
    if (!this.lineNumbers) return;
    
    const lineNumbers = this.lineNumbers.querySelectorAll('.snippet-embed-line-number');
    
    lineNumbers.forEach((lineNumber, index) => {
      lineNumber.addEventListener('click', () => {
        this.highlightLine(index + 1);
      });
      
      lineNumber.addEventListener('mouseenter', () => {
        this.highlightLine(index + 1, true);
      });
      
      lineNumber.addEventListener('mouseleave', () => {
        this.clearLineHighlight();
      });
    });
  }
  
  handleActionClick(event, button) {
    const action = button.dataset.action;
    
    switch (action) {
      case 'copy':
        this.copyCode();
        break;
      case 'fullscreen':
        this.toggleFullscreen();
        break;
    }
    
    // Track action clicks
    this.element.dispatchEvent(new CustomEvent('embed_action_click', {
      detail: { action, source: this.source }
    }));
  }
  
  handleCodeClick() {
    // Track code clicks
    this.element.dispatchEvent(new CustomEvent('embed_code_click', {
      detail: { 
        source: this.source,
        lines: this.getLineCount(),
        size: this.getCodeSize()
      }
    }));
  }
  
  handleResize() {
    if (this.state.isFullscreen) {
      // Adjust fullscreen layout
      this.codeContainer.style.maxHeight = 'none';
    }
  }
  
  handleKeydown(event) {
    // Handle escape key for fullscreen
    if (event.key === 'Escape' && this.state.isFullscreen) {
      this.exitFullscreen();
      return;
    }
    
    // Handle keyboard shortcuts
    const key = [];
    if (event.ctrlKey) key.push('ctrl');
    if (event.metaKey) key.push('cmd');
    if (event.key) key.push(event.key.toLowerCase());
    
    const shortcut = key.join('+');
    if (this.shortcuts[shortcut]) {
      event.preventDefault();
      this.shortcuts[shortcut]();
    }
  }
  
  handleSnippetUpdate(event) {
    const { snippet } = event.detail;
    this.updateSnippet(snippet);
  }
  
  handleThemeChange(event) {
    const { theme } = event.detail;
    this.setTheme(theme);
  }
  
  copyCode() {
    if (!this.codeElement) return;
    
    const codeText = this.codeElement.textContent;
    
    if (navigator.clipboard) {
      navigator.clipboard.writeText(codeText).then(() => {
        this.setCopySuccess(true);
        this.showToast('Code copied to clipboard!');
      }).catch(err => {
        console.error('Failed to copy code: ', err);
        this.fallbackCopyToClipboard(codeText);
      });
    } else {
      this.fallbackCopyToClipboard(codeText);
    }
    
    // Track copy action
    this.element.dispatchEvent(new CustomEvent('embed_copy', {
      detail: { 
        source: this.source,
        lines: this.getLineCount(),
        size: this.getCodeSize()
      }
    }));
  }
  
  toggleFullscreen() {
    if (this.state.isFullscreen) {
      this.exitFullscreen();
    } else {
      this.enterFullscreen();
    }
  }
  
  enterFullscreen() {
    this.state.isFullscreen = true;
    this.element.dataset.fullscreen = 'true';
    document.body.style.overflow = 'hidden';
    
    // Update fullscreen button
    const fullscreenBtn = this.element.querySelector('[data-action="fullscreen"]');
    if (fullscreenBtn) {
      fullscreenBtn.innerHTML = `
        {% component "icon" with name="minimize" library="feather" size="sm" %}
        {% endslot %}
      `;
    }
    
    // Track fullscreen enter
    this.element.dispatchEvent(new CustomEvent('embed_fullscreen_enter', {
      detail: { source: this.source }
    }));
  }
  
  exitFullscreen() {
    this.state.isFullscreen = false;
    delete this.element.dataset.fullscreen;
    document.body.style.overflow = '';
    
    // Update fullscreen button
    const fullscreenBtn = this.element.querySelector('[data-action="fullscreen"]');
    if (fullscreenBtn) {
      fullscreenBtn.innerHTML = `
        {% component "icon" with name="maximize" library="feather" size="sm" %}
        {% endslot %}
      `;
    }
    
    // Track fullscreen exit
    this.element.dispatchEvent(new CustomEvent('embed_fullscreen_exit', {
      detail: { source: this.source }
    }));
  }
  
  toggleExpansion() {
    this.state.isExpanded = !this.state.isExpanded;
    
    if (this.state.isExpanded) {
      this.codeContainer.style.maxHeight = 'none';
      this.expandButton.style.display = 'none';
    } else {
      this.codeContainer.style.maxHeight = '';
      this.expandButton.style.display = '';
    }
    
    // Update expand button text
    const expandText = this.expandButton.querySelector('.snippet-embed-expand-text');
    if (expandText) {
      expandText.textContent = this.state.isExpanded 
        ? 'Show less' 
        : `Show all ${this.getLineCount()} lines`;
    }
    
    // Track expansion toggle
    this.element.dispatchEvent(new CustomEvent('embed_toggle_expand', {
      detail: { 
        source: this.source,
        expanded: this.state.isExpanded
      }
    }));
  }
  
  highlightLine(lineNumber, temporary = false) {
    if (!this.lineNumbers) return;
    
    const lineNumbers = this.lineNumbers.querySelectorAll('.snippet-embed-line-number');
    const targetLine = lineNumbers[lineNumber - 1];
    
    if (targetLine) {
      if (temporary) {
        targetLine.classList.add('highlighted');
      } else {
        // Clear previous highlights
        this.clearLineHighlight();
        targetLine.classList.add('highlighted');
      }
    }
  }
  
  clearLineHighlight() {
    if (!this.lineNumbers) return;
    
    const lineNumbers = this.lineNumbers.querySelectorAll('.snippet-embed-line-number');
    lineNumbers.forEach(line => line.classList.remove('highlighted'));
  }
  
  syncLineNumbers() {
    if (!this.lineNumbers || !this.codeElement) return;
    
    this.lineNumbers.scrollTop = this.codeElement.scrollTop;
  }
  
  setCopySuccess(success) {
    this.state.copySuccess = success;
    
    const copyButton = this.element.querySelector('[data-action="copy"]');
    if (copyButton) {
      if (success) {
        copyButton.classList.add('copied');
        setTimeout(() => {
          copyButton.classList.remove('copied');
        }, 2000);
      }
    }
  }
  
  setTheme(theme) {
    this.theme = theme;
    this.element.dataset.theme = theme;
  }
  
  updateSnippet(snippet) {
    if (!this.codeElement) return;
    
    // Update title
    const titleElement = this.element.querySelector('.snippet-embed-title-link');
    if (titleElement && snippet.title) {
      titleElement.textContent = snippet.title;
      titleElement.href = snippet.url || '#';
    }
    
    // Update code
    this.codeElement.textContent = snippet.content || '';
    
    // Update language
    if (snippet.language) {
      this.codeElement.className = `language-${snippet.language}`;
      const languageElement = this.element.querySelector('.snippet-embed-language');
      if (languageElement) {
        languageElement.textContent = snippet.language;
      }
    }
    
    // Update source
    if (snippet.source) {
      this.source = snippet.source;
      this.element.dataset.source = snippet.source;
      const sourceElement = this.element.querySelector('.snippet-embed-source');
      if (sourceElement) {
        sourceElement.lastChild.textContent = snippet.source;
      }
    }
    
    // Re-apply syntax highlighting
    this.setupCodeHighlighting();
    
    // Update line numbers
    this.updateLineNumbers();
    
    // Track snippet update
    this.element.dispatchEvent(new CustomEvent('embed_updated', {
      detail: { 
        source: this.source,
        lines: this.getLineCount(),
        size: this.getCodeSize()
      }
    }));
  }
  
  updateLineNumbers() {
    if (!this.lineNumbers || !this.codeElement) return;
    
    const lines = this.codeElement.textContent.split('\n');
    const lineNumbersHtml = lines.map((_, index) => 
      `<span class="snippet-embed-line-number">${index + 1}</span>`
    ).join('');
    
    this.lineNumbers.innerHTML = lineNumbersHtml;
    this.setupLineNumberInteractions();
  }
  
  getLineCount() {
    if (!this.codeElement) return 0;
    return this.codeElement.textContent.split('\n').length;
  }
  
  getCodeSize() {
    if (!this.codeElement) return 0;
    return new Blob([this.codeElement.textContent]).size;
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
      this.showToast('Code copied to clipboard!');
    } catch (err) {
      console.error('Fallback: Oops, unable to copy', err);
      this.showToast('Failed to copy code', 'error');
    }
    
    document.body.removeChild(textArea);
  }
  
  showToast(message, type = 'success') {
    // Create toast element
    const toast = document.createElement('div');
    toast.className = `snippet-embed-toast snippet-embed-toast--${type}`;
    toast.textContent = message;
    toast.style.cssText = `
      position: fixed;
      bottom: 20px;
      right: 20px;
      background: ${type === 'success' ? 'var(--color-green-500)' : 'var(--color-red-500)'};
      color: white;
      padding: 12px 16px;
      border-radius: 6px;
      font-size: 14px;
      z-index: 1001;
      opacity: 0;
      transform: translateY(20px);
      transition: all 0.3s ease;
    `;
    
    document.body.appendChild(toast);
    
    // Animate in
    setTimeout(() => {
      toast.style.opacity = '1';
      toast.style.transform = 'translateY(0)';
    }, 10);
    
    // Remove after 3 seconds
    setTimeout(() => {
      toast.style.opacity = '0';
      toast.style.transform = 'translateY(20px)';
      setTimeout(() => {
        document.body.removeChild(toast);
      }, 300);
    }, 3000);
  }
  
  // Public methods
  setSnippet(snippet) {
    this.updateSnippet(snippet);
  }
  
  getSnippet() {
    return {
      content: this.codeElement ? this.codeElement.textContent : '',
      source: this.source,
      lines: this.getLineCount(),
      size: this.getCodeSize()
    };
  }
  
  getSource() {
    return this.source;
  }
  
  getTheme() {
    return this.theme;
  }
  
  refresh() {
    // Re-initialize the component
    this.setupCodeHighlighting();
    this.updateLineNumbers();
  }
}

// Initialize snippet_embed components
window.PETA_COMPONENTS = window.PETA_COMPONENTS || {};
window.PETA_COMPONENTS.snippet_embed = (element, props = {}) => {
  return new SnippetEmbedComponent(element);
};

// Auto-initialize
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('[data-component="snippet_embed"]').forEach(element => {
    new SnippetEmbedComponent(element);
  });
});