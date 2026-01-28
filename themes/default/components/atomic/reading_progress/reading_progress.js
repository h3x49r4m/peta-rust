// Reading Progress Component JavaScript
document.addEventListener("DOMContentLoaded", function () {
  const readingProgressComponents = document.querySelectorAll(
    '[data-component="reading_progress"]'
  );

  readingProgressComponents.forEach(function (component) {
    initializeReadingProgress(component);
  });
});

// Reading Progress Indicator
function initializeReadingProgress(component) {
  const progressBar = component;
  if (!progressBar) return;

  // Find the main content body (.article-body, .book-content-wrapper, .project-body)
  // This is the actual scrollable content area
  const contentBody = document.querySelector('.article-body') ||
                      document.querySelector('.book-content-wrapper') ||
                      document.querySelector('.project-body');
  if (!contentBody) return;

  function updateProgress() {
    const scrollTop = window.pageYOffset || document.documentElement.scrollTop;
    const scrollHeight = contentBody.offsetHeight;
    const clientHeight = document.documentElement.clientHeight;
    const contentTop = contentBody.offsetTop;

    let progress = 0;
    if (scrollTop > contentTop) {
      const contentScrollTop = scrollTop - contentTop;
      const contentHeight = scrollHeight - clientHeight;
      progress = Math.min(
        100,
        Math.max(0, (contentScrollTop / contentHeight) * 100)
      );
    }

    progressBar.style.width = progress + "%";
  }

  window.addEventListener("scroll", updateProgress);
  updateProgress();
}
