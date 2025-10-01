<script lang="ts">
	import { page } from "$app/state"
	import { House, Clock, Briefcase, Heart } from "phosphor-svelte"
	import type { Component } from "svelte"

	interface Route {
		name: string
		path: string
		icon: Component
	}

	const routes: Route[] = [
		{ name: "Home", path: "/", icon: House },
		{ name: "Jobs", path: "/jobs", icon: Briefcase },
		{ name: "Timers", path: "/timers", icon: Clock },
		{ name: "Health", path: "/health", icon: Heart }
	]

	let currentPath = $derived(page.url.pathname)
</script>

{#snippet RouteIcon(route: Route)}
	{@const Component = route.icon}
	<div class="transition-transform group-hover:rotate-12">
		<Component weight={route.path === currentPath ? "fill" : "regular"} />
	</div>
{/snippet}

<nav
	class="flex h-full w-60 flex-col border-r border-neutral-200 bg-neutral-50 dark:border-neutral-900 dark:bg-neutral-950"
>
	<div class="flex flex-col gap-1 px-2 py-1">
		{#each routes as route}
			<a
				href={route.path}
				class="group flex items-center gap-3 rounded-sm px-2 py-1 text-sm transition-colors
					{route.path === currentPath
					? 'bg-blue-100 text-blue-900 dark:bg-neutral-200 dark:text-neutral-900'
					: 'text-neutral-700 hover:bg-neutral-200 dark:text-neutral-300 dark:hover:bg-neutral-800'}"
			>
				{@render RouteIcon(route)}
				<span class="font-medium">{route.name}</span>
			</a>
		{/each}
	</div>
</nav>
