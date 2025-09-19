<script lang="ts">
	import { type Snippet } from "svelte"

	let { children }: { children?: Snippet } = $props()
</script>

<thead class="border-b border-b-neutral-800">
	{#if children}
		{@render children()}
	{/if}
</thead>
