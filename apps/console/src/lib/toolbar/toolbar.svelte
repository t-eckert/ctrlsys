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
	class="flex w-full items-center justify-between border-b border-neutral-100 bg-white px-2 py-1 dark:border-neutral-900 dark:bg-neutral-950"
>
	<span class="font-mono text-xs text-neutral-900 dark:text-neutral-100">CTRLSYS</span>

	<div class="flex items-center gap-4">
		<!-- Search Box -->
		<div class="relative flex items-center">
			<MagnifyingGlass
				size={12}
				class="absolute left-2 text-neutral-400 dark:text-neutral-500"
				weight="bold"
			/>
			<input
				type="text"
				placeholder="Search..."
				bind:value={store.searchQuery}
				class="h-6 w-64 rounded border border-neutral-300 bg-white pr-2 pl-7 text-xs text-neutral-900 placeholder-neutral-400 focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none dark:border-neutral-700 dark:bg-neutral-800 dark:text-neutral-100 dark:placeholder-neutral-500"
			/>
		</div>
	</div>

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

