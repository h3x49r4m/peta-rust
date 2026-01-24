class InputComponent {
  constructor(element) {
    this.element = element;
    this.input = element.querySelector('.input-field');
    this.state = {
      focused: false,
      dirty: false
    };
    
    this.init();
  }
  
  init() {
    // Add event listeners
    this.input.addEventListener('focus', () => this.handleFocus());
    this.input.addEventListener('blur', () => this.handleBlur());
    this.input.addEventListener('input', () => this.handleInput());
    
    // Initialize state
    this.updateClasses();
  }
  
  handleFocus() {
    this.state.focused = true;
    this.updateClasses();
    this.element.dispatchEvent(new CustomEvent('input:focus', { 
      detail: { value: this.input.value } 
    }));
  }
  
  handleBlur() {
    this.state.focused = false;
    this.updateClasses();
    this.element.dispatchEvent(new CustomEvent('input:blur', { 
      detail: { value: this.input.value } 
    }));
  }
  
  handleInput() {
    this.state.dirty = true;
    this.updateClasses();
    this.element.dispatchEvent(new CustomEvent('input:change', { 
      detail: { value: this.input.value } 
    }));
  }
  
  updateClasses() {
    if (this.state.focused) {
      this.element.classList.add('input-focused');
    } else {
      this.element.classList.remove('input-focused');
    }
    
    if (this.state.dirty) {
      this.element.classList.add('input-dirty');
    } else {
      this.element.classList.remove('input-dirty');
    }
  }
  
  // Public API
  getValue() {
    return this.input.value;
  }
  
  setValue(value) {
    this.input.value = value;
    this.state.dirty = true;
    this.updateClasses();
  }
  
  setError(error) {
    const errorElement = this.element.querySelector('.input-error-message');
    if (errorElement) {
      errorElement.textContent = error;
    }
    
    if (error) {
      this.element.classList.add('input-error');
    } else {
      this.element.classList.remove('input-error');
    }
  }
  
  focus() {
    this.input.focus();
  }
  
  blur() {
    this.input.blur();
  }
}

// Auto-initialize input components
document.addEventListener('DOMContentLoaded', () => {
  const inputElements = document.querySelectorAll('.input-wrapper');
  inputElements.forEach(element => {
    new InputComponent(element);
  });
});

// Export for manual initialization
window.InputComponent = InputComponent;