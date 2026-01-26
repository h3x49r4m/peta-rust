// Book Modal Component JavaScript
(function() {
  'use strict';

  class BookModal {
    constructor(modalElement) {
      this.modal = modalElement;
      this.overlay = this.modal;
      this.closeButton = this.modal.querySelector('.book-modal-close');
      
      this.init();
    }

    init() {
      // Close button functionality
      if (this.closeButton) {
        this.closeButton.addEventListener('click', () => {
          // Go back to books listing
          window.location.href = '/books.html';
        });
      }

      // Close on Escape key
      document.addEventListener('keydown', (e) => {
        if (e.key === 'Escape') {
          window.location.href = '/books.html';
        }
      });
    }

    isOpen() {
      return this.modal.classList.contains('active');
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

})();