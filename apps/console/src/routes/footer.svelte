<script lang="ts">
	import config from "$lib/config"
	import { version } from "$app/environment"
	import { H3, P } from "$lib/typography"
	import {
		GithubLogo,
		BookOpen,
		Code,
		Bug,
		Heart,
		Lightning,
		Palette,
		Database,
		Shield
	} from "phosphor-svelte"

	const currentYear = new Date().getFullYear()

	const links = {
		project: [
			{ name: "About", href: "/about", icon: BookOpen },
			{ name: "Documentation", href: "/demo/validators", icon: Code },
			{ name: "Components", href: "/storybook", icon: Palette, external: true },
			{
				name: "GitHub",
				href: "https://github.com/t-eckert/sveltekit-kickstart",
				icon: GithubLogo,
				external: true
			}
		],
		features: [
			{ name: "Feedback System", href: "/feedback", icon: Heart },
			{ name: "Admin Panel", href: "/admin", icon: Shield },
			{ name: "Database Studio", href: "#", icon: Database, action: "npm run db:studio" },
			{
				name: "Report Bug",
				href: "https://github.com/t-eckert/sveltekit-kickstart/issues",
				icon: Bug,
				external: true
			}
		],
		developer: [
			{ name: "SvelteKit", href: "https://kit.svelte.dev", external: true },
			{ name: "TypeScript", href: "https://www.typescriptlang.org", external: true },
			{ name: "Tailwind CSS", href: "https://tailwindcss.com", external: true },
			{ name: "Drizzle ORM", href: "https://orm.drizzle.team", external: true }
		]
	}
</script>

<footer class="border-t border-gray-200 bg-gray-50 dark:border-gray-800 dark:bg-gray-900">
	<div class="mx-auto max-w-7xl px-4 py-12">
		<!-- Main Footer Content -->
		<div class="grid grid-cols-1 gap-8 md:grid-cols-2 lg:grid-cols-4">
			<!-- Project Info -->
			<div class="space-y-4">
				<div class="flex items-center space-x-2">
					<Lightning size={24} class="text-blue-600 dark:text-blue-400" />
					<H3 text={config.title} />
				</div>
				<P class="text-gray-600 dark:text-gray-300">
					{#snippet children()}
						{config.description}
					{/snippet}
				</P>
				<div class="flex items-center space-x-2 text-sm text-gray-500 dark:text-gray-400">
					<span>Version {version}</span>
					<span>•</span>
					<span>Built with SvelteKit</span>
				</div>
			</div>

			<!-- Project Links -->
			<div class="space-y-4">
				<H3 text="Project" />
				<ul class="space-y-3">
					{#each links.project as link}
						<li>
							<a
								href={link.href}
								target={link.external ? "_blank" : undefined}
								rel={link.external ? "noopener noreferrer" : undefined}
								class="flex items-center space-x-2 text-sm text-gray-600 transition-colors hover:text-gray-900 dark:text-gray-300 dark:hover:text-gray-100"
							>
								<svelte:component this={link.icon} size={16} />
								<span>{link.name}</span>
								{#if link.external}
									<span class="text-xs text-gray-400">�</span>
								{/if}
							</a>
						</li>
					{/each}
				</ul>
			</div>

			<!-- Features -->
			<div class="space-y-4">
				<H3 text="Features" />
				<ul class="space-y-3">
					{#each links.features as link}
						<li>
							{#if link.action}
								<button
									class="flex items-center space-x-2 text-sm text-gray-600 transition-colors hover:text-gray-900 dark:text-gray-300 dark:hover:text-gray-100"
									title="Run: {link.action}"
								>
									<svelte:component this={link.icon} size={16} />
									<span>{link.name}</span>
									<span class="font-mono text-xs text-gray-400">{link.action}</span>
								</button>
							{:else}
								<a
									href={link.href}
									target={link.external ? "_blank" : undefined}
									rel={link.external ? "noopener noreferrer" : undefined}
									class="flex items-center space-x-2 text-sm text-gray-600 transition-colors hover:text-gray-900 dark:text-gray-300 dark:hover:text-gray-100"
								>
									<svelte:component this={link.icon} size={16} />
									<span>{link.name}</span>
									{#if link.external}
										<span class="text-xs text-gray-400">�</span>
									{/if}
								</a>
							{/if}
						</li>
					{/each}
				</ul>
			</div>

			<!-- Tech Stack -->
			<div class="space-y-4">
				<H3 text="Built With" />
				<ul class="space-y-3">
					{#each links.developer as link}
						<li>
							<a
								href={link.href}
								target="_blank"
								rel="noopener noreferrer"
								class="flex items-center justify-between text-sm text-gray-600 transition-colors hover:text-gray-900 dark:text-gray-300 dark:hover:text-gray-100"
							>
								<span>{link.name}</span>
								<span class="text-xs text-gray-400">�</span>
							</a>
						</li>
					{/each}
				</ul>
			</div>
		</div>

		<!-- Bottom Section -->
		<div class="mt-12 border-t border-gray-200 pt-8 dark:border-gray-700">
			<div class="flex flex-col items-center justify-between space-y-4 md:flex-row md:space-y-0">
				<!-- Copyright -->
				<div class="flex items-center space-x-4 text-sm text-gray-500 dark:text-gray-400">
					<span>� {currentYear} {config.title}</span>
					<span>"</span>
					<span>Made with SvelteKit & TypeScript</span>
				</div>

				<!-- Version & Build Info -->
				<div class="flex items-center space-x-4 text-xs text-gray-400 dark:text-gray-500">
					<div class="flex items-center space-x-2">
						<div class="h-2 w-2 animate-pulse rounded-full bg-green-500"></div>
						<span>v{version}</span>
					</div>
					<span>•</span>
					<span>Built {new Date().toLocaleDateString()}</span>
				</div>
			</div>

			<!-- Development Info -->
			<div class="mt-6 rounded-lg bg-gray-100 p-4 dark:bg-gray-800">
				<div class="flex items-start space-x-3">
					<Code size={20} class="mt-0.5 text-blue-600 dark:text-blue-400" />
					<div class="flex-1">
						<h4 class="mb-2 text-sm font-medium text-gray-900 dark:text-gray-100">
							Development Commands
						</h4>
						<div class="grid grid-cols-1 gap-2 text-xs md:grid-cols-2 lg:grid-cols-4">
							<div class="flex flex-col space-y-1">
								<code class="text-blue-600 dark:text-blue-400">npm run dev</code>
								<span class="text-gray-600 dark:text-gray-400">Start development</span>
							</div>
							<div class="flex flex-col space-y-1">
								<code class="text-blue-600 dark:text-blue-400">npm run build</code>
								<span class="text-gray-600 dark:text-gray-400">Build production</span>
							</div>
							<div class="flex flex-col space-y-1">
								<code class="text-blue-600 dark:text-blue-400">npm run storybook</code>
								<span class="text-gray-600 dark:text-gray-400">View components</span>
							</div>
							<div class="flex flex-col space-y-1">
								<code class="text-blue-600 dark:text-blue-400">npm run db:studio</code>
								<span class="text-gray-600 dark:text-gray-400">Database GUI</span>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</footer>
