<script lang="ts">
	import { type Snippet } from "svelte"

	let { children }: { children?: Snippet } = $props()
</script>

<tr class="border-b border-b-neutral-300 dark:border-b-neutral-700">
	{#if children}
		{@render children()}
	{/if}
</tr>
