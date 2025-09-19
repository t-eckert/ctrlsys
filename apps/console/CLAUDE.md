# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Core Development

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run preview` - Preview production build
- `npm run check` - Type check with Svelte Kit
- `npm run lint` - Run linting (Prettier + ESLint)
- `npm run format` - Format code with Prettier

### Testing

- `npm run test:unit` - Run Vitest unit tests
- `npm run test:e2e` - Run Playwright end-to-end tests
- `npm run test` - Run all tests

### Database (Drizzle + LibSQL)

- `npm run db:push` - Push schema changes to database
- `npm run db:migrate` - Run database migrations
- `npm run db:studio` - Open Drizzle Studio

### Storybook

- `npm run storybook` - Start Storybook development server
- `npm run build-storybook` - Build Storybook for production

### Alternative: Task Runner

All npm commands can also be run using `task` (e.g., `task dev`, `task build`, `task lint`)

## Architecture Overview

### Component System

- **UI Components**: Located in `src/lib/components/` with extensive Bits UI integration
- **Elements**: Basic HTML elements with styling in `src/lib/elements/`
- **Stories**: Storybook stories in component directories and `src/stories/`
- **Templates**: Component/page templates in `templates/` directory (currently empty but referenced)

### Styling & Theming

- **Tailwind CSS 4** with dark mode support
- **Class Variance Authority (CVA)** for component variants (aliased as `cva`)
- **Theme System**: Light/dark theming with Svelte stores in `src/lib/theme/`
- **Fonts**: Inter (via Google Fonts), JetBrains Mono Variable, and Libre Caslon Text

### Backend Architecture

- **Database**: Drizzle ORM with LibSQL/SQLite for edge deployment
- **Auth**: User/session management with password hashing (Argon2)
- **Server Utils**: Audit logging, KV store, and S3-compatible uploader in `src/lib/server/`
- **Database Schema**: User and session tables defined in `src/lib/server/db/schema.ts`

### Key Technologies

- **SvelteKit 2** with Svelte 5
- **TypeScript** throughout
- **Bits UI** for accessible component primitives
- **Phosphor Icons** for iconography
- **Custom Toast System** for notifications
- **Vitest** for unit testing, **Playwright** for E2E
- **Custom Markdown Processing** (replaced MDSvex)

### Environment Requirements

- `DATABASE_URL` environment variable required for database operations

### File Structure Notes

- Components follow a directory-per-component pattern with optional `.stories.svelte` files
- Server-side code isolated in `src/lib/server/`
- Routes follow SvelteKit conventions in `src/routes/`
- Static assets in `static/` directory
