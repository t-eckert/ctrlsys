<script lang="ts">
	import type { Snippet } from "svelte"

	let { children }: { children?: Snippet } = $props()
</script>

<article class="flex w-full max-w-2xl flex-col items-start gap-2">
	{#if children}
		{@render children()}
	{/if}
</article>
