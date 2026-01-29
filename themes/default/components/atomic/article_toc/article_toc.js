// Article TOC Component JavaScript
document.addEventListener("DOMContentLoaded", function () {
  const articleTocComponents = document.querySelectorAll(
    '[data-component="article_toc"]'
  );

  articleTocComponents.forEach(function (component) {
    // Initialize toggle button
    initializeToggleButton(component);

    // Add snippet icons to snippet headers
    addSnippetIcons(component);

    // Add smooth scrolling to TOC links
    const tocLinks = component.querySelectorAll('a[href^="#"]');
    tocLinks.forEach(function (link) {
      link.addEventListener("click", function (e) {
        e.preventDefault();
        const targetId = this.getAttribute("href").substring(1);
        const targetElement = document.getElementById(targetId);

        if (targetElement) {
          // Smooth scroll to target
          targetElement.scrollIntoView({
            behavior: "smooth",
            block: "start",
          });

          // Update active state on click only
          tocLinks.forEach((l) => l.classList.remove("active"));
          this.classList.add("active");

          // Update URL without jumping
          history.pushState(null, null, "#" + targetId);
        }
      });
    });
  });
});

function addSnippetIcons(component) {
  const tocLinks = component.querySelectorAll('.toc-link');
  
  // Create snippet icon SVG
  const snippetIcon = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
  snippetIcon.setAttribute('xmlns', 'http://www.w3.org/2000/svg');
  snippetIcon.setAttribute('fill', 'none');
  snippetIcon.setAttribute('viewBox', '0 0 24 24');
  snippetIcon.setAttribute('stroke', 'currentColor');
  snippetIcon.setAttribute('class', 'snippet-icon');
  snippetIcon.style.width = '14px';
  snippetIcon.style.height = '14px';
  snippetIcon.style.marginRight = '6px';
  snippetIcon.style.display = 'inline-block';
  snippetIcon.style.verticalAlign = 'middle';
  snippetIcon.innerHTML = '<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />';
  
  tocLinks.forEach(function(link) {
    const text = link.textContent.trim();
    if (text.startsWith('Snippet:')) {
      // Clone the icon for each link
      const iconClone = snippetIcon.cloneNode(true);
      // Prepend the icon to the link
      link.insertBefore(iconClone, link.firstChild);
    }
  });
}

function initializeToggleButton(component) {
  const toggleBtn = component.querySelector('.toc-toggle-btn');
  if (!toggleBtn) return;

  // Check for saved state
  const savedState = localStorage.getItem('article-toc-collapsed');
  const isCollapsed = savedState === 'true';

  // Apply saved state
  if (isCollapsed) {
    component.classList.add('collapsed');
    toggleBtn.setAttribute('aria-expanded', 'false');
  }

  // Toggle on click
  toggleBtn.addEventListener('click', function() {
    component.classList.toggle('collapsed');
    const isCollapsed = component.classList.contains('collapsed');
    
    // Update aria attribute
    toggleBtn.setAttribute('aria-expanded', !isCollapsed);
    
    // Save state to localStorage
    localStorage.setItem('article-toc-collapsed', isCollapsed.toString());
  });
}