// Homepage Layout JavaScript
class HomepageLayout {
  constructor(element) {
    this.element = element;
    this.init();
  }
  
  init() {
    this.setupAnimations();
    this.setupScrollEffects();
    this.setupHeroActions();
  }
  
  setupAnimations() {
    const cards = this.element.querySelectorAll('.featured-card');
    cards.forEach((card, index) => {
      card.style.opacity = '0';
      card.style.transform = 'translateY(20px)';
      
      setTimeout(() => {
        card.style.transition = 'all 0.6s ease';
        card.style.opacity = '1';
        card.style.transform = 'translateY(0)';
      }, index * 100);
    });
  }
  
  setupScrollEffects() {
    const heroSection = this.element.querySelector('.hero-section');
    const heroContent = heroSection.querySelector('.hero-content');
    
    window.addEventListener('scroll', () => {
      const scrollY = window.pageYOffset;
      const heroHeight = heroSection.offsetHeight;
      
      if (scrollY < heroHeight) {
        const opacity = 1 - (scrollY / heroHeight) * 0.5;
        const transform = `translateY(${scrollY * 0.5}px)`;
        
        heroContent.style.opacity = opacity;
        heroContent.style.transform = transform;
      }
    });
  }
  
  setupHeroActions() {
    const primaryBtn = this.element.querySelector('.btn-primary');
    const secondaryBtn = this.element.querySelector('.btn-secondary');
    
    [primaryBtn, secondaryBtn].forEach(btn => {
      if (btn) {
        btn.addEventListener('mouseenter', () => {
          btn.style.transform = 'translateY(-2px)';
        });
        
        btn.addEventListener('mouseleave', () => {
          btn.style.transform = 'translateY(0)';
        });
      }
    });
  }
}

// Initialize homepage layout
document.addEventListener('DOMContentLoaded', () => {
  const homepageLayout = document.querySelector('.homepage-layout');
  if (homepageLayout) {
    new HomepageLayout(homepageLayout);
  }
});