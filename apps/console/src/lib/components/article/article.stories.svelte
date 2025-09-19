<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Article from "./article.svelte"

	const { Story } = defineMeta({
		title: "Components/Article",
		component: Article,
		tags: ["autodocs"],
		parameters: {
			docs: {
				description: {
					component:
						"Semantic article container component for blog posts, documentation, and other standalone content sections."
				}
			}
		}
	})
</script>

<Story name="Basic Article">
	<Article>
		<h1 class="mb-4 text-3xl font-bold">Sample Article Title</h1>
		<p class="mb-6 text-gray-600">Published on January 15, 2025</p>
		<p class="mb-4">
			This is a sample article demonstrating the Article component. It provides semantic structure
			and consistent styling for content sections.
		</p>
		<p class="mb-4">
			Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut
			labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
			laboris nisi ut aliquip ex ea commodo consequat.
		</p>
		<h2 class="mb-3 text-xl font-semibold">Subsection</h2>
		<p>
			Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla
			pariatur. Excepteur sint occaecat cupidatat non proident.
		</p>
	</Article>
</Story>

<Story name="Article with Image">
	<Article>
		<h1 class="mb-4 text-3xl font-bold">Article with Featured Image</h1>
		<img
			src="https://images.unsplash.com/photo-1486312338219-ce68d2c6f44d?w=800"
			alt="Person working on laptop"
			class="mb-6 h-64 w-full rounded-lg object-cover"
		/>
		<p class="mb-4">
			This article includes a featured image to demonstrate how media content integrates within the
			article component structure.
		</p>
		<p>
			The Article component maintains proper semantic structure while allowing flexible content
			composition.
		</p>
	</Article>
</Story>

<Story name="Technical Documentation">
	<Article>
		<h1 class="mb-4 text-3xl font-bold">API Documentation</h1>
		<p class="mb-6">Complete guide to using our REST API endpoints.</p>

		<h2 class="mb-3 text-2xl font-semibold">Authentication</h2>
		<p class="mb-4">All API requests require authentication via Bearer token:</p>
		<pre class="mb-6 overflow-x-auto rounded bg-gray-100 p-4">
			<code>curl -H "Authorization: Bearer YOUR_TOKEN" https://api.example.com/data</code>
		</pre>

		<h2 class="mb-3 text-2xl font-semibold">Endpoints</h2>
		<ul class="mb-4 list-inside list-disc space-y-2">
			<li><code>GET /users</code> - Retrieve all users</li>
			<li><code>POST /users</code> - Create a new user</li>
			<li><code>PUT /users/:id</code> - Update existing user</li>
		</ul>
	</Article>
</Story>
