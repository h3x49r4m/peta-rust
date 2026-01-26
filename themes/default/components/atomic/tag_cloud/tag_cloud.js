// Tag Cloud Component JavaScript
document.addEventListener('DOMContentLoaded', function() {
  const tagClouds = document.querySelectorAll('[data-component="tag_cloud"]');
  
  tagClouds.forEach(tagCloud => {
    // Add hover effects for tags
    const tags = tagCloud.querySelectorAll('.tag-cloud-tag');
    
    tags.forEach(tag => {
      tag.addEventListener('mouseenter', function() {
        this.style.zIndex = '10';
      });
      
      tag.addEventListener('mouseleave', function() {
        this.style.zIndex = '';
      });
    });
    
    // Add click tracking
    tags.forEach(tag => {
      tag.addEventListener('click', function(e) {
        // Track tag clicks for analytics
        const tagName = this.textContent.trim();
        console.log('Tag clicked:', tagName);
        
        // Add ripple effect
        const ripple = document.createElement('span');
        ripple.style.position = 'absolute';
        ripple.style.borderRadius = '50%';
        ripple.style.background = 'rgba(59, 130, 246, 0.3)';
        ripple.style.width = ripple.style.height = '20px';
        ripple.style.top = '50%';
        ripple.style.left = '50%';
        ripple.style.transform = 'translate(-50%, -50%) scale(0)';
        ripple.style.animation = 'ripple 0.6s ease-out';
        
        this.style.position = 'relative';
        this.appendChild(ripple);
        
        setTimeout(() => {
          ripple.remove();
        }, 600);
      });
    });
    
    // Add animation for tag cloud container
    const container = tagCloud.querySelector('.tag-cloud-container');
    if (container) {
      // Stagger animation for tags
      const tagElements = container.querySelectorAll('.tag-cloud-tag');
      tagElements.forEach((tag, index) => {
        tag.style.opacity = '0';
        tag.style.transform = 'translateY(10px)';
        
        setTimeout(() => {
          tag.style.transition = 'opacity 0.3s ease, transform 0.3s ease';
          tag.style.opacity = '';
          tag.style.transform = '';
        }, index * 50);
      });
    }
  });
});

// Add ripple animation to styles
const tagCloudStyle = document.createElement('style');
  tagCloudStyle.textContent = `
    @keyframes ripple {
      to {
        transform: translate(-50%, -50%) scale(4);
        opacity: 0;
      }
    }
  `;
  document.head.appendChild(tagCloudStyle);