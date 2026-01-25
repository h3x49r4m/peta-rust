// Content Div Component
// This component provides consistent width constraints for page content

document.addEventListener('DOMContentLoaded', function() {
  // Find all content-div elements
  const contentDivs = document.querySelectorAll('.content-div');
  
  contentDivs.forEach(function(div) {
    // Get the max-width from data attribute if present
    const maxWidth = div.getAttribute('data-max-width');
    if (maxWidth && maxWidth !== '1200px') {
      div.style.maxWidth = maxWidth;
    }
  });
});