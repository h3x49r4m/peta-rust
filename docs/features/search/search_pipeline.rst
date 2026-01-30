Search Pipeline
===============

This document describes the complete search pipeline from content indexing to query execution.

Build Time (Server-Side)
-------------------------

When you run ``peta build``, the following steps occur:

1. **Parse RST Content**

   The RST files in ``_content/`` are parsed into ``RstContent`` objects::

   _content/
   ├── articles/*.rst   ──► parse_content() ──► Vec<RstContent>
   ├── books/*.rst
   ├── snippets/*.rst
   └── projects/*.rst

2. **Build Search Index**

   The ``SearchIndex::build()`` method processes each ``RstContent`` item::

   SearchIndex::build(Vec<RstContent>)
   │
   ├─ FOR EACH RstContent:
   │   ├─ create_document() ──► SearchDocument
   │   │   {id, title, excerpt, url, content_type, tags,
   │   │    date, author, content, word_count, reading_time}
   │   │
   │   └─ index_document() ──► Builds inverted indexes:
   │       terms: HashMap<term, Vec<document_index>>
   │       tags: HashMap<tag, Vec<document_index>>
   │       content_types: HashMap<type, Vec<document_index>>
   │
   └─ update_metadata() ──► SearchMetadata
       {version, build_timestamp, total_documents, total_terms,
        avg_document_length}

3. **Generate JSON Files**

   Two JSON files are generated for client-side search::

   generate_client_search() ──► JSON serialization
   │
   ├─► _out/dist/search.json (main search index)
   │   {
   │     "documents": [SearchDocument, ...],
   │     "terms": {term: [doc_indices], ...},
   │     "tags": {tag: [doc_indices], ...},
   │     "content_types": {type: [doc_indices], ...},
   │     "metadata": SearchMetadata
   │   }
   │
   └─► _out/dist/contexts/search.json (enhanced)
       {
         "index": {version, generated, total_pages},
         "pages": [{
           id, title, url, content, description, type, tags,
           category, date, author, views, reading_time
         }]
       }

4. **Render HTML**

   The search page is rendered with a two-column layout::

   render_template("search.html") ──► _out/dist/search.html
   │
   └─ Two-column layout:
       ├─ 75%: {{ component("search_results") }} ──► #searchResults container
       └─ 25%: {{ component("tag_cloud") }}

Runtime (Client-Side)
---------------------

When a user visits ``/search.html``, the following occurs:

5. **Page Load - Initialize Search**

   The browser loads the page and initializes the search functionality::

   Browser loads: /search.html
   │
   └─► themes/default/components/atomic/search_bar/search_bar.js
       │
       └─► PetaSearch.init()
             │
             └─► fetch('/search.json') ──► loads JSON into memory
                   │
                   ▼
             this.searchData = {
               documents: [...],
               terms: {...},
               tags: {...},
               content_types: {...},
               metadata: {...}
             }

6. **User Types Search Query**

   When a user types a query (e.g., "machine learning")::

   PetaSearch.search("machine learning")
   │
   ├─► tokenize() ──► ["machine", "learning"]
   │
   ├─► FOR EACH document in searchData.documents:
   │   │
   │   ├─ Apply filters (contentTypes, tags)
   │   │
   │   └─ calculateScore(doc, ["machine", "learning"]) ──► score
   │       ├─ Title match:        +10 points
   │       ├─ Tag match:          +5 points
   │       ├─ Excerpt match:      +3 points
   │       ├─ Content match:      +1 point
   │       ├─ Exact phrase bonus: +2 points
   │       └─ Recency bonus:     +1 or +2 points
   │
   ├─► generateHighlights(doc, terms) ──► matching text snippets
   │
   ├─► sortResults(scoredDocs) ──► sort by score (descending)
   │
   └─► slice(0, 20) ──► limit to max 20 results
         │
         ▼
   [{document, score, highlights}, ...]

7. **Render Results**

   Results are rendered into the DOM::

   renderResults(results, query)
   │
   └─► Update DOM: document.getElementById('searchResults').innerHTML = ...
         │
         ▼
   <div class="search-result-item">
     <h3><a href="/articles/ml-basics">Machine Learning Basics</a></h3>
     <p class="excerpt">Introduction to <mark>machine learning</mark>...</p>
     <div class="meta">article • Jan 15, 2026 • #ml #python</div>
   </div>

Data Structures
---------------

SearchDocument
~~~~~~~~~~~~~

Indexed per content item::

{
  id: "article_ml_basics",
  title: "Machine Learning Basics",
  excerpt: "Introduction to ML...",
  url: "/articles/ml-basics",
  content_type: "article",
  tags: ["ml", "python", "ai"],
  date: "2026-01-15",
  author: "John Doe",
  content: "Full text content...",
  word_count: 2500,
  reading_time: 13
}

search.json
~~~~~~~~~~~

The client-side search index file::

{
  "documents": [
    {id, title, excerpt, url, content_type, tags, date, author, content, ...},
    ...
  ],
  "terms": {
    "machine": [0, 5, 12],      // document indices
    "learning": [0, 5, 12, 23],
    ...
  },
  "tags": {
    "ml": [0, 5],
    "python": [0, 12, 23],
    ...
  },
  "content_types": {
    "article": [0, 5, 12],
    "book": [1, 6],
    ...
  },
  "metadata": {
    "version": "1.0.0",
    "build_timestamp": "2026-01-30T12:00:00Z",
    "total_documents": 50,
    "total_terms": 5000,
    "avg_document_length": 1500.5
  }
}

Scoring Algorithm
-----------------

For each term in the query, points are awarded as follows:

- Title match: +10 points
- Tag match: +5 points
- Excerpt match: +3 points
- Content match: +1 point
- Exact phrase match: +2 points (bonus)
- Recency bonus: +2 points (<30 days), +1 point (<365 days)

Total Score = Σ(term_scores) + recency_bonus

Results are sorted by score in descending order.