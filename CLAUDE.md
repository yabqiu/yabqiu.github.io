# CLAUDE.md - Project Documentation

## Project Overview

**Yanbin Qiu's Technical Blog** (`yabqiu.github.io`)
A Hugo-based static blog website serving as a personal knowledge repository focused on software programming, development practices, and technology exploration.

- **Site URL**: https://blog.yanbin.dev/ (primary) | https://yabqiu.github.io/ (GitHub Pages)
- **Blog Focus**: Software development, programming languages (Java, Python, Rust), frameworks, distributed systems
- **Content Language**: Mixed Chinese and English
- **Platform**: Hugo Static Site Generator on GitHub Pages

## Architecture & Technology Stack

### Core Technologies
- **Static Site Generator**: Hugo 0.152.2 (extended mode required)
- **Theme**: hugo-clarity (Git submodule) - Clarity Design System-based
- **Styling**: SASS/SCSS compiled via Dart Sass
- **Markup**: Markdown with Goldmark parser
- **Hosting**: GitHub Pages with GitHub Actions CI/CD
- **Comments**: Giscus (GitHub Discussions-based)
- **Search**: Fuse.js client-side full-text search

### Project Structure

```
yabqiu.github.io/
├── content/                 # Hugo content directory
│   ├── post/               # Blog posts and page bundles
│   ├── about.md           # Author profile page
│   ├── guestbook.md       # Comments/guestbook
│   ├── search.md          # Search functionality
│   └── archives.md        # Post archives listing
│
├── config/                 # Hugo configuration
│   └── _default/          # Environment-specific configs
│       ├── hugo.toml      # Main Hugo settings
│       ├── params.toml    # Theme parameters & site settings
│       ├── languages.toml # Localization settings
│       ├── markup.toml    # Markdown rendering config
│       └── menus/
│           └── menu.en.toml # Navigation structure
│
├── layouts/               # Custom layout overrides
│   ├── partials/         # Template partials
│   ├── shortcodes/       # Custom Hugo shortcodes
│   └── _default/         # Page templates
│
├── themes/               # Hugo themes (git submodule)
│   └── hugo-clarity/     # Primary theme
│
├── static/               # Static assets served as-is
│   ├── images/          # Blog images and media
│   ├── icons/           # Favicons and touch icons
│   └── logos/           # Site branding assets
│
├── assets/               # Hugo asset pipeline
│   ├── sass/
│   │   └── _custom.sass # Custom styling overrides
│   └── js/
│       └── custom.js    # Custom JavaScript
│
├── wp2hugo/              # WordPress migration tools/exports
└── .github/workflows/    # CI/CD automation
    └── hugo.yml         # GitHub Actions deployment
```

## Key Features

### Blog Functionality
- **Full-text Search**: Client-side search powered by Fuse.js
- **Comments System**: Giscus integration using GitHub Discussions
- **Table of Contents**: Auto-generated, collapsible TOC for articles
- **Series Support**: Group related articles via taxonomy
- **Featured Posts**: Highlighted content on homepage sidebar
- **Responsive Design**: Mobile-optimized with configurable navigation

### Content Management
- **Post Categories**: Technology, Programming, Frameworks, etc.
- **Tagging System**: Granular content classification
- **Code Highlighting**: Syntax highlighting with line numbers and copy functionality
- **Image Support**: Static directory or page bundles with modern format support
- **SEO Optimization**: Meta tags, Open Graph, structured data (JSON-LD)

### Analytics & Social
- **Multiple Analytics**: Support for Google Analytics, Plausible, Matomo, Umami
- **Social Integration**: GitHub, Twitter/X, LinkedIn profiles
- **RSS Feed**: Automated feed generation
- **Social Sharing**: Per-post sharing buttons

## Configuration

### Key Configuration Files

| File | Purpose |
|------|---------|
| `config/_default/hugo.toml` | Main Hugo settings, taxonomies, pagination |
| `config/_default/params.toml` | Theme parameters, author info, analytics, comments |
| `config/_default/languages.toml` | Site language and title configuration |
| `config/_default/markup.toml` | Markdown rendering and syntax highlighting |
| `config/_default/menus/menu.en.toml` | Navigation menu structure |

### Critical Settings
```toml
# hugo.toml
baseURL = "https://blog.yanbin.dev/"
theme = "hugo-clarity"
defaultContentLanguage = "en"
paginate = 10
summaryLength = 70

# params.toml
enableSearch = true
giscus = true
giscusRepo = "yabqiu/yabqiu.github.io"
numberOfTagsShown = 14
codeMaxLines = 7
mobileNavigation = "left"
```

