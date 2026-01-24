// Articles Layout JavaScript
class ArticlesLayout {
  constructor(element) {
    this.element = element;
    this.init();
  }
  
  init() {
    this.setupTagFilter();
    this.setupArticleCards();
    this.setupScrollToTop();
  }
  
  setupTagFilter() {
    const tagSelect = this.element.querySelector('.tag-select');
    if (tagSelect) {
      tagSelect.addEventListener('change', (e) => {
        const tag = e.target.value;
        const url = new URL(window.location);
        if (tag) {
          url.searchParams.set('tag', tag);
        } else {
          url.searchParams.delete('tag');
        }
        window.location = url.toString();
      });
    }
  }
  
  setupArticleCards() {
    const cards = this.element.querySelectorAll('.article-card');
    cards.forEach(card => {
      card.addEventListener('mouseenter', () => {
        card.classList.add('article-card--hover');
      });
      card.addEventListener('mouseleave', () => {
        card.classList.remove('article-card--hover');
      });
    });
  }
  
  setupScrollToTop() {
    const scrollToTopBtn = document.createElement('button');
    scrollToTopBtn.textContent = '↑';
    scrollToTopBtn.style.cssText = `
      position: fixed;
      bottom: 20px;
      right: 20px;
      width: 40px;
      height: 40px;
      border-radius: 50%;
      background: var(--primary-color, #3b82f6);
      color: white;
      border: none;
      cursor: pointer;
      font-size: 18px;
      z-index: 1000;
      opacity: 0.8;
      transition: opacity 0.2s;
    `;
    
    document.body.appendChild(scrollToTopBtn);
    
    scrollToTopBtn.addEventListener('click', () => {
      window.scrollTo({ top: 0, behavior: 'smooth' });
    });
    
    // Show/hide based on scroll position
    window.addEventListener('scroll', () => {
      if (window.pageYOffset > 200) {
        scrollToTopBtn.style.opacity = '1';
      } else {
        scrollToTopBtn.style.opacity = '0.8';
      }
    });
  }
}

// Initialize articles layout
document.addEventListener('DOMContentLoaded', () => {
  const articlesLayout = document.querySelector('.articles-container');
  if (articlesLayout) {
    new ArticlesLayout(articlesLayout);
  }
});

// Table of contents functionality
class TableOfContents {
  constructor(element) {
    this.element = element;
    this.init();
  }
  
  init() {
    this.setupTOCLinks();
    this.setupIntersectionObserver();
  }
  
  setupTOCLinks() {
    const links = this.element.querySelectorAll('.table-of-contents a');
    links.forEach(link => {
      link.addEventListener('click', (e) => {
        e.preventDefault();
        const targetId = link.getAttribute('href').substring(1);
        const targetElement = document.getElementById(targetId);
        if (targetElement) {
          targetElement.scrollIntoView({ behavior: 'smooth' });
        }
      });
    });
  }
  
  setupIntersectionObserver() {
    const sections = this.element.querySelectorAll('h2[id], h3[id], h4[id]');
    const observer = new IntersectionObserver((entries) => {
      entries.forEach(entry => {
        if (entry.isIntersecting) {
          const link = this.getLinkForSection(entry.target);
          if (link) {
            this.setActiveItem(link);
          }
        }
      });
    }, { threshold: 0.1 });
    
    sections.forEach(section => observer.observe(section));
  }
  
  getLinkForSection(section) {
    const id = section.id;
    return this.element.querySelector(`.table-of-contents a[href="#${id}"]`);
  }
  
  setActiveItem(activeLink) {
    const links = this.element.querySelectorAll('.table-of-contents a');
    links.forEach(link => {
      link.classList.remove('active');
    });
    activeLink.classList.add('active');
  }
}

// Initialize table of contents
document.addEventListener('DOMContentLoaded', () => {
  const toc = document.querySelector('.table-of-contents');
  if (toc) {
    new TableOfContents(toc);
  }
});