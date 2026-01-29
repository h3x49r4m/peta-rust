// Real-time client-side search functionality

class PetaSearch {
    constructor() {
        this.searchData = null;
        this.debounceTimer = null;
        this.debounceDelay = 300;
        this.minQueryLength = 2;
        this.maxResults = 20;
        this.initialized = false;
    }

    async init() {
        if (this.initialized) return;

        try {
            const response = await fetch('/search.json');
            if (!response.ok) {
                throw new Error(`Failed to load search index: ${response.status}`);
            }
            this.searchData = await response.json();
            this.initialized = true;
            console.log('Search index loaded:', this.searchData.metadata);
        } catch (error) {
            console.error('Failed to initialize search:', error);
            this.showError('Failed to load search functionality. Please refresh the page.');
        }
    }

    search(query, options = {}) {
        if (!this.searchData) {
            console.warn('Search data not loaded');
            return [];
        }

        if (query.length < this.minQueryLength) {
            return [];
        }

        const {
            contentTypes = [],
            tags = [],
            sortBy = 'relevance',
            limit = this.maxResults
        } = options;

        // Tokenize query
        const terms = this.tokenize(query);

        // Score documents
        const scoredDocs = [];
        for (let i = 0; i < this.searchData.documents.length; i++) {
            const doc = this.searchData.documents[i];

            // Apply filters
            if (contentTypes.length > 0 && !contentTypes.includes(doc.content_type)) {
                continue;
            }
            if (tags.length > 0 && !tags.some(tag => doc.tags.includes(tag))) {
                continue;
            }

            const score = this.calculateScore(doc, terms);
            if (score > 0) {
                scoredDocs.push({
                    document: doc,
                    score: score,
                    highlights: this.generateHighlights(doc, terms)
                });
            }
        }

        // Sort results
        this.sortResults(scoredDocs, sortBy);

        // Return limited results
        return scoredDocs.slice(0, limit);
    }

