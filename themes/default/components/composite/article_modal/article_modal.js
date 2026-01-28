// Article Modal Component JavaScript
document.addEventListener("DOMContentLoaded", function () {
  const articleModalComponents = document.querySelectorAll(
    '[data-component="article_modal"]'
  );

  articleModalComponents.forEach(function (component) {
    // Add smooth scrolling to TOC links
    const tocLinks = component.querySelectorAll(".article-toc a[href^='#']");
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