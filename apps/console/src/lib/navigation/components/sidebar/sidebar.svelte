<script lang="ts">
	import { page } from "$app/stores"
	import { getNavigationStore } from "../../navigation-store.svelte"
	import { House, Clock, Briefcase, Heart } from "phosphor-svelte"

	const navStore = getNavigationStore()

	const routes = [
		{ name: "Home", path: "/", icon: House },
		{ name: "Jobs", path: "/jobs", icon: Briefcase },
		{ name: "Timers", path: "/timers", icon: Clock },
		{ name: "Health", path: "/health", icon: Heart }
	]

	$effect(() => {
		navStore.setCurrentPath($page.url.pathname)
	})
</script>

<nav
	class="flex h-full w-60 flex-col border-r border-neutral-200 bg-neutral-50 dark:border-neutral-900 dark:bg-neutral-950"
>
	<div class="flex flex-col gap-1 p-2">
		{#each routes as route}
			<a
				href={route.path}
				class="flex items-center gap-3 rounded px-3 py-2 text-sm transition-colors
					{navStore.isActive(route.path)
					? 'bg-blue-100 text-blue-900 dark:bg-blue-900 dark:text-blue-100'
					: 'text-neutral-700 hover:bg-neutral-200 dark:text-neutral-300 dark:hover:bg-neutral-800'}"
			>
				<svelte:component
					this={route.icon}
					size={18}
					weight={navStore.isActive(route.path) ? "fill" : "regular"}
				/>
				<span class="font-medium">{route.name}</span>
			</a>
		{/each}
	</div>
</nav>

