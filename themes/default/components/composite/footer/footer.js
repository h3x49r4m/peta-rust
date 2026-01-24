class FooterComponent {
  constructor(element) {
    this.element = element;
    this.socialLinks = element.querySelectorAll('.footer-social-link');
    this.currentYear = new Date().getFullYear();
    
    this.init();
  }
  
  init() {
    this.setupEventListeners();
    this.updateCopyrightYear();
  }
  
  setupEventListeners() {
    // Add hover effects to social links
    this.socialLinks.forEach(link => {
      link.addEventListener('mouseenter', () => this.handleSocialHover(link, true));
      link.addEventListener('mouseleave', () => this.handleSocialHover(link, false));
    });
    
    // Handle external links
    const externalLinks = this.element.querySelectorAll('a[href^="http"]');
    externalLinks.forEach(link => {
      link.addEventListener('click', (e) => this.handleExternalLink(e, link));
    });
  }
  
  handleSocialHover(link, isHovering) {
    if (isHovering) {
      link.style.transform = 'translateY(-2px) scale(1.05)';
    } else {
      link.style.transform = '';
    }
  }
  
  handleExternalLink(event, link) {
    // Add external link indicator or open in new tab
    if (!link.target) {
      link.target = '_blank';
      link.rel = 'noopener noreferrer';
    }
  }
  
  updateCopyrightYear() {
    const copyrightElement = this.element.querySelector('.footer-copyright');
    if (copyrightElement) {
      const currentText = copyrightElement.textContent;
      const yearRegex = /\d{4}/;
      const updatedText = currentText.replace(yearRegex, this.currentYear);
      copyrightElement.textContent = updatedText;
    }
  }
  
  // Public methods
  updateSocialLinks(links) {
    const socialContainer = this.element.querySelector('.footer-social');
    if (!socialContainer) return;
    
    socialContainer.innerHTML = links.map(link => `
      <a href="${link.url}" class="footer-social-link" aria-label="${link.label}" target="_blank" rel="noopener noreferrer">
        ${this.getIconSvg(link.icon)}
      </a>
    `).join('');
    
    // Reinitialize event listeners for new links
    this.socialLinks = socialContainer.querySelectorAll('.footer-social-link');
    this.setupEventListeners();
  }
  
  getIconSvg(iconName) {
    const icons = {
      github: `
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 1-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.7 4.86 3.39 3.39 0 0 0 1.18-2.42c-1.84.92-3.51 1.38-5.5 1.38m-11.61 0L9 19m-4.5-8.5a3.37 3.37 0 0 1-2.61 3.14c-.35 3.14-1.54 6.44-7 6.44A5.44 5.44 0 0 0 4.77 20a5.07 5.07 0 0 0 4.86-19.7 3.39 3.39 0 0 0 2.42-1.18c.92-1.84 1.38-3.51 1.38-5.5"></path>
        </svg>
      `,
      twitter: `
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M23 3a10.9 10.9 0 0 1-3.14 1.53 4.48 4.48 0 0 0-4.2-5.05 4.48 4.48 0 0 0-5.05-4.2A10.9 10.9 0 0 1 3 1m13.5 0A10.9 10.9 0 0 1 21 3m-6 0a10.9 10.9 0 0 1-3.14 1.53 4.48 4.48 0 0 0-4.2-5.05 4.48 4.48 0 0 0-5.05-4.2A10.9 10.9 0 0 1 13.5 3"></path>
        </svg>
      `,
      discord: `
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10"></circle>
          <path d="M8 14s1.5 2 4 2 4-2 4-2 1.5-2 4-2z"></path>
          <line x1="9" y1="9" x2="9.01" y2="9"></line>
          <line x1="15" y1="9" x2="15.01" y2="9"></line>
        </svg>
      `,
      linkedin: `
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M16 8a6 6 0 0 1 6 6v7a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a6 6 0 0 1 6-6z"></path>
          <rect x="2" y="9" width="4" height="12"></rect>
          <rect x="18" y="9" width="4" height="12"></rect>
        </svg>
      `
    };
    
    return icons[iconName] || icons.github;
  }
}

// Initialize footer components
window.PETA_COMPONENTS = window.PETA_COMPONENTS || {};
window.PETA_COMPONENTS.footer = (element, props = {}) => {
  return new FooterComponent(element);
};

// Auto-initialize
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('[data-component="footer"]').forEach(element => {
    new FooterComponent(element);
  });
});