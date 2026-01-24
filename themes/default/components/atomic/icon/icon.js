class IconComponent {
  constructor(element) {
    this.element = element;
    this.iconName = element.dataset.iconName;
    this.init();
  }
  
  init() {
    // Add click handler if clickable
    if (this.element.classList.contains('icon-clickable')) {
      this.element.addEventListener('click', (e) => this.handleClick(e));
    }
    
    // Dispatch ready event
    this.element.dispatchEvent(new CustomEvent('icon:ready', {
      detail: { name: this.iconName }
    }));
  }
  
  handleClick(event) {
    this.element.dispatchEvent(new CustomEvent('icon:click', {
      detail: { name: this.iconName, event }
    }));
  }
  
  // Public API
  setIcon(name) {
    this.iconName = name;
    this.element.dataset.iconName = name;
    // In a real implementation, this would update the icon content
    this.element.dispatchEvent(new CustomEvent('icon:change', {
      detail: { name }
    }));
  }
  
  setSize(size) {
    const sizes = ['xs', 'sm', 'md', 'lg', 'xl', '2xl', '3xl'];
    sizes.forEach(s => this.element.classList.remove(`icon-${s}`));
    this.element.classList.add(`icon-${size}`);
  }
  
  setColor(color) {
    this.element.style.color = color;
  }
  
  addAnimation(animation) {
    this.element.classList.add(`icon-${animation}`);
  }
  
  removeAnimation(animation) {
    this.element.classList.remove(`icon-${animation}`);
  }
}

// Auto-initialize icon components
document.addEventListener('DOMContentLoaded', () => {
  const iconElements = document.querySelectorAll('.icon');
  iconElements.forEach(element => {
    new IconComponent(element);
  });
});

// Export for manual initialization
window.IconComponent = IconComponent;