<script lang="ts">
	import { H1, H2, H3, P } from "$lib/typography"
	import Card from "$lib/components/card/card.svelte"
	import Button from "$lib/components/button/button.svelte"
	import Badge from "$lib/components/badge/badge.svelte"
	import Avatar from "$lib/components/avatar/avatar.svelte"
	import Switch from "$lib/components/switch/switch.svelte"
	import ProgressBar from "$lib/components/progress-bar/progress-bar.svelte"
	import config from "$lib/config"
	import { CaretRight, Code, Database, Lock, Palette, TestTube, Gear, Globe } from "phosphor-svelte"

	const features = [
		{
			icon: Code,
			title: "40+ Pre-built Components",
			description:
				"Built on Bits UI with Tailwind CSS styling, covering everything from buttons to complex modals."
		},
		{
			icon: Lock,
			title: "Complete Authentication",
			description:
				"Secure user registration, login, session management with Argon2 password hashing."
		},
		{
			icon: Database,
			title: "Edge-Ready Database",
			description:
				"Drizzle ORM with LibSQL/SQLite, perfect for edge deployments and local development."
		},
		{
			icon: Palette,
			title: "Modern Styling",
			description:
				"Tailwind CSS 4 with dark mode support and Class Variance Authority for component variants."
		},
		{
			icon: TestTube,
			title: "Testing Setup",
			description:
				"Vitest for unit tests, Playwright for E2E testing, with comprehensive test utilities."
		},
		{
			icon: Gear,
			title: "Developer Experience",
			description: "TypeScript, ESLint, Prettier, Storybook, and AI-friendly development tools."
		},
		{
			icon: Globe,
			title: "Production Ready",
			description:
				"Containerization, audit logging, admin dashboard, and deployment configurations."
		}
	]

	const componentCategories = [
		{
			name: "Form & Input",
			components: [
				"Button",
				"Input",
				"Checkbox",
				"Radio Group",
				"Select",
				"Switch",
				"Slider",
				"Time Field"
			]
		},
		{
			name: "Layout & Navigation",
			components: ["Card", "Modal", "Sidebar", "Navigation Menu", "Breadcrumb", "Tabs", "Accordion"]
		},
		{
			name: "Data Display",
			components: ["Table", "Avatar", "Badge", "Progress Bar", "Meter", "Pagination", "Empty State"]
		},
		{
			name: "Feedback & Interaction",
			components: ["Toast", "Tooltip", "Popover", "Dropdown Menu", "Command Menu", "Error"]
		},
		{
			name: "Media & Content",
			components: ["Image", "Carousel", "Codeblock", "Article", "Typography", "Link Preview"]
		},
		{
			name: "Specialized",
			components: [
				"Filedrop",
				"Analytics",
				"Theme Toggle",
				"Range Calculator",
				"Money",
				"Relative Date"
			]
		}
	]

	const techStack = [
		{ name: "SvelteKit 2", description: "Modern full-stack framework with Svelte 5" },
		{ name: "TypeScript", description: "Type-safe development experience" },
		{ name: "Tailwind CSS 4", description: "Utility-first styling with dark mode" },
		{ name: "Drizzle ORM", description: "Type-safe database operations" },
		{ name: "Bits UI", description: "Accessible component primitives" },
		{ name: "Vitest", description: "Fast unit testing framework" },
		{ name: "Playwright", description: "Reliable end-to-end testing" },
		{ name: "Storybook", description: "Component development and documentation" }
	]
</script>

