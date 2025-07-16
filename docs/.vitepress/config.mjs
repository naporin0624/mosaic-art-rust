import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'Mosaic Art Generator',
  description: 'High-performance mosaic art generator written in Rust',

  // Site configuration
  base: '/mosaic-art-rust/',
  cleanUrls: true,
  ignoreDeadLinks: true,

  // Theme configuration
  themeConfig: {
    // Site branding
    logo: '/logo.png',
    siteTitle: 'Mosaic Art Generator',

    // Navigation
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Getting Started', link: '/getting-started/' },
      { text: 'Guide', link: '/guide/' },
      { text: 'GUI', link: '/gui/' },
      { text: 'API', link: '/api/' },
      { text: 'Gallery', link: '/gallery/' },
      {
        text: 'Links',
        items: [
          { text: 'GitHub', link: 'https://github.com/naporin0624/mosaic-art-rust' },
          { text: 'CI/CD', link: 'https://github.com/naporin0624/mosaic-art-rust/actions' },
          { text: 'Coverage', link: 'https://naporin0624.github.io/mosaic-art-rust/' },
        ],
      },
    ],

    // Sidebar navigation
    sidebar: {
      '/getting-started/': [
        {
          text: 'Getting Started',
          items: [
            { text: 'Installation', link: '/getting-started/installation' },
            { text: 'Quick Start', link: '/getting-started/quick-start' },
            { text: 'First Mosaic', link: '/getting-started/first-mosaic' },
            { text: 'Common Issues', link: '/getting-started/troubleshooting' },
          ],
        },
      ],

      '/guide/': [
        {
          text: 'User Guide',
          items: [{ text: 'Overview', link: '/guide/' }],
        },
      ],

      '/gui/': [
        {
          text: 'GUI Application',
          items: [
            { text: 'Overview', link: '/gui/index' },
            { text: 'Getting Started', link: '/gui/getting-started' },
            { text: 'Interface Guide', link: '/gui/interface-guide' },
            { text: 'Advanced Settings', link: '/gui/advanced-settings' },
            { text: 'Architecture', link: '/gui/architecture' },
            { text: 'Examples', link: '/gui/examples' },
            { text: 'Troubleshooting', link: '/gui/troubleshooting' },
          ],
        },
      ],

      '/api/': [
        {
          text: 'API Reference',
          items: [
            { text: 'Overview', link: '/api/' },
            { text: 'Core API', link: '/api/core' },
            { text: 'Modules', link: '/api/modules' },
          ],
        },
      ],

      '/gallery/': [
        {
          text: 'Gallery',
          items: [
            { text: 'Overview', link: '/gallery/' },
            { text: 'Examples', link: '/gallery/examples' },
            { text: 'Showcase', link: '/gallery/showcase' },
          ],
        },
      ],
    },

    // Social links
    socialLinks: [{ icon: 'github', link: 'https://github.com/naporin0624/mosaic-art-rust' }],

    // Footer
    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2024 mosaic-rust contributors',
    },

    // Edit link
    editLink: {
      pattern: 'https://github.com/naporin0624/mosaic-art-rust/edit/main/docs/:path',
      text: 'Edit this page on GitHub',
    },

    // Last updated
    lastUpdated: {
      text: 'Last updated',
      formatOptions: {
        dateStyle: 'short',
        timeStyle: 'short',
      },
    },

    // Search
    search: {
      provider: 'local',
      options: {
        translations: {
          button: {
            buttonText: 'Search Documentation',
            buttonAriaLabel: 'Search Documentation',
          },
          modal: {
            searchBox: {
              resetButtonTitle: 'Clear search',
              resetButtonAriaLabel: 'Clear search',
              cancelButtonText: 'Cancel',
              cancelButtonAriaLabel: 'Cancel',
            },
            startScreen: {
              recentSearchesTitle: 'Recent Searches',
              noRecentSearchesText: 'No recent searches',
              saveRecentSearchButtonTitle: 'Save to recent searches',
              removeRecentSearchButtonTitle: 'Remove from recent searches',
              favoriteSearchesTitle: 'Favorite Searches',
              removeFavoriteSearchButtonTitle: 'Remove from favorites',
            },
            errorScreen: {
              titleText: 'Unable to fetch results',
              helpText: 'You might want to check your network connection.',
            },
            footer: {
              selectText: 'to select',
              navigateText: 'to navigate',
              closeText: 'to close',
            },
            noResultsScreen: {
              noResultsText: 'No results for',
              suggestedQueryText: 'Try searching for',
              reportMissingResultsText: 'Believe this query should return results?',
              reportMissingResultsLinkText: 'Let us know.',
            },
          },
        },
      },
    },
  },

  // Markdown configuration
  markdown: {
    theme: {
      light: 'github-light',
      dark: 'github-dark',
    },
    lineNumbers: true,
    config: md => {
      // Add custom markdown plugins if needed
    },
  },

  // Head configuration
  head: [
    ['link', { rel: 'icon', href: '/favicon.ico' }],
    ['meta', { name: 'theme-color', content: '#3c8772' }],
    ['meta', { property: 'og:title', content: 'Mosaic Art Generator' }],
    [
      'meta',
      {
        property: 'og:description',
        content: 'High-performance mosaic art generator written in Rust',
      },
    ],
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:url', content: 'https://naporin0624.github.io/mosaic-art-rust/' }],
    [
      'meta',
      {
        property: 'og:image',
        content: 'https://naporin0624.github.io/mosaic-art-rust/og-image.png',
      },
    ],
    ['meta', { name: 'twitter:card', content: 'summary_large_image' }],
    ['meta', { name: 'twitter:title', content: 'Mosaic Art Generator' }],
    [
      'meta',
      {
        name: 'twitter:description',
        content: 'High-performance mosaic art generator written in Rust',
      },
    ],
    [
      'meta',
      {
        name: 'twitter:image',
        content: 'https://naporin0624.github.io/mosaic-art-rust/og-image.png',
      },
    ],
  ],

  // Build options
  vite: {
    build: {
      chunkSizeWarningLimit: 1000,
    },
  },
})
