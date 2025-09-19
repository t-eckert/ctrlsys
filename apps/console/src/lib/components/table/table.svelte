<script lang="ts">
	import { type Snippet } from "svelte"

	let { children }: { children?: Snippet } = $props()
</script>

<table class="w-full border-collapse">
	{#if children}
		{@render children()}
	{/if}
</table>
