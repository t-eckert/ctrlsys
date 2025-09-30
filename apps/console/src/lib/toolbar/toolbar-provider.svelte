<script lang="ts">
	import { initToolbarStore } from "./toolbar-store.svelte"
	import type { Snippet } from "svelte"

	let { children }: { children: Snippet } = $props()

	initToolbarStore()
</script>

{@render children()}