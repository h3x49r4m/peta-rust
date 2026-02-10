//! JavaScript generator for RST table functionality

use crate::core::Result;

/// JavaScript generator for tables
pub struct TableJsGenerator;

impl TableJsGenerator {
    /// Create a new table JS generator
    pub fn new() -> Self {
        Self
    }

    /// Generate JavaScript for table functionality
    pub fn generate(&self) -> Result<String> {
        let js = r#"
// RST Table JavaScript
(function() {
    'use strict';

    // Initialize tables when DOM is ready
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', initTables);
    } else {
        initTables();
    }

    function initTables() {
        initSearch();
        initSort();
        initCopy();
        initResponsiveScroll();
    }

    // Search functionality
    function initSearch() {
        document.querySelectorAll('.table-search').forEach(input => {
            input.addEventListener('input', function() {
                const table = this.closest('.rst-table').querySelector('table');
                const filter = this.value.toLowerCase();
                const rows = table.querySelectorAll('tbody tr');
                let visibleCount = 0;

                rows.forEach(row => {
                    const text = row.textContent.toLowerCase();
                    const isVisible = text.includes(filter);
                    row.style.display = isVisible ? '' : 'none';
                    if (isVisible) visibleCount++;
                });

                // Update footer info
                updateTableInfo(this.closest('.rst-table'), visibleCount);
            });
        });
    }

    // Sort functionality
    function initSort() {
        document.querySelectorAll('.rst-table th[data-sortable="true"]').forEach(th => {
            th.addEventListener('click', function() {
                const table = this.closest('table');
                const tbody = table.querySelector('tbody');
                const rows = Array.from(tbody.querySelectorAll('tr'));
                const cellIndex = Array.from(this.parentNode.children).indexOf(this);
                const isAsc = this.classList.toggle('sort-asc');

                // Remove sort-desc if present
                if (isAsc) {
                    this.classList.remove('sort-desc');
                } else {
                    this.classList.add('sort-desc');
                }

                // Clear sort indicators from other columns
                this.parentNode.querySelectorAll('th').forEach(header => {
                    if (header !== this) {
                        header.classList.remove('sort-asc', 'sort-desc');
                    }
                });

                // Sort rows
                rows.sort((a, b) => {
                    const aVal = a.children[cellIndex].textContent.trim();
                    const bVal = b.children[cellIndex].textContent.trim();

                    // Try numeric comparison
                    const aNum = parseFloat(aVal);
                    const bNum = parseFloat(bVal);

                    if (!isNaN(aNum) && !isNaN(bNum)) {
                        return isAsc ? aNum - bNum : bNum - aNum;
                    }

                    // String comparison
                    return isAsc ? aVal.localeCompare(bVal) : bVal.localeCompare(aVal);
                });

                // Re-append sorted rows
                rows.forEach(row => tbody.appendChild(row));
            });
        });
    }

    // Copy functionality
    function initCopy() {
        document.querySelectorAll('.table-copy').forEach(btn => {
            btn.addEventListener('click', async function() {
                const table = this.closest('.rst-table').querySelector('table');
                const caption = table.querySelector('caption')?.textContent || '';
                const headers = Array.from(table.querySelectorAll('thead th')).map(th => th.textContent.trim());
                const rows = Array.from(table.querySelectorAll('tbody tr')).map(tr =>
                    Array.from(tr.querySelectorAll('td')).map(td => td.textContent.trim()).join('\t')
                );

                // Build text representation
                let text = '';
                if (caption) text += caption + '\n';
                if (headers.length > 0) text += headers.join('\t') + '\n';
                text += rows.join('\n');

                // Copy to clipboard
                try {
                    await navigator.clipboard.writeText(text);

                    // Show success feedback
                    const originalText = this.textContent;
                    this.textContent = '✓';
                    this.style.background = '#10b981';

                    setTimeout(() => {
                        this.textContent = originalText;
                        this.style.background = '';
                    }, 1500);
                } catch (err) {
                    console.error('Failed to copy table:', err);
                    this.textContent = '✗';
                    this.style.background = '#ef4444';

                    setTimeout(() => {
                        this.textContent = '📋';
                        this.style.background = '';
                    }, 1500);
                }
            });
        });
    }

    // Responsive scroll indicators
    function initResponsiveScroll() {
        document.querySelectorAll('.table-wrapper').forEach(wrapper => {
            const updateScrollIndicator = () => {
                const hasScroll = wrapper.scrollWidth > wrapper.clientWidth;
                const isScrolledRight = wrapper.scrollLeft + wrapper.clientWidth >= wrapper.scrollWidth - 1;
                const isScrolledLeft = wrapper.scrollLeft <= 0;

                wrapper.classList.toggle('has-scroll', hasScroll);
                wrapper.classList.toggle('scroll-start', !isScrolledLeft);
                wrapper.classList.toggle('scroll-end', !isScrolledRight);
            };

            // Initial check
            updateScrollIndicator();

            // Update on scroll and resize
            wrapper.addEventListener('scroll', updateScrollIndicator);
            window.addEventListener('resize', updateScrollIndicator);
        });
    }

    // Update table info in footer
    function updateTableInfo(tableContainer, visibleCount) {
        const footer = tableContainer.querySelector('.table-footer');
        if (footer) {
            const table = tableContainer.querySelector('table');
            const totalRows = table.querySelectorAll('tbody tr').length;
            const colCount = table.querySelectorAll('thead th').length ||
                           table.querySelector('tbody tr')?.children.length || 0;

            if (visibleCount === totalRows) {
                footer.textContent = `${totalRows} rows × ${colCount} columns`;
            } else {
                footer.textContent = `${visibleCount} of ${totalRows} rows × ${colCount} columns`;
            }
        }
    }

    // Keyboard navigation for tables
    function initKeyboardNav() {
        document.querySelectorAll('.rst-table table').forEach(table => {
            table.setAttribute('tabindex', '0');

            table.addEventListener('keydown', function(e) {
                const tbody = this.querySelector('tbody');
                const rows = Array.from(tbody.querySelectorAll('tr'));
                const currentRow = tbody.querySelector(':focus') ||
                                  tbody.querySelector('tr:hover') ||
                                  rows[0];

                if (!currentRow) return;

                const currentIndex = rows.indexOf(currentRow);
                let newRow = null;

                switch (e.key) {
                    case 'ArrowUp':
                        if (currentIndex > 0) newRow = rows[currentIndex - 1];
                        break;
                    case 'ArrowDown':
                        if (currentIndex < rows.length - 1) newRow = rows[currentIndex + 1];
                        break;
                    case 'Home':
                        newRow = rows[0];
                        break;
                    case 'End':
                        newRow = rows[rows.length - 1];
                        break;
                }

                if (newRow) {
                    e.preventDefault();
                    newRow.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
                    rows.forEach(r => r.style.background = '');
                    newRow.style.background = 'rgba(59, 130, 246, 0.1)';
                }
            });
        });
    }

    // Initialize keyboard navigation
    setTimeout(initKeyboardNav, 100);

})();
"#.to_string();

        Ok(js)
    }
}

impl Default for TableJsGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_js_generation() {
        let generator = TableJsGenerator::new();
        let js = generator.generate().unwrap();

        assert!(js.contains("initSearch"));
        assert!(js.contains("initSort"));
        assert!(js.contains("initCopy"));
        assert!(js.contains("initResponsiveScroll"));
        assert!(js.contains(".table-search"));
        assert!(js.contains(".table-copy"));
    }
}
