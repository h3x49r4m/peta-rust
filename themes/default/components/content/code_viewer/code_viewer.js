class CodeViewerComponent {
  constructor(element) {
    this.element = element;
    this.theme = element.dataset.theme;
    this.language = element.dataset.language;
    this.content = element.querySelector('[data-content]');
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
    this.element.addEventListener('code_update', (e) => this.handleCodeUpdate(e));
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
    
    const lineNumbers = this.lineNumbers.querySelectorAll('.code-viewer-line-number');
    
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
      case 'download':
        this.downloadCode();
        break;
      case 'fullscreen':
        this.toggleFullscreen();
        break;
    }
    
    // Track action clicks
    this.element.dispatchEvent(new CustomEvent('code_action_click', {
      detail: { action }
    }));
  }
  
  handleCodeClick() {
    // Track code clicks
    this.element.dispatchEvent(new CustomEvent('code_click', {
      detail: { 
        language: this.language,
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
  
  handleCodeUpdate(event) {
    const { code, language } = event.detail;
    this.updateCode(code, language);
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
    this.element.dispatchEvent(new CustomEvent('code_copy', {
      detail: { 
        language: this.language,
        lines: this.getLineCount(),
        size: this.getCodeSize()
      }
    }));
  }
  
  downloadCode() {
    if (!this.codeElement) return;
    
    const codeText = this.codeElement.textContent;
    const filename = this.getFilename();
    const mimeType = this.getMimeType();
    
    const blob = new Blob([codeText], { type: mimeType });
    const url = URL.createObjectURL(blob);
    
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
    
    // Track download action
    this.element.dispatchEvent(new CustomEvent('code_download', {
      detail: { 
        filename,
        language: this.language,
        size: this.getCodeSize()
      }
    }));
    
    this.showToast(`Downloaded ${filename}`);
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
    this.element.dispatchEvent(new CustomEvent('fullscreen_enter'));
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
    this.element.dispatchEvent(new CustomEvent('fullscreen_exit'));
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
    const expandText = this.expandButton.querySelector('.code-viewer-expand-text');
    if (expandText) {
      expandText.textContent = this.state.isExpanded 
        ? 'Show less' 
        : `Show all ${this.getLineCount()} lines`;
    }
    
    // Track expansion toggle
    this.element.dispatchEvent(new CustomEvent('code_toggle_expand', {
      detail: { expanded: this.state.isExpanded }
    }));
  }
  
  highlightLine(lineNumber, temporary = false) {
    if (!this.lineNumbers) return;
    
    const lineNumbers = this.lineNumbers.querySelectorAll('.code-viewer-line-number');
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
    
    const lineNumbers = this.lineNumbers.querySelectorAll('.code-viewer-line-number');
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
  
  updateCode(code, language = null) {
    if (!this.codeElement) return;
    
    this.codeElement.textContent = code;
    
    if (language) {
      this.language = language;
      this.codeElement.className = `language-${language}`;
      this.element.dataset.language = language;
    }
    
    // Re-apply syntax highlighting
    this.setupCodeHighlighting();
    
    // Update line numbers
    this.updateLineNumbers();
    
    // Track code update
    this.element.dispatchEvent(new CustomEvent('code_updated', {
      detail: { 
        language: this.language,
        lines: this.getLineCount(),
        size: this.getCodeSize()
      }
    }));
  }
  
  updateLineNumbers() {
    if (!this.lineNumbers || !this.codeElement) return;
    
    const lines = this.codeElement.textContent.split('\n');
    const lineNumbersHtml = lines.map((_, index) => 
      `<span class="code-viewer-line-number">${index + 1}</span>`
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
  
  getFilename() {
    const filenameElement = this.element.querySelector('.code-viewer-filename');
    if (filenameElement) {
      return filenameElement.textContent.trim();
    }
    
    // Generate default filename
    const extensions = {
      javascript: 'js',
      python: 'py',
      rust: 'rs',
      html: 'html',
      css: 'css',
      json: 'json',
      xml: 'xml',
      yaml: 'yaml',
      markdown: 'md',
      typescript: 'ts'
    };
    
    const ext = extensions[this.language] || 'txt';
    return `code.${ext}`;
  }
  
  getMimeType() {
    const mimeTypes = {
      javascript: 'text/javascript',
      python: 'text/x-python',
      rust: 'text/x-rust',
      html: 'text/html',
      css: 'text/css',
      json: 'application/json',
      xml: 'application/xml',
      yaml: 'text/yaml',
      markdown: 'text/markdown',
      typescript: 'text/typescript'
    };
    
    return mimeTypes[this.language] || 'text/plain';
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
    toast.className = `code-viewer-toast code-viewer-toast--${type}`;
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
  setCode(code, language = null) {
    this.updateCode(code, language);
  }
  
  getCode() {
    return this.codeElement ? this.codeElement.textContent : '';
  }
  
  getLanguage() {
    return this.language;
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

// Initialize code_viewer components
window.PETA_COMPONENTS = window.PETA_COMPONENTS || {};
window.PETA_COMPONENTS.code_viewer = (element, props = {}) => {
  return new CodeViewerComponent(element);
};

// Auto-initialize
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('[data-component="code_viewer"]').forEach(element => {
    new CodeViewerComponent(element);
  });
});