## Development Workflow

### Local Development
1. **Prerequisites**: Hugo 0.152.2+ (extended), Git
2. **Clone with submodules**: `git clone --recurse-submodules [repo-url]`
3. **Local server**: `hugo server -D --disableFastRender`
4. **Build**: `hugo --minify --environment production`

### Content Creation
1. **New post**: `hugo new post/my-new-post.md`
2. **Frontmatter template** (from `archetypes/post.md`):
```yaml
---
title: "Article Title"
date: {{ .Date }}
description: "SEO meta description"
categories: [Technology]
tags: [programming, development]
featured: false
draft: true
---
```

### Deployment Pipeline
- **Trigger**: Push to `main` branch or manual workflow dispatch
- **Process**: GitHub Actions → Hugo build → GitHub Pages deployment
- **URL**: Automatically deployed to https://blog.yanbin.dev/

## Content Migration

### WordPress Integration
- **Migration Tools**: Located in `wp2hugo/` directory
- **Export Data**: WordPress content converted to Hugo-compatible markdown
- **Configuration**: `.db_config.json` (gitignored for security)
- **Status**: 200+ historical posts successfully migrated

## Customization Points

### Theme Customization
- **Layout Overrides**: `layouts/` directory overrides theme templates
- **Custom Styling**: `assets/sass/_custom.sass` for SASS modifications
- **Custom JavaScript**: `assets/js/custom.js` for additional functionality
- **Hook System**: `layouts/partials/hooks/` for template extensions

### Available Shortcodes
- `{{< archive >}}`: Display archive listing
- `{{< bundle-image >}}`: Handle page bundle images
- `{{< bundle-resource >}}`: Manage page bundle resources

## Author Information

**Yanbin Qiu** - Senior Software Developer
- **Primary Languages**: Java, Python, Rust
- **Expertise**: Spring ecosystem, distributed systems, containerization (Docker), message queues (Kafka)
- **Focus Areas**: Backend development, system architecture, emerging technologies
- **Contact**: yabqiu@gmail.com
- **Social**: [@YanbinQiu](https://twitter.com/YanbinQiu)

## Site Philosophy

The blog serves as a long-term knowledge repository emphasizing:
- **Original Content**: Hand-written articles over AI-generated content
- **Deep Learning**: Comprehensive exploration rather than surface-level knowledge
- **Knowledge Sharing**: Community engagement through comments and discussions
- **Sustainable Platform**: Static site approach for long-term maintenance and preservation
- **Learning Through Writing**: Using blog writing as a learning and retention tool

## Maintenance & Support

### Regular Tasks
- **Content Updates**: Regular blog posts and technical articles
- **Theme Updates**: Periodic hugo-clarity theme updates via git submodule
- **Dependency Management**: Hugo and Dart Sass version maintenance
- **Performance Monitoring**: Site speed and search functionality verification

### Troubleshooting
- **Build Issues**: Check Hugo version compatibility and theme submodule status
- **Comment Problems**: Verify Giscus repository settings and discussion permissions
- **Search Malfunction**: Ensure search index generation and Fuse.js loading
- **Styling Issues**: Review custom SASS compilation and theme overrides

### Security Considerations
- **Static Site Benefits**: Minimal attack surface with no server-side processing
- **Third-party Integration**: Giscus comments rely on GitHub authentication
- **Content Validation**: All content version-controlled through Git
- **Domain Security**: HTTPS enforcement via GitHub Pages

## Future Considerations

### Potential Enhancements
- **Multilingual Support**: Enhanced i18n for Chinese/English content
- **Performance Optimization**: Image optimization and lazy loading improvements
- **Content Organization**: Enhanced taxonomy and series organization
- **Analytics Enhancement**: Privacy-focused analytics implementation
- **Mobile Experience**: Progressive Web App (PWA) features

### Migration Path
- **Domain Independence**: yanbin.blog as primary with yabqiu.github.io as fallback
- **Content Preservation**: Git-based version control ensures content longevity
- **Platform Flexibility**: Hugo's wide hosting compatibility provides migration options

---

**Last Updated**: December 2024
**Hugo Version**: 0.152.2
**Theme Version**: hugo-clarity (latest)
**Deployment**: GitHub Actions + GitHub Pages