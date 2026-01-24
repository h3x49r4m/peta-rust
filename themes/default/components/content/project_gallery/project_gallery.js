class ProjectGalleryComponent {
  constructor(element) {
    this.element = element;
    this.layout = element.dataset.layout;
    this.columns = parseInt(element.dataset.columns) || 3;
    this.grid = element.querySelector('[data-grid]');
    this.items = element.querySelectorAll('.project-gallery-item');
    this.moreBtn = element.querySelector('[data-more-btn]');
    this.pagination = element.querySelector('[data-pagination]');
    this.paginationPages = element.querySelector('[data-pagination-pages]');
    this.prevBtn = element.querySelector('[data-pagination-prev]');
    this.nextBtn = element.querySelector('[data-pagination-next]');
    this.activeFilters = element.querySelector('[data-active-filters]');
    
    this.state = {
      currentPage: 1,
      itemsPerPage: this.calculateItemsPerPage(),
      filteredItems: Array.from(this.items),
      activeFilters: new Set(),
      isLoading: false,
      totalItems: this.items.length
    };
    
    this.init();
  }
  
  init() {
    this.setupEventListeners();
    this.setupFiltering();
    this.setupPagination();
    this.initializeItems();
  }
  
  setupEventListeners() {
    // More button click
    if (this.moreBtn) {
      this.moreBtn.addEventListener('click', () => this.handleMoreClick());
    }
    
    // Pagination button clicks
    if (this.prevBtn) {
      this.prevBtn.addEventListener('click', () => this.handlePaginationClick('prev'));
    }
    
    if (this.nextBtn) {
      this.nextBtn.addEventListener('click', () => this.handlePaginationClick('next'));
    }
    
    // Window resize
    window.addEventListener('resize', () => this.handleResize());
    
    // Custom events
    this.element.addEventListener('filter_change', (e) => this.handleFilterChange(e));
    this.element.addEventListener('gallery_update', (e) => this.handleGalleryUpdate(e));
  }
  
  setupFiltering() {
    // Listen for filter changes from language_filter component
    const languageFilter = this.element.querySelector('.language-filter');
    if (languageFilter) {
      languageFilter.addEventListener('filter_change', (e) => {
        this.applyFilter(e.detail.language);
      });
    }
  }
  
  setupPagination() {
    if (!this.pagination) return;
    
    this.updatePaginationButtons();
    this.renderPaginationPages();
  }
  
  initializeItems() {
    // Add animation classes to items
    this.items.forEach((item, index) => {
      item.style.animationDelay = `${(index + 1) * 0.1}s`;
    });
  }
  
  handleMoreClick() {
    // Track more button clicks
    this.element.dispatchEvent(new CustomEvent('gallery_more_click', {
      detail: { 
        currentPage: this.state.currentPage,
        totalItems: this.state.totalItems
      }
    }));
    
    // Add loading state
    this.moreBtn.classList.add('loading');
    this.moreBtn.disabled = true;
    
    // Simulate loading more items
    setTimeout(() => {
      this.loadMoreItems();
      this.moreBtn.classList.remove('loading');
      this.moreBtn.disabled = false;
    }, 1000);
  }
  
  handlePaginationClick(direction) {
    if (direction === 'prev') {
      this.state.currentPage = Math.max(1, this.state.currentPage - 1);
    } else {
      this.state.currentPage = Math.min(
        this.getTotalPages(),
        this.state.currentPage + 1
      );
    }
    
    this.updateGallery();
    this.updatePaginationButtons();
    this.renderPaginationPages();
    
    // Track pagination clicks
    this.element.dispatchEvent(new CustomEvent('gallery_pagination_click', {
      detail: { 
        direction,
        page: this.state.currentPage,
        totalPages: this.getTotalPages()
      }
    }));
  }
  
  handleResize() {
    const newItemsPerPage = this.calculateItemsPerPage();
    if (newItemsPerPage !== this.state.itemsPerPage) {
      this.state.itemsPerPage = newItemsPerPage;
      this.updateGallery();
      this.setupPagination();
    }
  }
  
  handleFilterChange(event) {
    const { language } = event.detail;
    this.applyFilter(language);
  }
  
  handleGalleryUpdate(event) {
    const { projects } = event.detail;
    this.updateProjects(projects);
  }
  
  applyFilter(language) {
    if (language) {
      this.state.activeFilters.add(language);
    } else {
      this.state.activeFilters.clear();
    }
    
    // Filter items
    this.state.filteredItems = Array.from(this.items).filter(item => {
      if (this.state.activeFilters.size === 0) return true;
      
      const itemLanguage = item.dataset.language;
      const itemTags = item.dataset.tags.split(',');
      
      // Check if item matches any active filter
      for (const filter of this.state.activeFilters) {
        if (itemLanguage === filter || itemTags.includes(filter)) {
          return true;
        }
      }
      
      return false;
    });
    
    // Reset to first page
    this.state.currentPage = 1;
    
    // Update UI
    this.updateGallery();
    this.updateActiveFiltersDisplay();
    this.setupPagination();
    
    // Track filter application
    this.element.dispatchEvent(new CustomEvent('gallery_filter_applied', {
      detail: {
        filters: Array.from(this.state.activeFilters),
        filteredCount: this.state.filteredItems.length,
        totalCount: this.state.totalItems
      }
    }));
  }
  
  updateGallery() {
    const startIndex = (this.state.currentPage - 1) * this.state.itemsPerPage;
    const endIndex = startIndex + this.state.itemsPerPage;
    const itemsToShow = this.state.filteredItems.slice(startIndex, endIndex);
    
    // Hide all items
    this.items.forEach(item => {
      item.classList.add('filtered-out');
      item.classList.remove('filtered-in');
    });
    
    // Show filtered items for current page
    itemsToShow.forEach((item, index) => {
      setTimeout(() => {
        item.classList.remove('filtered-out');
        item.classList.add('filtered-in');
      }, index * 50);
    });
    
    // Update more button
    if (this.moreBtn) {
      const hasMore = endIndex < this.state.filteredItems.length;
      this.moreBtn.style.display = hasMore ? 'inline-flex' : 'none';
    }
    
    // Update empty state
    this.updateEmptyState();
  }
  
  updateActiveFiltersDisplay() {
    if (!this.activeFilters) return;
    
    this.activeFilters.innerHTML = '';
    
    this.state.activeFilters.forEach(filter => {
      const filterElement = document.createElement('div');
      filterElement.className = 'project-gallery-active-filter';
      filterElement.innerHTML = `
        <span>${filter}</span>
        <button class="project-gallery-active-filter-remove" data-filter="${filter}">
          {% component "icon" with name="x" library="feather" size="xs" %}
          {% endslot %}
        </button>
      `;
      
      const removeBtn = filterElement.querySelector('.project-gallery-active-filter-remove');
      removeBtn.addEventListener('click', () => this.removeFilter(filter));
      
      this.activeFilters.appendChild(filterElement);
    });
  }
  
  removeFilter(filter) {
    this.state.activeFilters.delete(filter);
    this.applyFilter('');
    
    // Update language filter component
    const languageFilter = this.element.querySelector('.language-filter');
    if (languageFilter) {
      languageFilter.dispatchEvent(new CustomEvent('filter_clear'));
    }
  }
  
  updateEmptyState() {
    const existingEmpty = this.element.querySelector('.project-gallery-empty');
    
    if (this.state.filteredItems.length === 0) {
      if (!existingEmpty) {
        const emptyElement = document.createElement('div');
        emptyElement.className = 'project-gallery-empty';
        emptyElement.innerHTML = `
          <div class="project-gallery-empty-icon">
            {% component "icon" with name="folder" library="feather" size="lg" %}
            {% endslot %}
          </div>
          <div class="project-gallery-empty-text">No projects found</div>
          <div class="project-gallery-empty-subtext">Try adjusting your filters</div>
        `;
        this.grid.appendChild(emptyElement);
      }
    } else if (existingEmpty) {
      existingEmpty.remove();
    }
  }
  
  loadMoreItems() {
    // Increase items per page
    this.state.itemsPerPage += 3;
    this.updateGallery();
    
    // Track load more
    this.element.dispatchEvent(new CustomEvent('gallery_load_more', {
      detail: {
        itemsPerPage: this.state.itemsPerPage,
        currentPage: this.state.currentPage
      }
    }));
  }
  
  updatePaginationButtons() {
    if (!this.prevBtn || !this.nextBtn) return;
    
    this.prevBtn.disabled = this.state.currentPage === 1;
    this.nextBtn.disabled = this.state.currentPage >= this.getTotalPages();
  }
  
  renderPaginationPages() {
    if (!this.paginationPages) return;
    
    this.paginationPages.innerHTML = '';
    
    const totalPages = this.getTotalPages();
    const maxVisiblePages = 5;
    
    let startPage = Math.max(1, this.state.currentPage - Math.floor(maxVisiblePages / 2));
    let endPage = Math.min(totalPages, startPage + maxVisiblePages - 1);
    
    if (endPage - startPage + 1 < maxVisiblePages) {
      startPage = Math.max(1, endPage - maxVisiblePages + 1);
    }
    
    // Add first page and ellipsis if needed
    if (startPage > 1) {
      this.addPaginationPage(1);
      if (startPage > 2) {
        this.addPaginationEllipsis();
      }
    }
    
    // Add visible pages
    for (let i = startPage; i <= endPage; i++) {
      this.addPaginationPage(i);
    }
    
    // Add ellipsis and last page if needed
    if (endPage < totalPages) {
      if (endPage < totalPages - 1) {
        this.addPaginationEllipsis();
      }
      this.addPaginationPage(totalPages);
    }
  }
  
  addPaginationPage(pageNum) {
    const pageButton = document.createElement('button');
    pageButton.className = 'project-gallery-pagination-page';
    if (pageNum === this.state.currentPage) {
      pageButton.classList.add('project-gallery-pagination-page--active');
    }
    pageButton.textContent = pageNum;
    pageButton.addEventListener('click', () => {
      this.state.currentPage = pageNum;
      this.updateGallery();
      this.updatePaginationButtons();
      this.renderPaginationPages();
    });
    this.paginationPages.appendChild(pageButton);
  }
  
  addPaginationEllipsis() {
    const ellipsis = document.createElement('span');
    ellipsis.className = 'project-gallery-pagination-ellipsis';
    ellipsis.textContent = '...';
    ellipsis.style.cssText = `
      display: flex;
      align-items: center;
      justify-content: center;
      width: 36px;
      height: 36px;
      color: var(--text-muted);
    `;
    this.paginationPages.appendChild(ellipsis);
  }
  
  updateProjects(projects) {
    // In a real implementation, this would update the DOM with new projects
    // For now, we'll just update the state and refresh
    this.state.totalItems = projects.length;
    this.state.filteredItems = Array.from(this.items).slice(0, projects.length);
    
    this.updateGallery();
    this.setupPagination();
    
    // Track projects update
    this.element.dispatchEvent(new CustomEvent('gallery_projects_updated', {
      detail: { 
        projectsCount: projects.length,
        filteredCount: this.state.filteredItems.length
      }
    }));
  }
  
  calculateItemsPerPage() {
    if (this.layout === 'list') return 6;
    if (this.layout === 'masonry') return 9;
    
    const width = window.innerWidth;
    if (width >= 1024) return this.columns * 2;
    if (width >= 768) return 4;
    return 3;
  }
  
  getTotalPages() {
    return Math.ceil(this.state.filteredItems.length / this.state.itemsPerPage);
  }
  
  setLoading(isLoading) {
    this.state.isLoading = isLoading;
    
    if (isLoading) {
      this.element.classList.add('loading');
    } else {
      this.element.classList.remove('loading');
    }
  }
  
  // Public methods
  setLayout(layout) {
    this.layout = layout;
    this.element.dataset.layout = layout;
    this.state.itemsPerPage = this.calculateItemsPerPage();
    this.updateGallery();
    this.setupPagination();
  }
  
  setColumns(columns) {
    this.columns = columns;
    this.element.dataset.columns = columns;
    this.state.itemsPerPage = this.calculateItemsPerPage();
    this.updateGallery();
    this.setupPagination();
  }
  
  goToPage(page) {
    this.state.currentPage = Math.max(1, Math.min(page, this.getTotalPages()));
    this.updateGallery();
    this.updatePaginationButtons();
    this.renderPaginationPages();
  }
  
  clearFilters() {
    this.state.activeFilters.clear();
    this.applyFilter('');
  }
  
  refresh() {
    // Re-initialize the component
    this.state.itemsPerPage = this.calculateItemsPerPage();
    this.state.filteredItems = Array.from(this.items);
    this.state.currentPage = 1;
    this.updateGallery();
    this.setupPagination();
  }
}

// Initialize project_gallery components
window.PETA_COMPONENTS = window.PETA_COMPONENTS || {};
window.PETA_COMPONENTS.project_gallery = (element, props = {}) => {
  return new ProjectGalleryComponent(element);
};

// Auto-initialize
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('[data-component="project_gallery"]').forEach(element => {
    new ProjectGalleryComponent(element);
  });
});