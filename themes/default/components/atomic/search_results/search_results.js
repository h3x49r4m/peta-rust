// Search results rendering functionality

class SearchResultsRenderer {
    constructor() {
        this.typeIcons = {
            article: '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/><polyline points="10 9 9 9 8 9"/></svg>',
            book: '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/><path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/></svg>',
            snippet: '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/></svg>',
            project: '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 17 12 22 22 17"/><polyline points="2 12 12 17 22 12"/></svg>'
        };
    }

    renderEmpty() {
        const resultsDiv = document.getElementById('searchResults');
        if (!resultsDiv) return;

        resultsDiv.innerHTML = `
            <p class="search-results-empty">
                <svg class="search-empty-icon" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <circle cx="11" cy="11" r="8"/>
                    <path d="m21 21-4.35-4.35"/>
                </svg>
                Enter a search term to find content.
            </p>
        `;
    }

    renderLoading() {
        const resultsDiv = document.getElementById('searchResults');
        if (!resultsDiv) return;

        resultsDiv.innerHTML = `
            <div class="search-loading">
                <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M21 12a9 9 0 1 1-6.219-8.56"/>
                </svg>
                <p>Searching...</p>
            </div>
        `;
    }

    renderNoResults() {
        const resultsDiv = document.getElementById('searchResults');
        if (!resultsDiv) return;

        resultsDiv.innerHTML = `
            <div class="search-no-results">
                <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <circle cx="11" cy="11" r="8"/>
                    <path d="m21 21-4.35-4.35"/>
                </svg>
                <p>No results found</p>
                <p>Try different keywords or check your spelling</p>
            </div>
        `;
    }

    renderError(message) {
        const resultsDiv = document.getElementById('searchResults');
        if (!resultsDiv) return;

        resultsDiv.innerHTML = `
            <div class="search-error">
                <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <circle cx="12" cy="12" r="10"/>
                    <line x1="12" y1="8" x2="12" y2="12"/>
                    <line x1="12" y1="16" x2="12.01" y2="16"/>
                </svg>
                <p>${this.escapeHtml(message)}</p>
            </div>
        `;
    }

    renderResults(results, query) {
        const resultsDiv = document.getElementById('searchResults');
        if (!resultsDiv) return;

        if (results.length === 0) {
            this.renderNoResults();
            return;
        }

        let html = `
            <div class="search-meta">
                Found <strong>${results.length}</strong> result${results.length !== 1 ? 's' : ''} for "<strong>${this.escapeHtml(query)}</strong>"
            </div>
            <div class="search-results-list">
        `;

        for (const result of results) {
            const doc = result.document;
            const icon = this.typeIcons[doc.content_type] || this.typeIcons.article;
            const highlight = result.highlights.find(h => h.type === 'excerpt');

            html += `
                <div class="search-result-item">
                    <div class="result-header">
                        <span class="result-type-icon">${icon}</span>
                        <div>
                            <h3>
                                <a href="${doc.url}">${this.highlightText(doc.title, query)}</a>
                            </h3>
                            <div class="result-meta">
                                <span class="result-type">${doc.content_type}</span>
                                <span>•</span>
                                <span class="result-date">${this.formatDate(doc.date)}</span>
                                ${doc.reading_time ? `<span>•</span><span class="result-time">${doc.reading_time} min read</span>` : ''}
                            </div>
                        </div>
                        <div class="result-score">${Math.round(result.score * 10) / 10}</div>
                    </div>
                    ${doc.tags.length > 0 ? `
                        <div class="result-tags">
                            ${doc.tags.map(tag => `<span class="tag">${this.escapeHtml(tag)}</span>`).join('')}
                        </div>
                    ` : ''}
                    ${highlight ? `
                        <p class="result-excerpt">${this.highlightText(highlight.text, query)}</p>
                    ` : `
                        <p class="result-excerpt">${this.escapeHtml(doc.excerpt)}</p>
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
            result = result.replace(regex, '<mark>$1</mark>');
        }

        return result;
    }

    tokenize(query) {
        return query
            .toLowerCase()
            .split(/\s+/)
            .map(term => term.replace(/[.,\/#!$%\^&\*;:{}=\-_`~()]/g, ''))
            .filter(term => term.length > 0);
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

// Export for use in search page
window.SearchResultsRenderer = SearchResultsRenderer;