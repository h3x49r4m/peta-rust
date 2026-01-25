// Grid Card Component JavaScript
document.addEventListener('DOMContentLoaded', function() {
  const gridCards = document.querySelectorAll('[data-component="grid_card"]');
  
  gridCards.forEach(card => {
    // Add smooth hover effects
    card.addEventListener('mouseenter', function() {
      this.style.transform = 'translateY(-4px)';
    });
    
    card.addEventListener('mouseleave', function() {
      this.style.transform = '';
    });
    
    // Add click tracking for card links
    const cardLink = card.querySelector('.card-link');
    if (cardLink) {
      cardLink.addEventListener('click', function(e) {
        const title = card.querySelector('.card-title').textContent.trim();
        console.log('Card clicked:', title);
      });
    }
    
    // Add tag click tracking
    const tags = card.querySelectorAll('.card-tag');
    tags.forEach(tag => {
      tag.addEventListener('click', function(e) {
        e.preventDefault();
        const tagName = this.textContent.trim();
        console.log('Tag clicked:', tagName);
        
        // Add visual feedback
        this.style.transform = 'scale(0.95)';
        setTimeout(() => {
          this.style.transform = '';
        }, 150);
      });
    });
    
    // Add stagger animation on load
    setTimeout(() => {
      card.style.opacity = '0';
      card.style.transform = 'translateY(20px)';
      
      setTimeout(() => {
        card.style.transition = 'opacity 0.5s ease, transform 0.5s ease';
        card.style.opacity = '';
        card.style.transform = '';
      }, Math.random() * 200);
    }, 100);
  });
  
  // Add intersection observer for lazy loading animation
  const observerOptions = {
    threshold: 0.1,
    rootMargin: '0px 0px -50px 0px'
  };
  
  const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
      if (entry.isIntersecting) {
        entry.target.classList.add('visible');
        observer.unobserve(entry.target);
      }
    });
  }, observerOptions);
  
  gridCards.forEach(card => {
    observer.observe(card);
  });
});

// Add visible class animation
const style = document.createElement('style');
style.textContent = `
  .grid-card {
    opacity: 0;
    transform: translateY(20px);
    transition: opacity 0.5s ease, transform 0.5s ease;
  }
  
  .grid-card.visible {
    opacity: 1;
    transform: translateY(0);
  }
`;
document.head.appendChild(style);