    tokenize(query) {
        return query
            .toLowerCase()
            .split(/\s+/)
            .map(term => term.replace(/[.,\/#!$%\^&\*;:{}=\-_`~()]/g, ''))
            .filter(term => term.length > 0);
    }

    calculateScore(doc, terms) {
        let score = 0;
        const titleLower = doc.title.toLowerCase();
        const excerptLower = doc.excerpt.toLowerCase();
        const contentLower = doc.content.toLowerCase();
        const tagsLower = doc.tags.map(t => t.toLowerCase());

        for (const term of terms) {
            // Title matches (highest weight)
            if (titleLower.includes(term)) {
                score += 10;
            }

            // Tag matches (high weight)
            if (tagsLower.some(tag => tag.includes(term))) {
                score += 5;
            }

            // Excerpt matches (medium weight)
            if (excerptLower.includes(term)) {
                score += 3;
            }

            // Content matches (lower weight)
            if (contentLower.includes(term)) {
                score += 1;
            }

            // Exact phrase match (bonus)
            if (titleLower === term || excerptLower.includes(term)) {
                score += 2;
            }
        }

        // Recency bonus
        const date = new Date(doc.date);
        const daysOld = (Date.now() - date) / (1000 * 60 * 60 * 24);
        if (daysOld < 30) {
            score += 2;
        } else if (daysOld < 365) {
            score += 1;
        }

        return score;
    }

    generateHighlights(doc, terms) {
        const highlights = [];
        const excerptLower = doc.excerpt.toLowerCase();

        for (const term of terms) {
            if (doc.title.toLowerCase().includes(term)) {
                highlights.push({
                    type: 'title',
                    text: doc.title
                });
                break;
            }
        }

        for (const term of terms) {
            const pos = excerptLower.indexOf(term);
            if (pos !== -1) {
                const start = Math.max(0, pos - 50);
                const end = Math.min(doc.excerpt.length, pos + term.length + 50);
                let snippet = doc.excerpt.substring(start, end);
                if (start > 0) snippet = '...' + snippet;
                if (end < doc.excerpt.length) snippet = snippet + '...';

                highlights.push({
                    type: 'excerpt',
                    text: snippet
                });
                break;
            }
        }

        return highlights;
    }

    sortResults(results, sortBy) {
        switch (sortBy) {
            case 'relevance':
                results.sort((a, b) => b.score - a.score);
                break;
            case 'date':
                results.sort((a, b) => new Date(b.document.date) - new Date(a.document.date));
                break;
            case 'title':
                results.sort((a, b) => a.document.title.localeCompare(b.document.title));
                break;
        }
    }

    debounce(func, ...args) {
        clearTimeout(this.debounceTimer);
        this.debounceTimer = setTimeout(() => func(...args), this.debounceDelay);
    }

    showError(message) {
        const resultsDiv = document.getElementById('searchResults');
        if (resultsDiv) {
            resultsDiv.innerHTML = `
                <div class="search-error" style="color: #ef4444; text-align: center; padding: 2rem;">
                    <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="margin-bottom: 1rem;">
                        <circle cx="12" cy="12" r="10"/>
                        <line x1="12" y1="8" x2="12" y2="12"/>
                        <line x1="12" y1="16" x2="12.01" y2="16"/>
                    </svg>
                    <p>${message}</p>
                </div>
            `;
        }
    }

    renderResults(results, query) {
        const resultsDiv = document.getElementById('searchResults');

        if (!resultsDiv) return;

        if (results.length === 0) {
            resultsDiv.innerHTML = `
                <div class="search-no-results" style="color: var(--text-secondary); text-align: center; padding: 3rem;">
                    <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="margin-bottom: 1rem; opacity: 0.5;">
                        <circle cx="11" cy="11" r="8"/>
                        <path d="m21 21-4.35-4.35"/>
                    </svg>
                    <p style="font-size: 1.125rem; margin-bottom: 0.5rem;">No results found</p>
                    <p style="font-size: 0.9375rem;">Try different keywords or check your spelling</p>
                </div>
            `;
            return;
        }

        const typeIcons = {
            article: '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/><polyline points="10 9 9 9 8 9"/></svg>',
            book: '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/><path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/></svg>',
            snippet: '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/></svg>',
            project: '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 17 12 22 22 17"/><polyline points="2 12 12 17 22 12"/></svg>'
        };

        let html = `
            <div class="search-meta" style="color: var(--text-secondary); margin-bottom: 1.5rem; padding: 0.75rem 1rem; background: var(--background-secondary); border-radius: 8px;">
                Found <strong>${results.length}</strong> result${results.length !== 1 ? 's' : ''} for "<strong>${this.escapeHtml(query)}</strong>"
            </div>
            <div class="search-results-list" style="display: flex; flex-direction: column; gap: 1rem;">
        `;

        for (const result of results) {
            const doc = result.document;
            const icon = typeIcons[doc.content_type] || typeIcons.article;
            const highlight = result.highlights.find(h => h.type === 'excerpt');

            html += `
                <div class="search-result-item" style="background: var(--background-primary); border: 1px solid var(--border-color); border-radius: 12px; padding: 1.25rem; transition: all 0.2s ease;">
                    <div class="result-header" style="display: flex; align-items: flex-start; gap: 0.75rem; margin-bottom: 0.75rem;">
                        <span class="result-type-icon" style="color: var(--color-primary-500); display: flex; align-items: center; flex-shrink: 0; margin-top: 2px;">
                            ${icon}
                        </span>
                        <div style="flex: 1; min-width: 0;">
                            <h3 style="margin: 0; font-size: 1.125rem; font-weight: 600;">
                                <a href="${doc.url}" style="color: var(--text-color); text-decoration: none; transition: color 0.2s ease;" onmouseover="this.style.color='var(--color-primary-500)'" onmouseout="this.style.color='var(--text-color)'">
                                    ${this.highlightText(doc.title, query)}
                                </a>
                            </h3>
                            <div class="result-meta" style="display: flex; align-items: center; gap: 1rem; margin-top: 0.5rem; font-size: 0.875rem; color: var(--text-muted);">
                                <span class="result-type" style="text-transform: capitalize; font-weight: 500;">${doc.content_type}</span>
                                <span>•</span>
                                <span class="result-date">${this.formatDate(doc.date)}</span>
                                ${doc.reading_time ? `<span>•</span><span class="result-time">${doc.reading_time} min read</span>` : ''}
                            </div>
                        </div>
                        <div class="result-score" style="font-size: 0.75rem; color: var(--text-muted); opacity: 0.6;">
                            ${Math.round(result.score * 10) / 10}
                        </div>
                    </div>
                    ${doc.tags.length > 0 ? `
                        <div class="result-tags" style="display: flex; flex-wrap: wrap; gap: 0.5rem; margin-bottom: 0.75rem;">
                            ${doc.tags.map(tag => `<span class="tag" style="font-size: 0.75rem; padding: 0.25rem 0.75rem; background: var(--background-secondary); border-radius: 9999px; color: var(--text-muted);">${this.escapeHtml(tag)}</span>`).join('')}
                        </div>
                    ` : ''}
                    ${highlight ? `
                        <p class="result-excerpt" style="margin: 0; color: var(--text-secondary); line-height: 1.6; font-size: 0.9375rem;">
                            ${this.highlightText(highlight.text, query)}
                        </p>
                    ` : `
                        <p class="result-excerpt" style="margin: 0; color: var(--text-secondary); line-height: 1.6; font-size: 0.9375rem;">
                            ${this.escapeHtml(doc.excerpt)}
                        </p>
                    `}
                </div>
            `;
        }

        html += '</div>';
        resultsDiv.innerHTML = html;
    }

    highlightText(text, query) {
        if (!query || query.length < 2) return this.escapeHtml(text);
        const terms = this.tokenize(query);
        let result = this.escapeHtml(text);

        for (const term of terms) {
            const regex = new RegExp(`(${this.escapeRegex(term)})`, 'gi');
            result = result.replace(regex, '<mark style="background: rgba(59, 130, 246, 0.2); color: var(--color-primary-700); padding: 0.125rem 0.25rem; border-radius: 3px; font-weight: 500;">$1</mark>');
        }

        return result;
    }

    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }

    escapeRegex(string) {
        return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    }

    formatDate(dateStr) {
        const date = new Date(dateStr);
        return date.toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' });
    }
}

// Initialize search functionality
document.addEventListener('DOMContentLoaded', async () => {
    const searchInput = document.getElementById('searchInput');
    const resultsDiv = document.getElementById('searchResults');

    if (!searchInput || !resultsDiv) {
        console.warn('Search elements not found');
        return;
    }

    const petaSearch = new PetaSearch();

    // Initialize search index
    await petaSearch.init();

    // Set up search handler
    const handleSearch = async (query) => {
        const trimmedQuery = query.trim();

        if (trimmedQuery.length < petaSearch.minQueryLength) {
            resultsDiv.innerHTML = `
                <p style="color: var(--text-secondary); text-align: center; padding: 3rem;">
                    <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="margin-bottom: 1rem; opacity: 0.5;">
                        <circle cx="11" cy="11" r="8"/>
                        <path d="m21 21-4.35-4.35"/>
                    </svg>
                    Enter at least ${petaSearch.minQueryLength} characters to search
                </p>
            `;
            return;
        }

        // Show loading state
        resultsDiv.innerHTML = `
            <div class="search-loading" style="text-align: center; padding: 3rem; color: var(--text-muted);">
                <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="animation: spin 1s linear infinite; margin-bottom: 1rem;">
                    <path d="M21 12a9 9 0 1 1-6.219-8.56"/>
                </svg>
                <p>Searching...</p>
            </div>
            <style>
                @keyframes spin {
                    from { transform: rotate(0deg); }
                    to { transform: rotate(360deg); }
                }
            </style>
        `;

        // Perform search
        const results = petaSearch.search(trimmedQuery);

        // Render results
        petaSearch.renderResults(results, trimmedQuery);
    };

    // Debounced search on input
    searchInput.addEventListener('input', (e) => {
        petaSearch.debounce(handleSearch, e.target.value);
    });

    // Handle URL query parameter
    const urlParams = new URLSearchParams(window.location.search);
    const queryParam = urlParams.get('q');
    if (queryParam) {
        searchInput.value = queryParam;
        handleSearch(queryParam);
    }
});