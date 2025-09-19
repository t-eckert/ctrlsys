<script lang="ts">
	import type { Snippet } from "svelte"

	let { children }: { children: Snippet } = $props()
</script>

<div class="w-full border-x border-neutral-100 px-3 py-3">{@render children()}</div>
