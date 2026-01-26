// Book TOC Component JavaScript
document.addEventListener('DOMContentLoaded', function() {
  const bookTocComponents = document.querySelectorAll('[data-component="book_toc"]');
  
  bookTocComponents.forEach(function(component) {
    const toggleBtn = component.querySelector('#book-toc-toggle');
    const tocPanel = component.querySelector('.book-toc-panel');
    const tocContent = component.querySelector('#book-toc-content');
    
    if (!toggleBtn || !tocPanel || !tocContent) {
      console.error('Book TOC toggle button, panel, or content not found');
      return;
    }
    
    // Toggle button functionality
    toggleBtn.addEventListener('click', function() {
      const isExpanded = this.getAttribute('aria-expanded') === 'true';
      
      // Toggle the expanded state
      this.setAttribute('aria-expanded', !isExpanded);
      
      // Toggle icon rotation
      if (isExpanded) {
        this.style.transform = 'rotate(0deg)';
      } else {
        this.style.transform = 'rotate(180deg)';
      }
    });
    
    // Get book path from current URL
    const currentPath = window.location.pathname;
    const pathParts = currentPath.split('/').filter(p => p);
    
    // Extract book directory name from path (e.g., /books/quantum-computing-a-complete-guide/quantum-algorithms.html)
    let bookDirName = '';
    if (pathParts[0] === 'books' && pathParts.length >= 2) {
      bookDirName = pathParts[1];
    }
    
    // Function to load TOC from index page
    async function loadBookTOC() {
      if (!bookDirName) {
        console.error('Could not determine book directory name from URL');
        return;
      }
      
      const indexUrl = `/books/${bookDirName}/index.html`;
      
      try {
        const response = await fetch(indexUrl);
        if (!response.ok) {
          throw new Error(`Failed to fetch book index: ${response.status}`);
        }
        
        const html = await response.text();
        const parser = new DOMParser();
        const doc = parser.parseFromString(html, 'text/html');
        
        // Find TOC tree in the index page
        const tocTree = doc.querySelector('.toc-tree');
        if (tocTree) {
          // Create a filtered TOC with only chapter links
          const filteredToc = document.createElement('div');
          filteredToc.className = 'toc-tree';
          
          // Get all TOC items
          const tocItems = tocTree.querySelectorAll('.toc-item');
          let foundTocEnd = false;
          
          // Collect chapter URLs to fetch titles
          const chapterUrls = [];
          
          tocItems.forEach(function(item) {
            const link = item.querySelector('a');
            if (link) {
              const href = link.getAttribute('href');
              const text = link.textContent.trim();
              
              // Skip separators (dashes)
              if (/^[-\s]+$/.test(text)) {
                return;
              }
              
              // Stop after we've passed the actual chapter list
              // The actual chapters are the ones that match the toctree in index.rst
              // They are typically short, single-word or hyphenated names
              // Section headers like "What This Book Covers" are longer and descriptive
              
              // Check if this is a section header (contains multiple words or special patterns)
              const isSectionHeader = text.includes(' ') && 
                                     (text.includes(':') || 
                                      text.match(/^[A-Z]/) && 
                                      !text.match(/^[a-z-]+$/) &&
                                      !text.match(/^[a-z]+-[a-z]+$/));
              
              // Also check if it's a numbered list item (like "1. Supervised Learning")
              const isNumberedList = /^\d+\.\s/.test(text);
              
              // Stop processing if we hit a section header after chapters
              if ((isSectionHeader || isNumberedList) && !foundTocEnd) {
                // Check if we've seen any chapters yet
                const hasChapters = filteredToc.querySelectorAll('.toc-item').length > 0;
                if (hasChapters) {
                  foundTocEnd = true;
                  return;
                }
              }
              
              if (!foundTocEnd) {
                // Clone the item
                const clonedItem = item.cloneNode(true);
                const clonedLink = clonedItem.querySelector('a');
                
                if (clonedLink && href && !href.startsWith('http')) {
                  // Convert relative URLs to absolute book URLs
                  const fullUrl = `/books/${bookDirName}/${href}`;
                  clonedLink.setAttribute('href', fullUrl);
                  clonedLink.setAttribute('data-chapter-link', 'true');
                  clonedLink.setAttribute('data-chapter-url', fullUrl);
                  filteredToc.appendChild(clonedItem);
                  
                  // Add to list of chapters to fetch titles
                  chapterUrls.push(fullUrl);
                }
              }
            }
          });
          
          // Fetch titles for all chapters
          if (chapterUrls.length > 0) {
            Promise.all(chapterUrls.map(async function(url) {
              try {
                const response = await fetch(url);
                if (response.ok) {
                  const html = await response.text();
                  const parser = new DOMParser();
                  const doc = parser.parseFromString(html, 'text/html');
                  
                  // Find the book title in the page
                  const titleElement = doc.querySelector('.book-title');
                  if (titleElement) {
                    return { url: url, title: titleElement.textContent.trim() };
                  }
                }
              } catch (e) {
                console.error('Failed to fetch title for', url, e);
              }
              return null;
            })).then(function(titles) {
              // Update TOC links with actual titles
              titles.forEach(function(titleInfo) {
                if (titleInfo) {
                  const link = filteredToc.querySelector(`a[data-chapter-url="${titleInfo.url}"]`);
                  if (link) {
                    link.textContent = titleInfo.title;
                  }
                }
              });
            });
          }
          
          tocContent.innerHTML = '';
          tocContent.appendChild(filteredToc);
          
          // Handle TOC link clicks
          const tocLinks = tocContent.querySelectorAll('a[data-chapter-link]');
          tocLinks.forEach(function(link) {
            link.addEventListener('click', function(e) {
              e.preventDefault();
              const chapterUrl = link.getAttribute('href');
              if (chapterUrl) {
                window.location.href = chapterUrl;
              }
            });
          });
        } else {
          console.error('TOC tree not found in book index page');
        }
      } catch (error) {
        console.error('Failed to load book TOC:', error);
      }
    }
    
    // Load TOC when panel is opened for the first time
    let tocLoaded = false;
    toggleBtn.addEventListener('click', function() {
      if (!tocLoaded) {
        loadBookTOC();
        tocLoaded = true;
      }
    });
  });
});