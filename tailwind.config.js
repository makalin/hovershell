/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // Tokyo Night colors
        'tokyo-bg': '#1a1b26',
        'tokyo-fg': '#a9b1d6',
        'tokyo-primary': '#7aa2f7',
        'tokyo-secondary': '#9ece6a',
        'tokyo-accent': '#ff9e64',
        'tokyo-success': '#9ece6a',
        'tokyo-warning': '#e0af68',
        'tokyo-error': '#f7768e',
        'tokyo-border': '#565f89',
        
        // Dracula colors
        'dracula-bg': '#282a36',
        'dracula-fg': '#f8f8f2',
        'dracula-primary': '#bd93f9',
        'dracula-secondary': '#50fa7b',
        'dracula-accent': '#ff79c6',
        'dracula-success': '#50fa7b',
        'dracula-warning': '#f1fa8c',
        'dracula-error': '#ff5555',
        'dracula-border': '#6272a4',
      },
      fontFamily: {
        'mono': ['JetBrainsMono Nerd Font', 'Monaco', 'Consolas', 'monospace'],
      },
      backdropBlur: {
        'xs': '2px',
      },
      animation: {
        'fade-in': 'fadeIn 0.3s ease-out',
        'fade-out': 'fadeOut 0.3s ease-out',
        'slide-in': 'slideIn 0.3s ease-out',
        'slide-out': 'slideOut 0.3s ease-out',
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        fadeOut: {
          '0%': { opacity: '1' },
          '100%': { opacity: '0' },
        },
        slideIn: {
          '0%': { transform: 'translateY(-100%)' },
          '100%': { transform: 'translateY(0)' },
        },
        slideOut: {
          '0%': { transform: 'translateY(0)' },
          '100%': { transform: 'translateY(-100%)' },
        },
      },
    },
  },
  plugins: [],
}