<div class="mx-auto max-w-6xl space-y-16 px-4 py-12">
	<!-- Hero Section -->
	<section class="text-center">
		<H1 text={config.title} />
		<P class="mt-4 text-lg text-gray-600 dark:text-gray-300">
			{#snippet children()}
				A comprehensive SvelteKit starter template designed for rapid development of modern web
				applications. Built with TypeScript, Tailwind CSS, and production-ready features out of the
				box.
			{/snippet}
		</P>

		<div class="mt-8 flex flex-wrap justify-center gap-4">
			<Button role="primary" href="/auth/register">
				{#snippet children()}
					Get Started
					<CaretRight size={16} class="ml-1" />
				{/snippet}
			</Button>
			<Button role="secondary" href="https://github.com/t-eckert/sveltekit-kickstart">
				{#snippet children()}
					View on GitHub
				{/snippet}
			</Button>
		</div>
	</section>

	<!-- Features Grid -->
	<section>
		<H2 text="Key Features" />
		<div class="mt-8 grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
			{#each features as feature}
				<Card class="p-6">
					{#snippet children()}
						<div class="flex items-start space-x-4">
							<div class="flex-shrink-0">
								<div class="rounded-lg bg-blue-100 p-3 dark:bg-blue-900">
									<svelte:component
										this={feature.icon}
										size={24}
										class="text-blue-600 dark:text-blue-400"
									/>
								</div>
							</div>
							<div>
								<H3 text={feature.title} />
								<P class="mt-2 text-sm text-gray-600 dark:text-gray-300">
									{#snippet children()}{feature.description}{/snippet}
								</P>
							</div>
						</div>
					{/snippet}
				</Card>
			{/each}
		</div>
	</section>

	<!-- Component Showcase -->
	<section>
		<H2 text="Component Library" />
		<P class="mt-2 text-gray-600 dark:text-gray-300">
			{#snippet children()}
				Over 40 pre-built components organized into logical categories, all built with accessibility
				and customization in mind.
			{/snippet}
		</P>

		<div class="mt-8 grid grid-cols-1 gap-8 lg:grid-cols-2">
			{#each componentCategories as category}
				<Card class="p-6">
					{#snippet children()}
						<H3 text={category.name} />
						<div class="mt-4 flex flex-wrap gap-2">
							{#each category.components as component}
								<Badge text={component} color="blue" />
							{/each}
						</div>
					{/snippet}
				</Card>
			{/each}
		</div>
	</section>

	<!-- Live Component Demo -->
	<section>
		<H2 text="Component Showcase" />
		<P class="mt-2 text-gray-600 dark:text-gray-300">
			{#snippet children()}
				See some of our components in action with live examples.
			{/snippet}
		</P>

		<div class="mt-8 grid grid-cols-1 gap-8 lg:grid-cols-2">
			<!-- Buttons Demo -->
			<Card class="p-6">
				{#snippet children()}
					<H3 text="Buttons" />
					<div class="mt-4 space-y-4">
						<div class="flex flex-wrap gap-2">
							<Button role="primary">
								{#snippet children()}Primary{/snippet}
							</Button>
							<Button role="secondary">
								{#snippet children()}Secondary{/snippet}
							</Button>
							<Button role="tertiary">
								{#snippet children()}Tertiary{/snippet}
							</Button>
						</div>
						<div class="flex flex-wrap gap-2">
							<Button role="primary" tone="positive">
								{#snippet children()}Success{/snippet}
							</Button>
							<Button role="primary" tone="negative">
								{#snippet children()}Danger{/snippet}
							</Button>
							<Button role="primary" disabled>
								{#snippet children()}Disabled{/snippet}
							</Button>
						</div>
					</div>
				{/snippet}
			</Card>

			<!-- Form Components Demo -->
			<Card class="p-6">
				{#snippet children()}
					<H3 text="Form Elements" />
					<div class="mt-4 space-y-4">
						<div class="flex items-center space-x-2">
							<Switch />
							<span class="text-sm">Dark mode toggle</span>
						</div>
						<div>
							<span class="text-sm text-gray-600 dark:text-gray-300">Progress example</span>
							<ProgressBar value={75} max={100} class="mt-2" />
						</div>
						<div class="flex items-center space-x-2">
							<Avatar displayName="John Doe" />
							<span class="text-sm">User avatar component</span>
						</div>
					</div>
				{/snippet}
			</Card>
		</div>
	</section>

	<!-- Technology Stack -->
	<section>
		<H2 text="Technology Stack" />
		<P class="mt-2 text-gray-600 dark:text-gray-300">
			{#snippet children()}
				Built with modern technologies and best practices for scalable web development.
			{/snippet}
		</P>

		<div class="mt-8 grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
			{#each techStack as tech}
				<Card class="p-4">
					{#snippet children()}
						<div class="text-center">
							<H3 text={tech.name} />
							<P class="mt-2 text-xs text-gray-600 dark:text-gray-300">
								{#snippet children()}{tech.description}{/snippet}
							</P>
						</div>
					{/snippet}
				</Card>
			{/each}
		</div>
	</section>

	<!-- Getting Started -->
	<section
		class="rounded-2xl bg-gradient-to-r from-blue-50 to-indigo-50 p-8 dark:from-blue-950 dark:to-indigo-950"
	>
		<div class="text-center">
			<H2 text="Ready to Get Started?" />
			<P class="mt-4 text-gray-600 dark:text-gray-300">
				{#snippet children()}
					Clone the repository and start building your next project with SvelteKit Kickstart.
				{/snippet}
			</P>

			<div class="mt-8">
				<Card class="mx-auto max-w-2xl p-6">
					{#snippet children()}
						<pre class="text-left"><code class="text-sm"
								>npx degit t-eckert/sveltekit-kickstart my-project
cd my-project
npm install
npm run dev</code
							></pre>
					{/snippet}
				</Card>
			</div>

			<div class="mt-6 flex flex-wrap justify-center gap-4">
				<Button role="primary" href="/auth/register">
					{#snippet children()}
						Try the Demo
					{/snippet}
				</Button>
				<Button role="secondary" href="/admin">
					{#snippet children()}
						Explore Admin
					{/snippet}
				</Button>
			</div>
		</div>
	</section>

	<!-- Footer Links -->
	<section class="border-t pt-8 text-center">
		<P class="text-gray-600 dark:text-gray-300">
			{#snippet children()}
				Built with ❤️ using SvelteKit.
				<a
					href="https://github.com/t-eckert/sveltekit-kickstart"
					class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300"
				>
					View source on GitHub
				</a>
			{/snippet}
		</P>
	</section>
</div>
