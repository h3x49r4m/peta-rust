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
  // Initialize reading progress
  initializeReadingProgress(component);

  

  // Initialize code block copy functionality
  initializeCodeBlocks(component);

  // Initialize image lightbox
  initializeImageLightbox(component);

  // Initialize share functionality
  initializeShareButtons(component);

  // Estimate reading time
  estimateReadingTime(component);

  // Initialize smooth scrolling
  initializeSmoothScrolling(component);
}

// Reading Progress Indicator
function initializeReadingProgress(component) {
  const progressBar = component.querySelector(".reading-progress");
  if (!progressBar) return;

  const articleBody = component.querySelector(".article-body");
  if (!articleBody) return;

  function updateProgress() {
    const scrollTop = window.pageYOffset || document.documentElement.scrollTop;
    const scrollHeight = articleBody.offsetHeight;
    const clientHeight = document.documentElement.clientHeight;
    const articleTop = articleBody.offsetTop;

    let progress = 0;
    if (scrollTop > articleTop) {
      const articleScrollTop = scrollTop - articleTop;
      const articleHeight = scrollHeight - clientHeight;
      progress = Math.min(
        100,
        Math.max(0, (articleScrollTop / articleHeight) * 100)
      );
    }

    progressBar.style.width = progress + "%";
  }

  window.addEventListener("scroll", updateProgress);
  updateProgress();
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

  // Add active state tracking
  const tocLinks = tocNav.querySelectorAll(".toc-link");
  const observerOptions = {
    root: null,
    rootMargin: "-20% 0px -70% 0px",
    threshold: 0,
  };

  const observer = new IntersectionObserver((entries) => {
    entries.forEach((entry) => {
      if (entry.isIntersecting) {
        tocLinks.forEach((link) => link.classList.remove("active"));
        const activeLink = tocNav.querySelector(`[href="#${entry.target.id}"]`);
        if (activeLink) {
          activeLink.classList.add("active");
        }
      }
    });
  }, observerOptions);

  headings.forEach((heading) => observer.observe(heading));
}

// Enhanced Code Block Functionality
function initializeCodeBlocks(component) {
  const codeBlocks = component.querySelectorAll("pre code");

  codeBlocks.forEach(function (codeBlock) {
    const pre = codeBlock.parentElement;

    // Create copy button with enhanced styling
    const copyButton = createCopyButton();
    pre.style.position = "relative";
    pre.appendChild(copyButton);

    // Add language indicator if available
    const language = detectLanguage(codeBlock);
    if (language) {
      const languageLabel = document.createElement("div");
      languageLabel.className = "code-language";
      languageLabel.textContent = language;
      languageLabel.style.cssText = `
        position: absolute;
        top: 0.5rem;
        left: 0.5rem;
        background: linear-gradient(135deg, rgba(59, 130, 246, 0.2), rgba(139, 92, 246, 0.1));
        color: #3b82f6;
        padding: 0.25rem 0.5rem;
        border-radius: 0.25rem;
        font-size: 0.75rem;
        font-weight: 600;
        text-transform: uppercase;
        letter-spacing: 0.05em;
        backdrop-filter: blur(10px);
      `;
      pre.appendChild(languageLabel);
    }

    // Handle copy functionality
    copyButton.addEventListener("click", function () {
      copyCode(codeBlock, copyButton);
    });
  });
}

function createCopyButton() {
  const button = document.createElement("button");
  button.className = "code-copy-button";
  button.innerHTML = `
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
      <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
    </svg>
    Copy
  `;

  button.style.cssText = `
    position: absolute;
    top: 0.5rem;
    right: 0.5rem;
    background: linear-gradient(135deg, rgba(255, 255, 255, 0.95), rgba(248, 250, 252, 0.95));
    border: 1px solid #e2e8f0;
    border-radius: 0.5rem;
    padding: 0.375rem 0.75rem;
    font-size: 0.875rem;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.375rem;
    transition: all 0.3s ease;
    color: #4a5568;
    backdrop-filter: blur(10px);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  `;

  button.addEventListener("mouseenter", function () {
    if (
      !this.classList.contains("copied") &&
      !this.classList.contains("error")
    ) {
      this.style.background = "linear-gradient(135deg, #f8fafc, #f1f5f9)";
      this.style.borderColor = "#cbd5e0";
      this.style.transform = "translateY(-2px)";
      this.style.boxShadow = "0 4px 12px rgba(0, 0, 0, 0.1)";
    }
  });

  button.addEventListener("mouseleave", function () {
    if (
      !this.classList.contains("copied") &&
      !this.classList.contains("error")
    ) {
      this.style.background =
        "linear-gradient(135deg, rgba(255, 255, 255, 0.95), rgba(248, 250, 252, 0.95))";
      this.style.borderColor = "#e2e8f0";
      this.style.transform = "translateY(0)";
      this.style.boxShadow = "0 2px 4px rgba(0, 0, 0, 0.05)";
    }
  });

  return button;
}

