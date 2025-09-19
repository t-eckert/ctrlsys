<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Card from "./card.svelte"
	import Button from "$lib/components/button/button.svelte"

	const { Story } = defineMeta({
		title: "Components/Card",
		component: Card,
		tags: ["autodocs"],
		argTypes: {
			class: { control: "text" }
		},
		parameters: {
			layout: "centered",
			docs: {
				description: {
					component: `
# Card Component

A flexible container component for grouping related content with consistent styling and responsive design.

## Features

- **Flexible Container**: Accepts any content through snippet children
- **Responsive Design**: Adapts to different screen sizes
- **Dark Mode**: Automatic dark mode support
- **Backdrop Blur**: Modern glass-morphism effect
- **Customizable**: Accepts custom CSS classes for styling variations

## Basic Usage

\`\`\`svelte
<Card>
  {#snippet children()}
    <h3>Card Title</h3>
    <p>Card content goes here.</p>
  {/snippet}
</Card>
\`\`\`

## Custom Styling

\`\`\`svelte
<Card class="max-w-md">
  {#snippet children()}
    <p>Card with custom width</p>
  {/snippet}
</Card>
\`\`\`
					`
				}
			}
		}
	})
</script>

<Story name="Default">
	{#snippet children()}
		<h3 class="mb-1 text-lg font-medium">Default Card</h3>
		<p class="max-w-xs text-sm text-neutral-600 dark:text-neutral-200">
			This is a basic card with default styling. It includes a subtle border, background, and
			shadow.
		</p>
	{/snippet}
</Story>

<Story name="Simple Content">
	{#snippet children()}
		<p class="text-sm">Simple card with minimal content.</p>
	{/snippet}
</Story>

<Story name="Rich Content">
	{#snippet children()}
		<div class="w-full max-w-sm space-y-4">
			<div>
				<h2 class="text-xl font-semibold text-neutral-900 dark:text-neutral-100">Feature Card</h2>
				<p class="mt-1 text-sm text-neutral-600 dark:text-neutral-400">
					Cards are versatile containers that can hold various types of content. They provide visual
					separation and hierarchy to your interface.
				</p>
			</div>
			<div class="flex justify-end gap-2">
				<Button>
					<span class="text-sm font-medium">Primary Action</span>
				</Button>
				<Button role="tertiary">
					<span class="text-sm font-medium">Secondary</span>
				</Button>
			</div>
		</div>
	{/snippet}
</Story>

<Story name="List Card">
	{#snippet children()}
		<div class="space-y-3">
			<h3 class="text-lg font-medium">Task List</h3>
			<ul class="space-y-2">
				<li class="flex items-center gap-2">
					<input type="checkbox" checked class="rounded" />
					<span class="text-sm text-neutral-500 line-through">Completed task</span>
				</li>
				<li class="flex items-center gap-2">
					<input type="checkbox" class="rounded" />
					<span class="text-sm">Pending task</span>
				</li>
				<li class="flex items-center gap-2">
					<input type="checkbox" class="rounded" />
					<span class="text-sm">Another pending task</span>
				</li>
			</ul>
		</div>
	{/snippet}
</Story>

<Story name="Image Card">
	{#snippet children()}
		<div class="space-y-3">
			<div class="h-32 w-full rounded bg-gradient-to-br from-blue-500 to-purple-600"></div>
			<div>
				<h3 class="text-lg font-medium">Image Placeholder</h3>
				<p class="text-sm text-neutral-600">
					Cards can contain images, graphics, or any visual content alongside text.
				</p>
			</div>
		</div>
	{/snippet}
</Story>

<Story name="Stats Card">
	{#snippet children()}
		<div class="space-y-2 text-center">
			<div class="text-3xl font-bold text-blue-600">1,234</div>
			<div class="text-sm font-medium text-neutral-700">Total Users</div>
			<div class="text-xs text-neutral-500">↑ 12% from last month</div>
		</div>
	{/snippet}
</Story>

