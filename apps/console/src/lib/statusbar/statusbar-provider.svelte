<script lang="ts">
	import { initStatusBarStore } from "./statusbar-store.svelte"
	import type { Snippet } from "svelte"

	let { children }: { children: Snippet } = $props()

	initStatusBarStore()
</script>

{@render children()}