/**
 * Grid Cards Component JavaScript
 * Handles interactive functionality for the grid cards composite component
 */

class GridCardsComponent {
  constructor(element) {
    this.element = element;
    this.gridElement = element.querySelector('.grid-cards-grid');
    this.columns = parseInt(element.dataset.columns) || 3;
    this.init();
  }

  init() {
    this.setupResponsiveGrid();
    this.setupIntersectionObserver();
    this.setupKeyboardNavigation();
    this.setupFiltering();
    this.setupSearch();
  }

  /**
   * Setup responsive grid behavior
   */
  setupResponsiveGrid() {
    // Adjust grid columns based on container width
    const resizeObserver = new ResizeObserver(entries => {
      for (const entry of entries) {
        this.adjustGridColumns(entry.contentRect.width);
      }
    });

    resizeObserver.observe(this.element);
  }

  /**
   * Adjust grid columns based on available width
   */
  adjustGridColumns(width) {
    const cards = this.gridElement?.querySelectorAll('.grid-card');
    if (!cards.length) return;

    let optimalColumns = this.columns;
    
    if (width < 640) {
      optimalColumns = 1;
    } else if (width < 768) {
      optimalColumns = Math.min(2, this.columns);
    } else if (width < 1024) {
      optimalColumns = Math.min(3, this.columns);
    }

    this.element.dataset.columns = optimalColumns;
  }

  /**
   * Setup intersection observer for lazy loading animations
   */
  setupIntersectionObserver() {
    const options = {
      root: null,
      rootMargin: '50px',
      threshold: 0.1
    };

    const observer = new IntersectionObserver((entries) => {
      entries.forEach(entry => {
        if (entry.isIntersecting) {
          entry.target.style.animationPlayState = 'running';
          observer.unobserve(entry.target);
        }
      });
    }, options);

    const cards = this.gridElement?.querySelectorAll('.grid-card');
    cards?.forEach(card => {
      card.style.animationPlayState = 'paused';
      observer.observe(card);
    });
  }

  /**
   * Setup keyboard navigation for grid cards
   */
  setupKeyboardNavigation() {
    const cards = this.gridElement?.querySelectorAll('.grid-card');
    if (!cards.length) return;

    let currentIndex = -1;

    this.element.addEventListener('keydown', (e) => {
      switch (e.key) {
        case 'ArrowRight':
          e.preventDefault();
          currentIndex = Math.min(currentIndex + 1, cards.length - 1);
          this.focusCard(cards[currentIndex]);
          break;
        case 'ArrowLeft':
          e.preventDefault();
          currentIndex = Math.max(currentIndex - 1, 0);
          this.focusCard(cards[currentIndex]);
          break;
        case 'ArrowDown':
          e.preventDefault();
          const cols = parseInt(this.element.dataset.columns) || 3;
          currentIndex = Math.min(currentIndex + cols, cards.length - 1);
          this.focusCard(cards[currentIndex]);
          break;
        case 'ArrowUp':
          e.preventDefault();
          const colsUp = parseInt(this.element.dataset.columns) || 3;
          currentIndex = Math.max(currentIndex - colsUp, 0);
          this.focusCard(cards[currentIndex]);
          break;
        case 'Enter':
          if (currentIndex >= 0 && cards[currentIndex]) {
            const link = cards[currentIndex].querySelector('.card-title a, .card-link');
            if (link) link.click();
          }
          break;
      }
    });
  }

  /**
   * Focus on a specific card
   */
  focusCard(card) {
    if (!card) return;
    
    // Remove previous focus
    this.gridElement?.querySelectorAll('.grid-card')?.forEach(c => {
      c.classList.remove('keyboard-focused');
    });
    
    // Add focus to current card
    card.classList.add('keyboard-focused');
    card.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
  }

  /**
   * Setup filtering functionality
   */
  setupFiltering() {
    // Look for filter controls in the page
    const filterControls = document.querySelectorAll('[data-filter-target="grid-cards"]');
    
    filterControls.forEach(control => {
      control.addEventListener('change', (e) => {
        this.filterCards(e.target.value, e.target.dataset.filterType);
      });
    });
  }

  /**
   * Filter cards based on criteria
   */
  filterCards(value, type) {
    const cards = this.gridElement?.querySelectorAll('.grid-card');
    if (!cards.length) return;

    cards.forEach(card => {
      let shouldShow = true;

      switch (type) {
        case 'tag':
          const tags = card.dataset.tags?.split(',') || [];
          shouldShow = !value || tags.includes(value);
          break;
        case 'content-type':
          shouldShow = !value || card.dataset.contentType === value;
          break;
        case 'search':
          const searchText = value.toLowerCase();
          const title = card.querySelector('.card-title')?.textContent.toLowerCase() || '';
          const excerpt = card.querySelector('.card-excerpt')?.textContent.toLowerCase() || '';
          shouldShow = !searchText || title.includes(searchText) || excerpt.includes(searchText);
          break;
      }

      card.style.display = shouldShow ? '' : 'none';
    });

    this.updateEmptyState();
  }

  /**
   * Setup search functionality
   */
  setupSearch() {
    // Look for search inputs
    const searchInputs = document.querySelectorAll('[data-search-target="grid-cards"]');
    
    searchInputs.forEach(input => {
      let timeout;
      input.addEventListener('input', (e) => {
        clearTimeout(timeout);
        timeout = setTimeout(() => {
          this.filterCards(e.target.value, 'search');
        }, 300);
      });
    });
  }

  /**
   * Update empty state visibility
   */
  updateEmptyState() {
    const cards = this.gridElement?.querySelectorAll('.grid-card');
    const visibleCards = Array.from(cards).filter(card => card.style.display !== 'none');
    const emptyState = this.element.querySelector('.grid-cards-empty');

    if (emptyState) {
      emptyState.style.display = visibleCards.length === 0 ? '' : 'none';
    }
  }

  /**
   * Public method to refresh the component
   */
  refresh() {
    this.setupIntersectionObserver();
    this.updateEmptyState();
  }

  /**
   * Public method to add new cards
   */
  addCards(newCards) {
    if (!this.gridElement || !newCards.length) return;

    newCards.forEach((cardData, index) => {
      // Create card element (this would need to be implemented based on your templating system)
      const cardElement = this.createCardElement(cardData);
      if (cardElement) {
        this.gridElement.appendChild(cardElement);
        
        // Add staggered animation
        setTimeout(() => {
          cardElement.style.animationPlayState = 'running';
        }, (index + 1) * 100);
      }
    });

    this.refresh();
  }

  /**
   * Create a card element from data (placeholder for actual implementation)
   */
  createCardElement(cardData) {
    // This would need to be implemented based on your templating system
    // For now, return null as the actual card creation should be handled by the backend
    return null;
  }

  /**
   * Destroy the component and clean up
   */
  destroy() {
    // Clean up observers and event listeners
    if (this.resizeObserver) {
      this.resizeObserver.disconnect();
    }
    if (this.intersectionObserver) {
      this.intersectionObserver.disconnect();
    }
  }
}

// Initialize components when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
  const gridCardsElements = document.querySelectorAll('[data-component="grid_cards"]');
  
  gridCardsElements.forEach(element => {
    new GridCardsComponent(element);
  });
});

// Export for potential module usage
if (typeof module !== 'undefined' && module.exports) {
  module.exports = GridCardsComponent;
}