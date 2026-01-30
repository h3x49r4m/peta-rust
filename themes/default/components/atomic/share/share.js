(function() {
  'use strict';
  
  let shareComponent = null;
  let shareToggle = null;
  let shareDropdown = null;
  let shareOptions = null;
  let isDropdownOpen = false;
  
  // Initialize the share component
  function initShare() {
    shareComponent = document.querySelector('[data-component="share"]');
    
    if (!shareComponent) {
      console.warn('Share component not found');
      return;
    }
    
    shareToggle = shareComponent.querySelector('.share-toggle');
    shareDropdown = shareComponent.querySelector('.share-dropdown');
    shareOptions = shareComponent.querySelectorAll('.share-option');
    
    if (!shareToggle || !shareDropdown) {
      console.warn('Share toggle or dropdown not found');
      return;
    }
    
    // Add click event listener to toggle
    shareToggle.addEventListener('click', toggleDropdown);
    
    // Add keyboard support (Enter and Space keys)
    shareToggle.addEventListener('keydown', function(e) {
      if (e.key === 'Enter' || e.key === ' ') {
        e.preventDefault();
        toggleDropdown();
      }
      
      // Close dropdown on Escape
      if (e.key === 'Escape' && isDropdownOpen) {
        e.preventDefault();
        closeDropdown();
      }
    });
    
    // Add click event listeners to share options
    shareOptions.forEach(function(option) {
      option.addEventListener('click', handleShare);
      option.addEventListener('keydown', function(e) {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          handleShare.call(option);
        }
      });
    });
    
    // Close dropdown when clicking outside
    document.addEventListener('click', handleClickOutside);
    
    // Close dropdown on scroll
    window.addEventListener('scroll', closeDropdown);
  }
  
  // Toggle dropdown visibility
  function toggleDropdown() {
    if (isDropdownOpen) {
      closeDropdown();
    } else {
      openDropdown();
    }
  }
  
  // Open dropdown
  function openDropdown() {
    isDropdownOpen = true;
    shareToggle.classList.add('active');
    shareDropdown.classList.add('active');
    shareDropdown.setAttribute('aria-hidden', 'false');
    
    // Focus first share option for accessibility
    if (shareOptions.length > 0) {
      shareOptions[0].focus();
    }
  }
  
  // Close dropdown
  function closeDropdown() {
    isDropdownOpen = false;
    shareToggle.classList.remove('active');
    shareDropdown.classList.remove('active');
    shareDropdown.setAttribute('aria-hidden', 'true');
    
    // Return focus to toggle button
    shareToggle.focus();
  }
  
  // Handle click outside of dropdown
  function handleClickOutside(e) {
    if (isDropdownOpen && !shareComponent.contains(e.target)) {
      closeDropdown();
    }
  }
  
  // Handle share action
  function handleShare() {
    const platform = this.getAttribute('data-platform');
    const url = encodeURIComponent(window.location.href);
    const title = encodeURIComponent(document.title || 'Check this out');
    
    switch (platform) {
      case 'x':
        shareOnX(url, title);
        break;
      case 'linkedin':
        shareOnLinkedIn(url, title);
        break;
      case 'copy':
        copyLink();
        break;
      default:
        console.warn('Unknown share platform:', platform);
    }
    
    closeDropdown();
  }
  
  // Share on X
  function shareOnX(url, title) {
    const shareUrl = `https://twitter.com/intent/tweet?text=${title}&url=${url}`;
    openShareWindow(shareUrl, 550, 420);
  }
  
  // Share on LinkedIn
  function shareOnLinkedIn(url, title) {
    const shareUrl = `https://www.linkedin.com/sharing/share-offsite/?url=${url}`;
    openShareWindow(shareUrl, 550, 560);
  }
  
  // Copy link to clipboard
  function copyLink() {
    const link = window.location.href;
    
    if (navigator.clipboard && navigator.clipboard.writeText) {
      navigator.clipboard.writeText(link)
        .then(function() {
          showToast('Link copied to clipboard!');
        })
        .catch(function(err) {
          console.error('Failed to copy link:', err);
          fallbackCopyTextToClipboard(link);
        });
    } else {
      fallbackCopyTextToClipboard(link);
    }
  }
  
  // Fallback copy to clipboard for older browsers
  function fallbackCopyTextToClipboard(text) {
    const textArea = document.createElement('textarea');
    textArea.value = text;
    
    // Avoid scrolling to bottom
    textArea.style.top = '0';
    textArea.style.left = '0';
    textArea.style.position = 'fixed';
    textArea.style.opacity = '0';
    
    document.body.appendChild(textArea);
    textArea.focus();
    textArea.select();
    
    try {
      const successful = document.execCommand('copy');
      if (successful) {
        showToast('Link copied to clipboard!');
      } else {
        showToast('Failed to copy link', true);
      }
    } catch (err) {
      console.error('Fallback: Oops, unable to copy', err);
      showToast('Failed to copy link', true);
    }
    
    document.body.removeChild(textArea);
  }
  
  // Open share popup window
  function openShareWindow(url, width, height) {
    const left = (window.innerWidth - width) / 2;
    const top = (window.innerHeight - height) / 2;
    const specs = `width=${width},height=${height},left=${left},top=${top},resizable=yes,scrollbars=yes,status=yes`;
    
    window.open(url, 'Share', specs);
  }
  
  // Show toast notification
  function showToast(message, isError = false) {
    // Remove existing toast
    const existingToast = document.querySelector('.share-toast');
    if (existingToast) {
      existingToast.remove();
    }
    
    // Create toast element
    const toast = document.createElement('div');
    toast.className = 'share-toast' + (isError ? ' share-toast-error' : '');
    toast.textContent = message;
    toast.setAttribute('role', 'alert');
    toast.setAttribute('aria-live', 'polite');
    
    // Add styles
    Object.assign(toast.style, {
      position: 'fixed',
      bottom: '100px',
      right: '80px',
      padding: '12px 20px',
      background: isError ? '#ef4444' : '#10b981',
      color: 'white',
      borderRadius: '8px',
      fontSize: '14px',
      fontWeight: '500',
      boxShadow: '0 4px 12px rgba(0, 0, 0, 0.15)',
      zIndex: '2000',
      animation: 'shareToastFadeIn 0.3s ease, shareToastFadeOut 0.3s ease 2.7s',
      opacity: '0'
    });
    
    // Add animation keyframes
    if (!document.querySelector('#share-toast-styles')) {
      const style = document.createElement('style');
      style.id = 'share-toast-styles';
      style.textContent = `
        @keyframes shareToastFadeIn {
          from { opacity: 0; transform: translateY(10px); }
          to { opacity: 1; transform: translateY(0); }
        }
        @keyframes shareToastFadeOut {
          from { opacity: 1; transform: translateY(0); }
          to { opacity: 0; transform: translateY(10px); }
        }
      `;
      document.head.appendChild(style);
    }
    
    // Append to body
    document.body.appendChild(toast);
    
    // Remove toast after animation
    setTimeout(function() {
      if (toast.parentNode) {
        toast.remove();
      }
    }, 3000);
  }
  
  // Initialize when DOM is ready
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initShare);
  } else {
    initShare();
  }
  
  // Clean up on page unload
  window.addEventListener('beforeunload', function() {
    if (shareToggle) {
      shareToggle.removeEventListener('click', toggleDropdown);
    }
    shareOptions.forEach(function(option) {
      option.removeEventListener('click', handleShare);
    });
    document.removeEventListener('click', handleClickOutside);
    window.removeEventListener('scroll', closeDropdown);
  });
})();