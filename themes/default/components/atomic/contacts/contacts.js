class ContactsComponent {
  constructor(element) {
    this.element = element;
    this.init();
  }
  
  init() {
    // Initialize component
    console.log('Contacts component initialized');
    
    // Add event listeners
    this.bindEvents();
    
    // Initialize social links if needed
    this.initSocialLinks();
  }
  
  bindEvents() {
    // Add any event listeners here
    const socialLinks = this.element.querySelectorAll('.social-link');
    socialLinks.forEach(link => {
      link.addEventListener('click', this.handleSocialLinkClick.bind(this));
    });
  }
  
  initSocialLinks() {
    // Add any initialization for social links
    const socialLinks = this.element.querySelectorAll('.social-link');
    socialLinks.forEach(link => {
      // Add external link indicator
      if (link.hostname !== window.location.hostname) {
        link.setAttribute('target', '_blank');
        link.setAttribute('rel', 'noopener noreferrer');
      }
    });
  }
  
  handleSocialLinkClick(event) {
    // Handle social link clicks
    const link = event.currentTarget;
    const url = link.getAttribute('href');
    
    // Track analytics if needed
    if (typeof gtag !== 'undefined') {
      gtag('event', 'social_link_click', {
        'event_category': 'contacts',
        'event_label': url
      });
    }
  }
  
  destroy() {
    // Clean up component
    console.log('Contacts component destroyed');
    
    // Remove event listeners
    const socialLinks = this.element.querySelectorAll('.social-link');
    socialLinks.forEach(link => {
      link.removeEventListener('click', this.handleSocialLinkClick);
    });
  }
}

// Initialize components
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('[data-component="contacts"]').forEach(element => {
    new ContactsComponent(element);
  });
});