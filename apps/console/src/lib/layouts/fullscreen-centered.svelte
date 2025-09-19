<script lang="ts">
	import type { Snippet } from "svelte"

	interface Props {
		children: Snippet
	}

	let { children }: Props = $props()
</script>

<div class="overflow-none flex h-screen w-screen items-center justify-center">
	{#if children}
		{@render children()}
	{/if}
</div>
