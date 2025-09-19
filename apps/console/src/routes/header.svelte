<script lang="ts">
	import Home from "./home.svelte"
	import Navigation from "$lib/navigation/navigation.svelte"
	import ThemeToggle from "$lib/theme/theme-toggle.svelte"
</script>

<div class="mx-auto flex w-full max-w-7xl flex-row items-center justify-between px-4 py-2">
	<Home />

	<div class="flex flex-row items-center gap-2">
		<Navigation />
		<ThemeToggle />
	</div>
</div>
