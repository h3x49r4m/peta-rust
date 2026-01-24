// Theme Hook - Provides theme management and personalization
class useTheme {
  constructor() {
    this.currentTheme = 'light';
    this.availableThemes = ['light', 'dark', 'auto'];
    this.themeConfig = null;
    this.customProperties = new Map();
    this.isInitialized = false;
    this.init();
  }
  
  async init() {
    try {
      // Load theme configuration
      await this.loadThemeConfig();
      
      // Initialize theme from localStorage or system preference
      this.initializeTheme();
      
      // Setup system theme detection
      this.setupSystemThemeDetection();
      
      // Setup CSS custom properties
      this.setupCustomProperties();
      
      this.isInitialized = true;
      this.emit('themeInitialized', { theme: this.currentTheme });
    } catch (error) {
      console.error('Failed to initialize theme:', error);
    }
  }
  
  async loadThemeConfig() {
    try {
      const response = await fetch('/assets/js/theme.json');
      this.themeConfig = await response.json();
    } catch (error) {
      // Use default theme config if loading fails
      this.themeConfig = {
        themes: {
          light: {
            colors: {
              primary: '#3b82f6',
              secondary: '#64748b',
              success: '#10b981',
              warning: '#f59e0b',
              error: '#ef4444',
              background: '#ffffff',
              backgroundSecondary: '#f1f5f9',
              text: '#1e293b',
              textSecondary: '#64748b',
              border: '#e2e8f0'
            },
            fonts: {
              body: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
              heading: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
              monospace: '"SF Mono", Monaco, "Cascadia Code", "Roboto Mono", Consolas, monospace'
            },
            spacing: {
              xs: '0.25rem',
              sm: '0.5rem',
              md: '1rem',
              lg: '1.5rem',
              xl: '2rem'
            }
          },
          dark: {
            colors: {
              primary: '#60a5fa',
              secondary: '#94a3b8',
              success: '#34d399',
              warning: '#fbbf24',
              error: '#f87171',
              background: '#0f172a',
              backgroundSecondary: '#1e293b',
              text: '#f1f5f9',
              textSecondary: '#94a3b8',
              border: '#334155'
            },
            fonts: {
              body: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
              heading: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
              monospace: '"SF Mono", Monaco, "Cascadia Code", "Roboto Mono", Consolas, monospace'
            },
            spacing: {
              xs: '0.25rem',
              sm: '0.5rem',
              md: '1rem',
              lg: '1.5rem',
              xl: '2rem'
            }
          }
        }
      };
    }
  }
  
  initializeTheme() {
    // Check for saved theme preference
    const savedTheme = localStorage.getItem('theme');
    
    if (savedTheme && this.availableThemes.includes(savedTheme)) {
      this.currentTheme = savedTheme;
    } else if (savedTheme === 'auto') {
      this.currentTheme = this.getSystemTheme();
    } else {
      // Default to light theme
      this.currentTheme = 'light';
    }
    
    this.applyTheme(this.currentTheme);
  }
  
  setupSystemThemeDetection() {
    // Listen for system theme changes
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    mediaQuery.addEventListener('change', (e) => {
      if (this.currentTheme === 'auto' || localStorage.getItem('theme') === 'auto') {
        const systemTheme = e.matches ? 'dark' : 'light';
        this.applyTheme(systemTheme);
        this.emit('systemThemeChanged', { theme: systemTheme });
      }
    });
  }
  
  setupCustomProperties() {
    this.updateCustomProperties();
  }
  
  getSystemTheme() {
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
  }
  
  setTheme(theme) {
    if (!this.availableThemes.includes(theme)) {
      console.warn(`Theme "${theme}" is not available`);
      return;
    }
    
    const previousTheme = this.currentTheme;
    this.currentTheme = theme;
    
    // Save preference
    localStorage.setItem('theme', theme);
    
    // Apply theme
    if (theme === 'auto') {
      const systemTheme = this.getSystemTheme();
      this.applyTheme(systemTheme);
    } else {
      this.applyTheme(theme);
    }
    
    this.emit('themeChanged', { 
      theme: this.currentTheme, 
      previousTheme,
      isAuto: theme === 'auto'
    });
  }
  
  applyTheme(theme) {
    const themeData = this.themeConfig.themes[theme];
    if (!themeData) {
      console.warn(`Theme data for "${theme}" not found`);
      return;
    }
    
    // Apply theme class to body
    document.body.className = document.body.className.replace(/theme-\w+/g, '');
    document.body.classList.add(`theme-${theme}`);
    
    // Update CSS custom properties
    this.updateCustomProperties(theme);
    
    // Update meta theme-color
    this.updateMetaThemeColor(themeData.colors.primary);
  }
  
