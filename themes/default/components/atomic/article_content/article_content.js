// Article Content Component JavaScript
document.addEventListener("DOMContentLoaded", function () {
  const articleContentComponents = document.querySelectorAll(
    '[data-component="article_content"]'
  );

  articleContentComponents.forEach(function (component) {
    // Initialize article content component
    initializeArticleContent(component);
  });
});

function initializeArticleContent(component) {
  // Initialize table of contents
  initializeTableOfContents(component);

  // Initialize image lightbox
  initializeImageLightbox(component);

  // Estimate reading time
  estimateReadingTime(component);

  // Initialize smooth scrolling
  initializeSmoothScrolling(component);
}

// Table of Contents Generation
function initializeTableOfContents(component) {
  const tocPlaceholder = component.querySelector("#table-of-contents");
  const tocNav = component.querySelector("#toc-nav");
  if (!tocPlaceholder || !tocNav) return;

  const headings = component.querySelectorAll(
    ".article-body h2, .article-body h3, .article-body h4"
  );
  if (headings.length === 0) {
    tocPlaceholder.style.display = "none";
    return;
  }

  tocPlaceholder.style.display = "block";
  let tocHTML = "";

  headings.forEach((heading, index) => {
    const level = parseInt(heading.tagName.charAt(1));
    const text = heading.textContent.trim();
    const id = heading.id || `heading-${index}`;

    if (!heading.id) {
      heading.id = id;
    }

    const indent = (level - 2) * 16;
    tocHTML += `
      <a href="#${id}" class="toc-link" style="padding-left: ${indent}px" data-level="${level}">
        ${text}
      </a>
    `;
  });

  tocNav.innerHTML = tocHTML;
}

// Enhanced Image Lightbox
function initializeImageLightbox(component) {
  const images = component.querySelectorAll(".article-body img");

  images.forEach(function (img) {
    img.style.cursor = "pointer";
    img.addEventListener("click", function () {
      createLightbox(this.src, this.alt);
    });
  });
}

function createLightbox(src, alt) {
  // Create overlay
  const overlay = document.createElement("div");
  overlay.className = "lightbox-overlay";
  overlay.style.cssText = `
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.95);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 9999;
    cursor: pointer;
    opacity: 0;
    transition: opacity 0.3s ease;
    backdrop-filter: blur(10px);
  `;

  // Create image container
  const imageContainer = document.createElement("div");
  imageContainer.style.cssText = `
    position: relative;
    max-width: 90%;
    max-height: 90%;
    display: flex;
    align-items: center;
    justify-content: center;
  `;

  // Create enlarged image
  const enlargedImg = document.createElement("img");
  enlargedImg.src = src;
  enlargedImg.alt = alt || "";
  enlargedImg.style.cssText = `
    max-width: 100%;
    max-height: 100%;
    object-fit: contain;
    border-radius: 1rem;
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
  `;

  // Create close button
  const closeButton = document.createElement("button");
  closeButton.innerHTML = `
    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <line x1="18" y1="6" x2="6" y2="18"></line>
      <line x1="6" y1="6" x2="18" y2="18"></line>
    </svg>
  `;
  closeButton.style.cssText = `
    position: absolute;
    top: -40px;
    right: 0;
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 0.5rem;
    color: white;
    cursor: pointer;
    padding: 0.5rem;
    transition: all 0.3s ease;
    backdrop-filter: blur(10px);
  `;

  closeButton.addEventListener("mouseenter", function () {
    this.style.background = "rgba(255, 255, 255, 0.2)";
    this.style.transform = "scale(1.1)";
  });

  closeButton.addEventListener("mouseleave", function () {
    this.style.background = "rgba(255, 255, 255, 0.1)";
    this.style.transform = "scale(1)";
  });

  imageContainer.appendChild(enlargedImg);
  imageContainer.appendChild(closeButton);
  overlay.appendChild(imageContainer);
  document.body.appendChild(overlay);

  // Animate in
  setTimeout(() => {
    overlay.style.opacity = "1";
  }, 10);

  // Close handlers
  function closeLightbox() {
    overlay.style.opacity = "0";
    setTimeout(() => {
      document.body.removeChild(overlay);
    }, 300);
  }

  overlay.addEventListener("click", closeLightbox);
  closeButton.addEventListener("click", closeLightbox);

  // Close on escape key
  function handleEscape(e) {
    if (e.key === "Escape") {
      closeLightbox();
      document.removeEventListener("keydown", handleEscape);
    }
  }
  document.addEventListener("keydown", handleEscape);
}

// Reading Time Estimation
function estimateReadingTime(component) {
  const readingTimeElement = component.querySelector(".reading-estimate");
  if (!readingTimeElement) return;

  const articleBody = component.querySelector(".article-body");
  if (!articleBody) return;

  const text = articleBody.textContent;
  const wordsPerMinute = 200;
  const words = text.trim().split(/\s+/).length;
  const minutes = Math.ceil(words / wordsPerMinute);

  readingTimeElement.textContent = `${minutes} min read`;
}

// Smooth Scrolling
function initializeSmoothScrolling(component) {
  const links = component.querySelectorAll('a[href^="#"]');

  links.forEach((link) => {
    link.addEventListener("click", function (e) {
      e.preventDefault();
      const targetId = this.getAttribute("href");
      const targetElement = document.querySelector(targetId);

      if (targetElement) {
        const offsetTop = targetElement.offsetTop - 80;
        window.scrollTo({
          top: offsetTop,
          behavior: "smooth",
        });
      }
    });
  });
}
