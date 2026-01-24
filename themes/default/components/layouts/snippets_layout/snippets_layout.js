// Snippets Layout JavaScript
class SnippetsLayout {
  constructor(element) {
    this.element = element;
    this.init();
  }
  
  init() {
    this.setupLanguageFilter();
    this.setupSnippetCards();
    this.setupCopyToClipboard();
  }
  
  setupLanguageFilter() {
    const languageFilter = this.element.querySelector('#language-filter');
    if (languageFilter) {
      languageFilter.addEventListener('change', (e) => {
        const language = e.target.value;
        const url = new URL(window.location);
        if (language) {
          url.searchParams.set('language', language);
        } else {
          url.searchParams.delete('language');
        }
        window.location = url.toString();
      });
    }
  }
  
  setupSnippetCards() {
    const cards = this.element.querySelectorAll('.snippet-card');
    cards.forEach(card => {
      card.addEventListener('mouseenter', () => {
        card.classList.add('snippet-card--hover');
      });
      card.addEventListener('mouseleave', () => {
        card.classList.remove('snippet-card--hover');
      });
    });
  }
  
  setupCopyToClipboard() {
    const copyButtons = this.element.querySelectorAll('.snippet-link.secondary');
    copyButtons.forEach(button => {
      button.addEventListener('click', (e) => {
        e.preventDefault();
        const snippetCard = button.closest('.snippet-card');
        const preview = snippetCard.querySelector('.snippet-preview');
        
        if (preview) {
          navigator.clipboard.writeText(preview.textContent).then(() => {
            const originalText = button.textContent;
            button.textContent = 'Copied!';
            button.style.background = 'var(--success-color, #10b981)';
            
            setTimeout(() => {
              button.textContent = originalText;
              button.style.background = '';
            }, 2000);
          });
        }
      });
    });
  }
}

// Initialize snippets layout
document.addEventListener('DOMContentLoaded', () => {
  const snippetsLayout = document.querySelector('.snippets-layout');
  if (snippetsLayout) {
    new SnippetsLayout(snippetsLayout);
  }
});