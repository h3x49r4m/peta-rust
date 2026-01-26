// Book Modal Component JavaScript

(function() {
  'use strict';

  class BookModal {
    constructor(modalElement) {
      this.modal = modalElement;
      this.overlay = this.modal;
      this.closeButton = this.modal.querySelector('.book-modal-close');
      this.chapterLinks = this.modal.querySelectorAll('.chapter-link');
      this.chapterNavButtons = this.modal.querySelectorAll('.chapter-nav-btn');
      this.currentChapterIndex = 0;
      this.chapters = [];

      this.init();
    }

    init() {
      // Close button functionality
      if (this.closeButton) {
        this.closeButton.addEventListener('click', () => this.close());
      }

      // Close on overlay click
      this.overlay.addEventListener('click', (e) => {
        if (e.target === this.overlay) {
          this.close();
        }
      });

      // Close on Escape key
      document.addEventListener('keydown', (e) => {
        if (e.key === 'Escape' && this.isOpen()) {
          this.close();
        }
      });

      // Chapter link clicks
      this.chapterLinks.forEach(link => {
        link.addEventListener('click', (e) => {
          e.preventDefault();
          const chapterIndex = parseInt(link.getAttribute('data-chapter-index'));
          this.navigateToChapter(chapterIndex);
        });
      });

      // Chapter navigation buttons
      this.chapterNavButtons.forEach(button => {
        button.addEventListener('click', () => {
          const chapterIndex = parseInt(button.getAttribute('data-chapter-index'));
          this.navigateToChapter(chapterIndex);
        });
      });

      // Extract chapters from DOM
      this.extractChapters();

      // Open modal by default to show full book content
      this.open();
    }

    extractChapters() {
      this.chapterLinks.forEach(link => {
        const index = parseInt(link.getAttribute('data-chapter-index'));
        const title = link.querySelector('.chapter-name').textContent.trim();
        const url = link.getAttribute('data-chapter-url');
        this.chapters.push({ index, title, url });
      });
    }

    open() {
      this.modal.classList.add('active');
      document.body.style.overflow = 'hidden';
    }

    close() {
      this.modal.classList.remove('active');
      document.body.style.overflow = '';
    }

    isOpen() {
      return this.modal.classList.contains('active');
    }

    navigateToChapter(index) {
      if (index < 0 || index >= this.chapters.length) {
        return;
      }

      this.currentChapterIndex = index;

      // Update active state in sidebar
      this.chapterLinks.forEach((link, i) => {
        if (i === index) {
          link.parentElement.classList.add('active');
        } else {
          link.parentElement.classList.remove('active');
        }
      });

      // Fetch and load chapter content
      const chapter = this.chapters[index];
      if (chapter && chapter.url) {
        this.loadChapterContent(chapter.url);
      }
    }

    loadChapterContent(chapterUrl) {
      const contentArea = this.modal.querySelector('.book-modal-content');
      if (!contentArea) return;

      // Show loading state
      contentArea.innerHTML = '<p class="loading">Loading chapter...</p>';

      // Build the full URL for the chapter
      // The chapterUrl is like "introduction.html", need to prepend book path
      const currentPath = window.location.pathname;
      // If we're on book index (ends with index.html or just /), use same directory
      let baseUrl = currentPath.replace(/\/[^\/]*$/, '/');
      const fullUrl = baseUrl + chapterUrl;

      console.log('Loading chapter from:', fullUrl);

      // Fetch chapter content
      fetch(fullUrl)
        .then(response => {
          if (!response.ok) {
            throw new Error(`Failed to load chapter: ${response.statusText}`);
          }
          return response.text();
        })
        .then(html => {
          // Parse the HTML and extract the main content
          const parser = new DOMParser();
          const doc = parser.parseFromString(html, 'text/html');
          const mainContent = doc.querySelector('.book-content');
          const articleContent = doc.querySelector('article.article-content');
          const contentDiv = doc.querySelector('.content-div');

          let contentHtml = '';
          if (mainContent) {
            contentHtml = mainContent.innerHTML;
          } else if (articleContent) {
            contentHtml = articleContent.innerHTML;
          } else if (contentDiv) {
            contentHtml = contentDiv.innerHTML;
          } else {
            // Fallback: use entire body content
            contentHtml = doc.body.innerHTML;
          }

          this.renderChapterContent(contentHtml);
        })
        .catch(error => {
          console.error('Error loading chapter:', error);
          contentArea.innerHTML = `
            <div class="error-message">
              <h3>Error Loading Chapter</h3>
              <p>${error.message}</p>
              <p>URL: ${fullUrl}</p>
              <p>Please try again or check your connection.</p>
            </div>
          `;
        });
    }

    renderChapterContent(contentHtml) {
      const contentArea = this.modal.querySelector('.book-modal-content');
      if (!contentArea) return;

      // Find the chapter title
      const parser = new DOMParser();
      const doc = parser.parseFromString(contentHtml, 'text/html');
      const firstHeading = doc.querySelector('h1, h2, h3');
      const chapterTitle = firstHeading ? firstHeading.textContent.trim() : '';

      // Build the new content structure
      const newContent = `
        <div class="chapter-content">
          ${chapterTitle ? `<h3 class="current-chapter-title">${chapterTitle}</h3>` : ''}
          <div class="current-chapter-body">
            ${contentHtml}
          </div>
        </div>
        ${this.renderChapterNavigation()}
      `;

      contentArea.innerHTML = newContent;

      // Re-attach navigation button event listeners
      this.attachNavigationListeners();
    }

    renderChapterNavigation() {
      const prevDisabled = this.currentChapterIndex <= 0 ? 'disabled' : '';
      const nextDisabled = this.currentChapterIndex >= this.chapters.length - 1 ? 'disabled' : '';

      return `
        <div class="chapter-navigation">
          <button class="chapter-nav-btn chapter-nav-prev" 
                  data-chapter-index="${this.currentChapterIndex - 1}"
                  ${prevDisabled}>
            ← Previous Chapter
          </button>
          <button class="chapter-nav-btn chapter-nav-next" 
                  data-chapter-index="${this.currentChapterIndex + 1}"
                  ${nextDisabled}>
            Next Chapter →
          </button>
        </div>
      `;
    }

    attachNavigationListeners() {
      const navButtons = this.modal.querySelectorAll('.chapter-nav-btn');
      navButtons.forEach(button => {
        button.addEventListener('click', () => {
          const chapterIndex = parseInt(button.getAttribute('data-chapter-index'));
          this.navigateToChapter(chapterIndex);
        });
      });
    }

    // Public method to open modal with specific chapter
    openWithChapter(chapterIndex = 0) {
      this.open();
      this.navigateToChapter(chapterIndex);
    }
  }

  // Initialize all book modals when DOM is ready
  function initBookModals() {
    const modals = document.querySelectorAll('[data-component="book_modal"]');
    modals.forEach(modal => {
      new BookModal(modal);
    });
  }

  // Run initialization when DOM is ready
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initBookModals);
  } else {
    initBookModals();
  }

  // Make BookModal available globally for external use
  window.BookModal = BookModal;

  // Add styles for loading and error states
  const bookModalStyle = document.createElement('style');
  bookModalStyle.textContent = `
    .loading {
      text-align: center;
      padding: 2rem;
      color: var(--text-secondary, #718096);
      font-size: 1.125rem;
    }

    .error-message {
      text-align: center;
      padding: 2rem;
      color: var(--color-danger, #ef4444);
    }

    .error-message h3 {
      margin: 0 0 1rem 0;
      color: var(--text-color, #1a202c);
    }

    .error-message p {
      margin: 0.5rem 0;
      color: var(--text-secondary, #718096);
    }
  `;
  document.head.appendChild(bookModalStyle);

})();