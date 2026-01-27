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

          // Update active state
          tocLinks.forEach((l) => l.classList.remove("active"));
          this.classList.add("active");

          // Update URL without jumping
          history.pushState(null, null, "#" + targetId);
        }
      });
    });

    // Highlight current section on scroll
    function updateActiveTocItem() {
      const sections = document.querySelectorAll(
        ".project-body h1[id], .project-body h2[id], .project-body h3[id]"
      );
      const scrollPosition = window.scrollY + 100;

      let currentSection = "";
      sections.forEach(function (section) {
        const sectionTop = section.offsetTop;
        if (scrollPosition >= sectionTop) {
          currentSection = section.getAttribute("id");
        }
      });

      if (currentSection) {
        tocLinks.forEach(function (link) {
          link.classList.remove("active");
          if (link.getAttribute("href") === "#" + currentSection) {
            link.classList.add("active");
          }
        });
      }
    }

    // Throttled scroll handler
    let scrollTimer = null;
    window.addEventListener("scroll", function () {
      if (scrollTimer) clearTimeout(scrollTimer);
      scrollTimer = setTimeout(updateActiveTocItem, 100);
    });

    // Initial check for active item
    updateActiveTocItem();
  });
});