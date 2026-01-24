// Search Hook - Provides search functionality across the site
class useSearch {
  constructor() {
    this.searchIndex = null;
    this.searchData = null;
    this.isInitialized = false;
    this.init();
  }
  
  async init() {
    try {
      // Load search index
      const response = await fetch('/assets/js/search.json');
      this.searchData = await response.json();
      this.searchIndex = this.buildSearchIndex(this.searchData);
      this.isInitialized = true;
    } catch (error) {
      console.error('Failed to initialize search:', error);
    }
  }
  
  buildSearchIndex(data) {
    const index = {};
    
    // Index articles
    if (data.articles) {
      data.articles.forEach(article => {
        const id = `article-${article.id}`;
        index[id] = {
          type: 'article',
          title: article.title,
          content: article.content || '',
          excerpt: article.excerpt || '',
          tags: article.tags || [],
          url: article.url,
          date: article.date,
          author: article.author
        };
      });
    }
    
    // Index books
    if (data.books) {
      data.books.forEach(book => {
        const id = `book-${book.id}`;
        index[id] = {
          type: 'book',
          title: book.title,
          content: book.content || '',
          excerpt: book.excerpt || '',
          tags: book.tags || [],
          url: book.url,
          author: book.author,
          chapters: book.chapters || []
        };
      });
    }
    
    // Index snippets
    if (data.snippets) {
      data.snippets.forEach(snippet => {
        const id = `snippet-${snippet.id}`;
        index[id] = {
          type: 'snippet',
          title: snippet.title,
          content: snippet.content || '',
          excerpt: snippet.excerpt || '',
          language: snippet.language,
          url: snippet.url,
          tags: snippet.tags || []
        };
      });
    }
    
    // Index projects
    if (data.projects) {
      data.projects.forEach(project => {
        const id = `project-${project.id}`;
        index[id] = {
          type: 'project',
          title: project.title,
          content: project.content || '',
          excerpt: project.excerpt || '',
          technologies: project.technologies || [],
          url: project.url,
          status: project.status
        };
      });
    }
    
    return index;
  }
  
  search(query, options = {}) {
    if (!this.isInitialized) {
      return { results: [], total: 0 };
    }
    
    const {
      types = ['article', 'book', 'snippet', 'project'],
      tags = [],
      languages = [],
      technologies = [],
      limit = 20,
      offset = 0
    } = options;
    
    const searchTerm = query.toLowerCase().trim();
    const results = [];
    
    // Search through indexed content
    for (const [id, item] of Object.entries(this.searchIndex)) {
      // Filter by type
      if (!types.includes(item.type)) continue;
      
      // Filter by tags
      if (tags.length > 0 && (!item.tags || !tags.some(tag => item.tags.includes(tag)))) continue;
      
      // Filter by languages (for snippets)
      if (languages.length > 0 && item.type === 'snippet' && !languages.includes(item.language)) continue;
      
      // Filter by technologies (for projects)
      if (technologies.length > 0 && item.type === 'project' && 
          (!item.technologies || !technologies.some(tech => item.technologies.includes(tech)))) continue;
      
      // Calculate relevance score
      let score = 0;
      
      // Title matches (highest weight)
      if (item.title && item.title.toLowerCase().includes(searchTerm)) {
        score += 10;
      }
      
      // Content matches
      if (item.content && item.content.toLowerCase().includes(searchTerm)) {
        score += 5;
      }
      
      // Excerpt matches
      if (item.excerpt && item.excerpt.toLowerCase().includes(searchTerm)) {
        score += 3;
      }
      
      // Tag matches
      if (item.tags && item.tags.some(tag => tag.toLowerCase().includes(searchTerm))) {
        score += 2;
      }
      
      // Technology matches (for projects)
      if (item.technologies && item.technologies.some(tech => tech.toLowerCase().includes(searchTerm))) {
        score += 2;
      }
      
      // Language matches (for snippets)
      if (item.language && item.language.toLowerCase().includes(searchTerm)) {
        score += 2;
      }
      
      if (score > 0) {
        results.push({
          id,
          ...item,
          score
        });
      }
    }
    
    // Sort by score (descending)
    results.sort((a, b) => b.score - a.score);
    
    // Apply pagination
    const total = results.length;
    const paginatedResults = results.slice(offset, offset + limit);
    
    return {
      results: paginatedResults,
      total,
      hasMore: offset + limit < total
    };
  }
  
  getSuggestions(query, limit = 5) {
    if (!this.isInitialized || !query.trim()) {
      return [];
    }
    
    const searchTerm = query.toLowerCase().trim();
    const suggestions = new Set();
    
    // Extract suggestions from titles
    for (const item of Object.values(this.searchIndex)) {
      if (item.title && item.title.toLowerCase().includes(searchTerm)) {
        suggestions.add(item.title);
      }
    }
    
    // Extract suggestions from tags
    for (const item of Object.values(this.searchIndex)) {
      if (item.tags) {
        item.tags.forEach(tag => {
          if (tag.toLowerCase().includes(searchTerm)) {
            suggestions.add(tag);
          }
        });
      }
    }
    
    // Extract suggestions from technologies
    for (const item of Object.values(this.searchIndex)) {
      if (item.technologies) {
        item.technologies.forEach(tech => {
          if (tech.toLowerCase().includes(searchTerm)) {
            suggestions.add(tech);
          }
        });
      }
    }
    
    return Array.from(suggestions).slice(0, limit);
  }
  
  getPopularTags(limit = 10) {
    if (!this.isInitialized) {
      return [];
    }
    
    const tagCounts = {};
    
    for (const item of Object.values(this.searchIndex)) {
      if (item.tags) {
        item.tags.forEach(tag => {
          tagCounts[tag] = (tagCounts[tag] || 0) + 1;
        });
      }
    }
    
    return Object.entries(tagCounts)
      .sort(([, a], [, b]) => b - a)
      .slice(0, limit)
      .map(([tag, count]) => ({ tag, count }));
  }
  
  getRecentContent(limit = 10, type = null) {
    if (!this.isInitialized) {
      return [];
    }
    
    const items = Object.values(this.searchIndex)
      .filter(item => !type || item.type === type)
      .filter(item => item.date)
      .sort((a, b) => new Date(b.date) - new Date(a.date))
      .slice(0, limit);
    
    return items;
  }
}

// Export for use in components
window.useSearch = useSearch;

// Auto-initialize search hook
const searchHook = new useSearch();