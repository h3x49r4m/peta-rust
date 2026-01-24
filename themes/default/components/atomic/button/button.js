/**
 * Button Component JavaScript
 * Handles button interactions, loading states, and accessibility
 */

class ButtonComponent {
  constructor(element) {
    this.element = element;
    this.isLoading = false;
    this.isDisabled = element.hasAttribute('disabled');
    
    this.init();
  }
  
  init() {
    // Add event listeners
    this.bindEvents();
    
    // Initialize state
    this.updateState();
    
    // Handle initial loading state
    if (this.element.querySelector('.btn-loading')) {
      this.setLoading(true);
    }
  }
  
  bindEvents() {
    // Click handler
    this.element.addEventListener('click', this.handleClick.bind(this));
    
    // Focus/blur handlers
    this.element.addEventListener('focus', this.handleFocus.bind(this));
    this.element.addEventListener('blur', this.handleBlur.bind(this));
    
    // Keyboard support
    this.element.addEventListener('keydown', this.handleKeyDown.bind(this));
    
    // Monitor disabled state changes
    this.observer = new MutationObserver(this.handleDisabledChange.bind(this));
    this.observer.observe(this.element, {
      attributes: true,
      attributeFilter: ['disabled']
    });
  }
  
  handleClick(event) {
    // Prevent click if disabled or loading
    if (this.isDisabled || this.isLoading) {
      event.preventDefault();
      return;
    }
    
    // Add ripple effect
    this.createRipple(event);
    
    // Trigger custom event
    this.element.dispatchEvent(new CustomEvent('button:click', {
      detail: {
        originalEvent: event,
        component: this
      }
    }));
  }
  
  handleFocus() {
    this.element.classList.add('focused');
    
    // Announce to screen readers
    this.announceToScreenReader('Button focused');
  }
  
  handleBlur() {
    this.element.classList.remove('focused');
  }
  
  handleKeyDown(event) {
    // Handle Enter and Space for accessibility
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      this.element.click();
    }
  }
  
  handleDisabledChange(mutations) {
    mutations.forEach(mutation => {
      if (mutation.attributeName === 'disabled') {
        this.isDisabled = this.element.hasAttribute('disabled');
        this.updateState();
      }
    });
  }
  
  setLoading(loading) {
    this.isLoading = loading;
    this.updateState();
    
    // Update ARIA attributes
    if (loading) {
      this.element.setAttribute('aria-busy', 'true');
      this.element.setAttribute('disabled', '');
    } else {
      this.element.removeAttribute('aria-busy');
      if (!this.isDisabled) {
        this.element.removeAttribute('disabled');
      }
    }
  }
  
  updateState() {
    // Update loading state
    const loadingElement = this.element.querySelector('.btn-loading');
    if (this.isLoading && !loadingElement) {
      this.showLoadingIndicator();
    } else if (!this.isLoading && loadingElement) {
      this.hideLoadingIndicator();
    }
    
    // Update disabled state
    if (this.isDisabled) {
      this.element.setAttribute('aria-disabled', 'true');
    } else {
      this.element.removeAttribute('aria-disabled');
    }
  }
  
  showLoadingIndicator() {
    const textElement = this.element.querySelector('.btn-text');
    const loadingHTML = `
      <span class="btn-loading">
        <span class="btn-spinner"></span>
        <span class="btn-text">${textElement ? textElement.textContent : 'Loading...'}</span>
      </span>
    `;
    
    if (textElement) {
      textElement.parentNode.innerHTML = loadingHTML;
    } else {
      this.element.innerHTML = loadingHTML;
    }
  }
  
  hideLoadingIndicator() {
    const loadingElement = this.element.querySelector('.btn-loading');
    if (loadingElement) {
      const textElement = loadingElement.querySelector('.btn-text');
      if (textElement) {
        // Restore original text
        const originalText = textElement.textContent;
        loadingElement.parentNode.innerHTML = `<span class="btn-text">${originalText}</span>`;
      }
    }
  }
  
  createRipple(event) {
    const ripple = document.createElement('span');
    ripple.className = 'btn-ripple';
    
    // Calculate ripple size and position
    const rect = this.element.getBoundingClientRect();
    const size = Math.max(rect.width, rect.height);
    const x = event.clientX - rect.left - size / 2;
    const y = event.clientY - rect.top - size / 2;
    
    ripple.style.width = ripple.style.height = size + 'px';
    ripple.style.left = x + 'px';
    ripple.style.top = y + 'px';
    
    // Add ripple to button
    this.element.appendChild(ripple);
    
    // Remove ripple after animation
    setTimeout(() => {
      if (ripple.parentNode) {
        ripple.parentNode.removeChild(ripple);
      }
    }, 600);
  }
  
  announceToScreenReader(message) {
    const announcement = document.createElement('div');
    announcement.setAttribute('aria-live', 'polite');
    announcement.setAttribute('aria-atomic', 'true');
    announcement.className = 'sr-only';
    announcement.textContent = message;
    
    document.body.appendChild(announcement);
    
    setTimeout(() => {
      if (announcement.parentNode) {
        announcement.parentNode.removeChild(announcement);
      }
    }, 1000);
  }
  
  destroy() {
    // Clean up event listeners
    this.element.removeEventListener('click', this.handleClick);
    this.element.removeEventListener('focus', this.handleFocus);
    this.element.removeEventListener('blur', this.handleBlur);
    this.element.removeEventListener('keydown', this.handleKeyDown);
    
    // Clean up observer
    if (this.observer) {
      this.observer.disconnect();
    }
    
    // Remove ripple elements
    const ripples = this.element.querySelectorAll('.btn-ripple');
    ripples.forEach(ripple => ripple.remove());
  }
}

// Initialize button components
document.addEventListener('DOMContentLoaded', () => {
  const buttons = document.querySelectorAll('[data-component="button"]');
  
  buttons.forEach(element => {
    // Store component instance on element
    element.buttonComponent = new ButtonComponent(element);
  });
});

// Export for external use
if (typeof window !== 'undefined') {
  window.ButtonComponent = ButtonComponent;
}

/* Add screen reader only styles */
const style = document.createElement('style');
style.textContent = `
  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border-width: 0;
  }
  
  .btn-ripple {
    position: absolute;
    border-radius: 50%;
    background-color: rgba(255, 255, 255, 0.6);
    transform: scale(0);
    animation: ripple 0.6s ease-out;
    pointer-events: none;
  }
  
  @keyframes ripple {
    to {
      transform: scale(4);
      opacity: 0;
    }
  }
`;
document.head.appendChild(style);