  updateCustomProperties(theme = null) {
    const activeTheme = theme || this.currentTheme;
    const themeData = this.themeConfig.themes[activeTheme];
    
    if (!themeData) return;
    
    const root = document.documentElement;
    
    // Update color properties
    if (themeData.colors) {
      Object.entries(themeData.colors).forEach(([key, value]) => {
        root.style.setProperty(`--color-${key}`, value);
      });
    }
    
    // Update font properties
    if (themeData.fonts) {
      Object.entries(themeData.fonts).forEach(([key, value]) => {
        root.style.setProperty(`--font-${key}`, value);
      });
    }
    
    // Update spacing properties
    if (themeData.spacing) {
      Object.entries(themeData.spacing).forEach(([key, value]) => {
        root.style.setProperty(`--spacing-${key}`, value);
      });
    }
  }
  
  updateMetaThemeColor(color) {
    let metaThemeColor = document.querySelector('meta[name="theme-color"]');
    if (!metaThemeColor) {
      metaThemeColor = document.createElement('meta');
      metaThemeColor.name = 'theme-color';
      document.head.appendChild(metaThemeColor);
    }
    metaThemeColor.content = color;
  }
  
  getCurrentTheme() {
    return this.currentTheme;
  }
  
  getAvailableThemes() {
    return [...this.availableThemes];
  }
  
  getThemeConfig(theme = null) {
    const activeTheme = theme || this.currentTheme;
    return this.themeConfig.themes[activeTheme] || null;
  }
  
  toggleTheme() {
    const themes = ['light', 'dark'];
    const currentIndex = themes.indexOf(this.currentTheme === 'auto' ? this.getSystemTheme() : this.currentTheme);
    const nextIndex = (currentIndex + 1) % themes.length;
    this.setTheme(themes[nextIndex]);
  }
  
  setCustomProperty(name, value) {
    this.customProperties.set(name, value);
    document.documentElement.style.setProperty(name, value);
    this.emit('customPropertyChanged', { name, value });
  }
  
  getCustomProperty(name) {
    return this.customProperties.get(name) || 
           getComputedStyle(document.documentElement).getPropertyValue(name);
  }
  
  removeCustomProperty(name) {
    this.customProperties.delete(name);
    document.documentElement.style.removeProperty(name);
    this.emit('customPropertyRemoved', { name });
  }
  
  // Color utilities
  getColor(name, shade = null) {
    const themeData = this.getThemeConfig();
    if (!themeData || !themeData.colors) return null;
    
    const colorName = shade ? `${name}-${shade}` : name;
    return themeData.colors[colorName] || null;
  }
  
  generateColorVariations(baseColor) {
    // Simple color variation generation
    // In a real implementation, you might use a color manipulation library
    return {
      50: this.lightenColor(baseColor, 0.9),
      100: this.lightenColor(baseColor, 0.8),
      200: this.lightenColor(baseColor, 0.6),
      300: this.lightenColor(baseColor, 0.4),
      400: this.lightenColor(baseColor, 0.2),
      500: baseColor,
      600: this.darkenColor(baseColor, 0.1),
      700: this.darkenColor(baseColor, 0.2),
      800: this.darkenColor(baseColor, 0.3),
      900: this.darkenColor(baseColor, 0.4)
    };
  }
  
  lightenColor(color, amount) {
    // Simple color lightening - in production, use a proper color library
    const num = parseInt(color.replace('#', ''), 16);
    const r = Math.min(255, Math.floor((num >> 16) + (255 - (num >> 16)) * amount));
    const g = Math.min(255, Math.floor(((num >> 8) & 0x00FF) + (255 - ((num >> 8) & 0x00FF)) * amount));
    const b = Math.min(255, Math.floor((num & 0x0000FF) + (255 - (num & 0x0000FF)) * amount));
    return `#${((r << 16) | (g << 8) | b).toString(16).padStart(6, '0')}`;
  }
  
  darkenColor(color, amount) {
    // Simple color darkening - in production, use a proper color library
    const num = parseInt(color.replace('#', ''), 16);
    const r = Math.max(0, Math.floor((num >> 16) * (1 - amount)));
    const g = Math.max(0, Math.floor(((num >> 8) & 0x00FF) * (1 - amount)));
    const b = Math.max(0, Math.floor((num & 0x0000FF) * (1 - amount)));
    return `#${((r << 16) | (g << 8) | b).toString(16).padStart(6, '0')}`;
  }
  
  // Event system
  emit(event, data) {
    const customEvent = new CustomEvent(`theme:${event}`, { detail: data });
    document.dispatchEvent(customEvent);
  }
  
  on(event, callback) {
    document.addEventListener(`theme:${event}`, callback);
  }
  
  off(event, callback) {
    document.removeEventListener(`theme:${event}`, callback);
  }
}

// Export for use in components
window.useTheme = useTheme;

// Auto-initialize theme hook
const themeHook = new useTheme();