// Books Layout JavaScript
class BooksLayout {
  constructor(element) {
    this.element = element;
    this.init();
  }
  
  init() {
    this.setupCategoryFilter();
    this.setupBookCards();
    this.setupSearch();
  }
  
  setupCategoryFilter() {
    const categoryFilter = this.element.querySelector('#category-filter');
    if (categoryFilter) {
      categoryFilter.addEventListener('change', (e) => {
        const category = e.target.value;
        const url = new URL(window.location);
        if (category) {
          url.searchParams.set('category', category);
        } else {
          url.searchParams.delete('category');
        }
        window.location = url.toString();
      });
    }
  }
  
  setupBookCards() {
    const cards = this.element.querySelectorAll('.book-card');
    cards.forEach(card => {
      card.addEventListener('mouseenter', () => {
        card.classList.add('book-card--hover');
      });
      card.addEventListener('mouseleave', () => {
        card.classList.remove('book-card--hover');
      });
    });
  }
  
  setupSearch() {
    // Add search functionality if needed
    const searchInput = this.element.querySelector('#search-input');
    if (searchInput) {
      searchInput.addEventListener('input', (e) => {
        const query = e.target.value.toLowerCase();
        const cards = this.element.querySelectorAll('.book-card');
        
        cards.forEach(card => {
          const title = card.querySelector('.book-title').textContent.toLowerCase();
          const excerpt = card.querySelector('.book-excerpt')?.textContent.toLowerCase() || '';
          
          if (title.includes(query) || excerpt.includes(query)) {
            card.style.display = 'block';
          } else {
            card.style.display = 'none';
          }
        });
      });
    }
  }
}

// Initialize books layout
document.addEventListener('DOMContentLoaded', () => {
  const booksLayout = document.querySelector('.books-layout');
  if (booksLayout) {
    new BooksLayout(booksLayout);
  }
});