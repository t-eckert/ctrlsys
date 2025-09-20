# ctrlsys Console

A modern web interface for managing the ctrlsys control system platform. Built with SvelteKit 2 and Svelte 5, this console provides a unified interface for managing jobs, timers, and monitoring system health across your Kubernetes infrastructure.

## Features

- **Job Management**: Schedule and monitor Kubernetes jobs through the JobScheduler service
- **Timer Operations**: Create and manage timer-based operations via the Timer service
- **System Monitoring**: Real-time health checks and status monitoring
- **Dark/Light Theming**: Automatic system theme detection with manual override
- **Modern UI**: Built with Tailwind CSS 4 and accessible component primitives
- **Authentication**: Secure user management with session-based auth

## Quick Start

### Prerequisites

- Node.js 18+
- ctrlsys backend services running (JobScheduler, Timer service)

### Development Setup

1. **Install Dependencies**
   ```bash
   npm install
   ```

2. **Environment Configuration**
   ```bash
   cp .env.example .env
   ```

   Configure the required environment variable:
   - `DATABASE_URL`: Path to your SQLite database (e.g., `file:local.db`)

3. **Database Setup**
   ```bash
   # Initialize database schema
   npm run db:push

   # Optional: Open database studio
   npm run db:studio
   ```

4. **Start Development Server**
   ```bash
   npm run dev
   ```

   Access the console at http://localhost:5173

### Production Deployment

1. **Build for Production**
   ```bash
   npm run build
   ```

2. **Preview Production Build**
   ```bash
   npm run preview
   ```

## Architecture

### Technology Stack

- **SvelteKit 2** - Full-stack web framework
- **Svelte 5** - Component framework with runes-based reactivity
- **TypeScript** - Type-safe development
- **Tailwind CSS 4** - Utility-first styling with dark mode
- **Bits UI** - Accessible component primitives
- **Drizzle ORM** - Type-safe database operations
- **LibSQL/SQLite** - Edge-compatible database

### Project Structure

```
src/
├── lib/
│   ├── components/     # Reusable UI components
│   ├── layouts/        # Page layout components
│   ├── theme/          # Theme management
│   ├── server/         # Server-side utilities
│   └── utils/          # Shared utilities
├── routes/
│   ├── (pages)/        # Main application pages
│   ├── auth/           # Authentication pages
│   └── admin/          # Admin interface
└── app.html            # HTML template
```

### Backend Integration

The console integrates with ctrlsys backend services:

- **JobScheduler Service**: ConnectRPC API for job management
- **Timer Service**: gRPC API for timer operations
- **Control Plane**: Central coordination service

## Development

### Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run preview` - Preview production build
- `npm run check` - Type checking and linting
- `npm run test` - Run all tests
- `npm run test:unit` - Unit tests with Vitest
- `npm run test:e2e` - E2E tests with Playwright
- `npm run storybook` - Component documentation

### Component Development

The console uses a comprehensive component library built on Bits UI:

- Fully accessible components following WAI-ARIA guidelines
- Consistent design system with Class Variance Authority (CVA)
- Dark/light theme support throughout
- TypeScript interfaces for all component props
- Storybook documentation for component usage

### Authentication

Built-in authentication system includes:

- User registration and login
- Secure session management
- Password hashing with Argon2
- Audit logging for security events
- Admin user management interface

## Contributing

When adding new features:

1. Follow existing component patterns and TypeScript conventions
2. Add Storybook stories for new components
3. Include proper accessibility attributes
4. Test in both light and dark themes
5. Update tests for new functionality

## License

This project is part of the ctrlsys platform - a hobby project for homelab control systems.