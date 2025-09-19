<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Link from "./link.svelte"

	const { Story } = defineMeta({
		title: "Components/Link",
		component: Link,
		tags: ["autodocs"],
		argTypes: {
			href: { control: "text" },
			target: {
				control: { type: "select" },
				options: ["_blank", "_self", "_parent", "_top"]
			},
			rel: { control: "text" },
			download: { control: "text" },
			title: { control: "text" },
			ariaLabel: { control: "text" },
			tabindex: { control: "number" },
			onClick: { action: "clicked" }
		},
		args: {
			href: "#"
		},
		parameters: {
			docs: {
				description: {
					component: `
# Link Component

A styled link component that automatically handles external links with proper security attributes and accessibility features.

## Features

- **Automatic External Link Detection**: Automatically opens external links in new tabs
- **Security**: Adds \`rel="noopener noreferrer"\` to external links for security
- **Accessibility**: Full ARIA support and keyboard navigation
- **Styling**: Consistent underlined styling with hover and focus states
- **Dark Mode**: Automatic dark mode support
- **TypeScript**: Full TypeScript support for all anchor tag properties

## External Link Detection

The component automatically detects external links by checking if the href:
- Starts with \`http://\`
- Starts with \`https://\`
- Starts with \`//\`

For external links, it automatically:
- Sets \`target="_blank"\`
- Adds \`rel="noopener noreferrer"\` for security

## Basic Usage

\`\`\`svelte
<!-- Internal link -->
<Link href="/about">About Us</Link>

<!-- External link (opens in new tab automatically) -->
<Link href="https://example.com">External Site</Link>

<!-- Download link -->
<Link href="/file.pdf" download>Download PDF</Link>
\`\`\`

## Accessibility

The component supports all ARIA attributes and follows accessibility best practices:
- Proper focus management
- Screen reader support
- Keyboard navigation
					`
				}
			}
		}
	})
</script>

<Story name="Default">
	{#snippet children()}
		<Link href="#">Default Link</Link>
	{/snippet}
</Story>

<Story name="Internal Link">
	{#snippet children()}
		<Link href="/internal-page">Internal Page Link</Link>
	{/snippet}
</Story>

<Story name="External Link">
	{#snippet children()}
		<Link href="https://example.com">External Site (opens in new tab)</Link>
	{/snippet}
</Story>

<Story name="External Link with Protocol">
	{#snippet children()}
		<Link href="http://example.com">HTTP External Link</Link>
	{/snippet}
</Story>

<Story name="Protocol Relative Link">
	{#snippet children()}
		<Link href="//example.com">Protocol Relative Link</Link>
	{/snippet}
</Story>

<Story name="Download Link">
	{#snippet children()}
		<Link href="/sample.pdf" download="sample-document.pdf">Download PDF</Link>
	{/snippet}
</Story>

<Story name="Email Link">
	{#snippet children()}
		<Link href="mailto:contact@example.com">Send Email</Link>
	{/snippet}
</Story>

<Story name="Phone Link">
	{#snippet children()}
		<Link href="tel:+1234567890">Call +1 (234) 567-890</Link>
	{/snippet}
</Story>

<Story name="With Custom Target">
	{#snippet children()}
		<Link href="/page" target="_parent">Open in Parent Frame</Link>
	{/snippet}
</Story>

<Story name="With Custom Rel">
	{#snippet children()}
		<Link href="https://example.com" rel="nofollow">External Link (nofollow)</Link>
	{/snippet}
</Story>

<Story name="With Title">
	{#snippet children()}
		<Link href="/help" title="Get help and support">Help & Support</Link>
	{/snippet}
</Story>

<Story name="With ARIA Label">
	{#snippet children()}
		<Link href="/profile" ariaLabel="Go to your user profile page">Profile</Link>
	{/snippet}
</Story>

<Story name="Disabled-style Link">
	{#snippet children()}
		<Link href="#" tabindex={-1} ariaHidden={true} style="opacity: 0.5; pointer-events: none;"
			>Disabled Link</Link
		>
	{/snippet}
</Story>

<Story name="Long Link Text">
	{#snippet children()}
		<Link href="https://example.com/very/long/path/to/some/resource">
			This is a very long link text that demonstrates how the component handles longer content and
			text wrapping
		</Link>
	{/snippet}
</Story>

<Story name="Link in Text">
	{#snippet children()}
		<p>
			This is a paragraph with an <Link href="https://example.com">inline link</Link> that demonstrates
			how the component works within flowing text content.
		</p>
	{/snippet}
</Story>

<Story name="Multiple Links">
	{#snippet children()}
		<div class="space-y-2">
			<div><Link href="/home">Home</Link></div>
			<div><Link href="/about">About</Link></div>
			<div><Link href="https://github.com">GitHub</Link></div>
			<div><Link href="mailto:hello@example.com">Contact</Link></div>
		</div>
	{/snippet}
</Story>

<Story name="Anchor Link">
	{#snippet children()}
		<Link href="#section1">Jump to Section 1</Link>
	{/snippet}
</Story>

<Story
	name="With Click Handler"
	args={{
		href: "#",
		onClick: () => alert("Link clicked!")
	}}
>
	{#snippet children()}
		<Link href="#" onclick={() => alert("Link clicked!")}>Click me for alert</Link>
	{/snippet}
</Story>

<Story name="External Link Security Demo">
	{#snippet children()}
		<div class="space-y-4">
			<p class="text-sm text-gray-600">Inspect these links to see automatic security attributes:</p>
			<div class="space-y-2">
				<div>
					<Link href="https://google.com"
						>Google (auto: target="_blank" rel="noopener noreferrer")</Link
					>
				</div>
				<div><Link href="/internal">Internal Link (no additional attributes)</Link></div>
				<div>
					<Link href="//example.com"
						>Protocol Relative (auto: target="_blank" rel="noopener noreferrer")</Link
					>
				</div>
			</div>
		</div>
	{/snippet}
</Story>
