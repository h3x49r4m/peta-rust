// Page Tags Component JavaScript
document.addEventListener('DOMContentLoaded', function() {
  const pageTagsComponents = document.querySelectorAll('[data-component="page_tags"]');
  
  pageTagsComponents.forEach(function(component) {
    // Initialize page tags component
    console.log('Page tags component initialized');
    
    // Add any interactive behavior here
    component.addEventListener('click', function(e) {
      // Handle clicks within the component
      if (e.target.classList.contains('tag-cloud-tag')) {
        console.log('Tag clicked:', e.target.textContent);
      }
    });
  });
});