# SvelteKit Kick Start

This is a starter SvelteKit set up that includes all of the tooling and configuration that I like to use when building websites. It's very specifically tuned to my needs. If you would like to make something like it your own, fork this!

> This is a work in progress. I am still developing some of the items that are listed below.

## Quickstart

```shell
npx degit t-eckert/sveltekit-kickstart {project-name}
```

This will copy down the repository without the Git history.

## Environment Setup

### 1. Environment Variables

Copy the example environment file and configure it for your needs:

```shell
cp .env.example .env
```

The only required environment variable is:

- `DATABASE_URL`: Path to your SQLite database file (e.g., `file:local.db`)

### 2. Database Setup

Initialize your database with the schema:

```shell
# Install dependencies
npm install

# Generate and run database migrations
npm run db:push

# Optional: Open Drizzle Studio to view your database
npm run db:studio
```

### 3. Development

Start the development server:

```shell
npm run dev
```

### 4. Customize for Your Project

**Remove the Splash Component**: The home page (`/`) includes a splash component showcasing the starter kit. To build your own site:

1. Edit `src/routes/+page.svelte` to remove the `<Splash />` component
2. Optionally delete `src/lib/components/splash/` directory
3. Build your own home page content

### 5. Authentication Setup

The application includes a complete authentication system with:

- User registration at `/auth/register`
- User login at `/auth/login`
- Session management with secure cookies
- Password hashing using Argon2
- Audit logging for security events

Create your first user by visiting `/auth/register` and then log in at `/auth/login`.

### 6. Admin Features

Once logged in, you can access admin features at:

- `/admin` - Admin dashboard
- `/admin/users` - User management
- `/admin/feedback` - Feedback management and analytics
- `/admin/audit` - Audit log viewer
- `/admin/kv` - Key-value store management

### 7. User Feedback System

The starter includes a complete feedback system:

- **User Submission**: `/feedback` - Users can submit feedback with ratings and contact info
- **Admin Management**: `/admin/feedback` - View, filter, and manage feedback submissions
- **Analytics**: Feedback statistics including average ratings and status tracking
- **Audit Integration**: All feedback actions are logged for security

### Production Deployment

For production deployments:

1. Set `DATABASE_URL` to your production database (e.g., Turso LibSQL URL)
2. Configure additional environment variables as needed
3. Run the build command: `npm run build`
4. Use the production server: `npm run preview`

See `.env.example` for all available configuration options.

## Why

I have built several SvelteKit sites. I love the framework and I can work very quickly in it, but I found myself rewriting a lot of the same functionality with each new site. This starter kit allows me to solve all the repetitive and difficult problems once and build from there.

I've also formatted this project for AI-based development with Claude. This includes a CLAUDE.md file; tools for creating new API routes, componenents, pages, and stores; and descriptions of components to help Claude use them effectively.

## Features

### Pre-built Components

Using [Bits UI](https://www.bits-ui.com/) as a foundation, this project offers

- [Accordion](./src/lib/components/accordion/accordion.svelte)
- [Article]
- Avatar
- Avatar Group
- Badge
- Breadcrumb
- Button
- Card
- Carousel
- Checkbox
- Codeblock
- Dropdown Menu
- Email
- Empty State
- Error
- Filedrop
- Head
- Image
- Indicator
- Input
- Link Preview
- Menubar
- Meter
- Modal
- Money
- Navigation Menu
- Pagination
- Panel
- Popover
- Progress Bar
- Radio Group
- Range Calculator
- Relative Date
- Scroll Area
- Select
- Sidebar
- Slider
- Switch
- Tabs
- Time Field
- Time Range Field
- Toggle
- Toggle Group
- Toolbar
- Tooltip
- Window

### Tailwind CSS 4 and Class Variance Authority

Already configured for using Tailwind out of the box with `dark:` set up for dark theming. Includes the Prettier config to consistently order Tailwind classes. Uses Class Variance Authority to manage complex variants (e.g. different button styling)

### Taskfile

Common actions

### Phosphor Icons

An icon library using [Phosphor Icons](https://phosphoricons.com/).

### Testing

Configured unit testing with Vitest.

### Open Graph Config and OG Image Generator

### Bits UI

Many of the components in this starter kit make use of [Bits UI](https://www.bits-ui.com/). It is a spectacular system for developing more complex components.

### User and Admin pages

### Caddy

### Uploader to S3 Compatible Object Stores

### Libsql (SQLite) Configuration for Edge DBs using Drizzle

The kit comes pre-configured for using Drizzle with SQLite with the Libsql

### Key-Value Store

Using the Libsql edge DB, this key-value store provides a great way to s

### Audit Log

### Containerization

### A Considered Font Stack

### Templates

### Storybook Configuration

### Toast

### Light/Dark Theming

## Opinions

- I only use `+page.server.ts` because I often just want my page load code to run on the server anyway.

## Things to check out

- https://svelteflow.dev/
- https://github.com/beyonk-group/svelte-mapbox
- https://threlte.xyz/

## Helpful Links

- [Icons](https://phosphoricons.com/): Full, searchable listing of icons in the icons library.
