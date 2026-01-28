// Project Modal Component JavaScript
document.addEventListener("DOMContentLoaded", function () {
  const projectModalComponents = document.querySelectorAll(
    '[data-component="project_modal"]'
  );

  projectModalComponents.forEach(function (component) {
    // Add smooth scrolling to TOC links
    const tocLinks = component.querySelectorAll(".project-toc a[href^='#']");
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