function detectLanguage(codeBlock) {
  const classes = codeBlock.className.split(" ");
  for (const cls of classes) {
    if (cls.startsWith("language-")) {
      return cls.replace("language-", "");
    }
  }
  return null;
}

function copyCode(codeBlock, button) {
  const text = codeBlock.textContent;

  navigator.clipboard
    .writeText(text)
    .then(function () {
      button.classList.add("copied");
      button.innerHTML = `
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="20 6 9 17 4 12"></polyline>
        </svg>
        Copied!
      `;
      button.style.background = "linear-gradient(135deg, #10b981, #059669)";
      button.style.borderColor = "#10b981";
      button.style.color = "white";
      button.style.transform = "translateY(-2px)";
      button.style.boxShadow = "0 4px 12px rgba(16, 185, 129, 0.3)";

      setTimeout(() => resetButton(button), 2000);
    })
    .catch(function (err) {
      console.error("Failed to copy text: ", err);
      button.classList.add("error");
      button.innerHTML = `
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="10"></circle>
          <line x1="12" y1="8" x2="12" y2="12"></line>
          <line x1="12" y1="16" x2="12.01" y2="16"></line>
        </svg>
        Failed!
      `;
      button.style.background = "linear-gradient(135deg, #ef4444, #dc2626)";
      button.style.borderColor = "#ef4444";
      button.style.color = "white";
      button.style.transform = "translateY(-2px)";
      button.style.boxShadow = "0 4px 12px rgba(239, 68, 68, 0.3)";

      setTimeout(() => resetButton(button), 2000);
    });
}

function resetButton(button) {
  button.classList.remove("copied", "error");
  button.innerHTML = `
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
      <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
    </svg>
    Copy
  `;
  button.style.background =
    "linear-gradient(135deg, rgba(255, 255, 255, 0.95), rgba(248, 250, 252, 0.95))";
  button.style.borderColor = "#e2e8f0";
  button.style.color = "#4a5568";
  button.style.transform = "translateY(0)";
  button.style.boxShadow = "0 2px 4px rgba(0, 0, 0, 0.05)";
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

// Share Functionality
function initializeShareButtons(component) {
  const shareButtons = component.querySelectorAll(".share-button");

  shareButtons.forEach((button) => {
    button.addEventListener("click", function () {
      const platform = this.dataset.platform;
      handleShare(platform, this);
    });
  });
}

function handleShare(platform, button) {
  const url = window.location.href;
  const title = document.title;

  switch (platform) {
    case "twitter":
      window.open(
        `https://twitter.com/intent/tweet?url=${encodeURIComponent(
          url
        )}&text=${encodeURIComponent(title)}`,
        "_blank"
      );
      break;
    case "linkedin":
      window.open(
        `https://www.linkedin.com/sharing/share-offsite/?url=${encodeURIComponent(
          url
        )}`,
        "_blank"
      );
      break;
    case "copy":
      navigator.clipboard.writeText(url).then(() => {
        const originalText = button.innerHTML;
        button.innerHTML = `
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="20 6 9 17 4 12"></polyline>
          </svg>
          Copied!
        `;
        button.style.background = "linear-gradient(135deg, #10b981, #059669)";
        button.style.borderColor = "#10b981";
        button.style.color = "white";
        button.style.transform = "translateY(-2px)";
        button.style.boxShadow = "0 4px 12px rgba(16, 185, 129, 0.3)";

        setTimeout(() => {
          button.innerHTML = originalText;
          button.style.background = "";
          button.style.borderColor = "";
          button.style.color = "";
          button.style.transform = "";
          button.style.boxShadow = "";
        }, 2000);
      });
      break;
  }
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