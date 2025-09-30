<script lang="ts">
	import { getToolbarStore } from "./toolbar-store.svelte"
	import { getThemeStore } from "$lib/theme/theme-store.svelte"
	import { MagnifyingGlass, Moon, Sun } from "phosphor-svelte"

	const store = getToolbarStore()
	const themeStore = getThemeStore()

	function toggleTheme() {
		const newTheme = themeStore.theme === "dark" ? "light" : "dark"
		themeStore.setTheme(newTheme)
	}
</script>

<div
	class="toolbar flex h-8 w-full items-center border-b border-neutral-200 bg-neutral-50 px-4 dark:border-neutral-800 dark:bg-neutral-900"
>
	<div class="flex flex-1 items-center gap-4">
		<span class="text-xs font-semibold text-neutral-900 dark:text-neutral-100">CTRLSYS</span>

		<!-- Search Box -->
		<div class="relative flex items-center">
			<MagnifyingGlass
				size={14}
				class="absolute left-2 text-neutral-400 dark:text-neutral-500"
				weight="bold"
			/>
			<input
				type="text"
				placeholder="Search..."
				bind:value={store.searchQuery}
				class="h-6 w-64 rounded border border-neutral-300 bg-white pl-7 pr-2 text-xs text-neutral-900 placeholder-neutral-400 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-neutral-700 dark:bg-neutral-800 dark:text-neutral-100 dark:placeholder-neutral-500"
			/>
		</div>
	</div>

	<!-- Right Section: Theme Toggle -->
	<div class="flex items-center gap-2">
		<button
			onclick={toggleTheme}
			class="flex h-6 w-6 items-center justify-center rounded hover:bg-neutral-200 dark:hover:bg-neutral-800"
			title="Toggle theme"
		>
			{#if themeStore.theme === "dark"}
				<Moon size={14} weight="fill" class="text-neutral-700 dark:text-neutral-300" />
			{:else}
				<Sun size={14} weight="fill" class="text-neutral-700 dark:text-neutral-300" />
			{/if}
		</button>
	</div>
</div>