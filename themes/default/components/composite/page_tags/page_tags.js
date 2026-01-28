// Page Tags Component JavaScript
document.addEventListener('DOMContentLoaded', function() {
  const pageTagsComponents = document.querySelectorAll('[data-component="page_tags"]');

  pageTagsComponents.forEach(function(component) {
    // Initialize page tags component
    console.log('Page tags component initialized');

    // Get the page title link and ensure it has the correct URL
    const pageTitleLink = component.querySelector('.page-title-link');
    if (pageTitleLink) {
      const currentHref = pageTitleLink.getAttribute('href');

      // Determine the correct URL based on the page title
      const pageTitle = component.querySelector('.page-title').textContent.trim().toLowerCase();
      const pageUrls = {
        'articles': '/articles.html',
        'books': '/books.html',
        'projects': '/projects.html',
        'snippets': '/snippets.html'
      };

      let targetUrl = currentHref;

      // If current href is a book page (contains /books/), redirect to books.html
      if (currentHref && currentHref.includes('/books/')) {
        targetUrl = '/books.html';
      } else if (currentHref && currentHref.includes('/articles/')) {
        targetUrl = '/articles.html';
      } else if (currentHref && currentHref.includes('/projects/')) {
        targetUrl = '/projects.html';
      } else if (currentHref && currentHref.includes('/snippets/')) {
        targetUrl = '/snippets.html';
      } else if (pageUrls[pageTitle]) {
        targetUrl = pageUrls[pageTitle];
      } else if (!currentHref || currentHref === '#') {
        // Default to home if page is not recognized
        targetUrl = '/';
      }

      pageTitleLink.setAttribute('href', targetUrl);
    }

    // Tag filtering functionality
    component.addEventListener('click', function(e) {
      const tagLink = e.target.closest('.tag-cloud-tag');
      if (tagLink) {
        e.preventDefault();
        const tagName = tagLink.textContent.trim().replace(/\s*\(\d+\)\s*$/, '');

        // Check if there's a grid_cards container to filter
        const gridCardsContainer = document.querySelector('[data-component="grid_cards"]');
        
        if (gridCardsContainer) {
          // Filter grid cards on current page
          // Update active tag state
          document.querySelectorAll('.tag-cloud-tag').forEach(tag => {
            tag.classList.remove('active');
          });
          tagLink.classList.add('active');

          // Filter grid cards
          filterGridCardsByTag(tagName);

          // Update URL hash
          history.replaceState(null, '', '#' + encodeURIComponent(tagName));
        } else {
          // No grid_cards container, redirect to list page with tag
          const pageTitleLink = component.querySelector('.page-title-link');
          if (pageTitleLink) {
            const listPageUrl = pageTitleLink.getAttribute('href');
            if (listPageUrl && listPageUrl !== '#') {
              window.location.href = listPageUrl + '#' + encodeURIComponent(tagName);
            }
          }
        }
      }
    });

    // Check for tag in URL hash on page load
    const hash = window.location.hash.slice(1);
    if (hash) {
      const decodedTag = decodeURIComponent(hash);
      setTimeout(() => {
        const tagElement = Array.from(document.querySelectorAll('.tag-cloud-tag'))
          .find(tag => tag.textContent.trim().replace(/\s*\(\d+\)\s*$/, '') === decodedTag);
        if (tagElement) {
          tagElement.click();
        }
      }, 100);
    }
  });
});

function filterGridCardsByTag(tagName) {
  const gridCardsContainer = document.querySelector('[data-component="grid_cards"]');
  if (!gridCardsContainer) return;

  const cards = gridCardsContainer.querySelectorAll('.grid-card');
  let visibleCount = 0;

  cards.forEach(card => {
    const cardTags = card.querySelectorAll('.card-tag');
    let hasTag = false;

    cardTags.forEach(cardTag => {
      if (cardTag.textContent.trim() === tagName) {
        hasTag = true;
      }
    });

    if (hasTag) {
      card.style.display = '';
      visibleCount++;
    } else {
      card.style.display = 'none';
    }
  });

  // Update title to show filtered tag
  const titleElement = gridCardsContainer.querySelector('.grid-cards-title');
  if (titleElement) {
    const originalTitle = titleElement.getAttribute('data-original-title') || titleElement.textContent;
    if (!titleElement.getAttribute('data-original-title')) {
      titleElement.setAttribute('data-original-title', originalTitle);
    }
    titleElement.textContent = tagName ? `${originalTitle} - Tag: ${tagName}` : originalTitle;
  }

  // Show/hide empty state
  const emptyState = gridCardsContainer.querySelector('.grid-cards-empty');
  if (emptyState) {
    emptyState.style.display = visibleCount === 0 ? '' : 'none';
  }

  // Add "Show all" button if filtered
  let showAllButton = gridCardsContainer.querySelector('.show-all-button');
  if (tagName && visibleCount > 0) {
    if (!showAllButton) {
      showAllButton = document.createElement('button');
      showAllButton.className = 'show-all-button';
      showAllButton.textContent = 'Show All';
      showAllButton.addEventListener('click', function() {
        // Remove active state from all tags
        document.querySelectorAll('.tag-cloud-tag').forEach(tag => {
          tag.classList.remove('active');
        });

        // Show all cards
        cards.forEach(card => {
          card.style.display = '';
        });

        // Reset title
        if (titleElement) {
          titleElement.textContent = titleElement.getAttribute('data-original-title');
        }

        // Hide empty state
        if (emptyState) {
          emptyState.style.display = 'none';
        }

        // Remove button
        showAllButton.remove();

        // Update URL
        history.replaceState(null, '', ' ');
      });

      const header = gridCardsContainer.querySelector('.grid-cards-header');
      if (header) {
        header.appendChild(showAllButton);
      }
    }
  } else if (showAllButton) {
    showAllButton.remove();
  }
}