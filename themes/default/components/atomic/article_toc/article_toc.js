// Article TOC Component JavaScript
document.addEventListener("DOMContentLoaded", function () {
  const articleTocComponents = document.querySelectorAll(
    '[data-component="article_toc"]'
  );

  articleTocComponents.forEach(function (component) {
    // Initialize toggle button
    initializeToggleButton(component);

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