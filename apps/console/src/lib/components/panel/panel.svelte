<script lang="ts">
	import type { Snippet } from "svelte"

	interface Props {
		actions: Snippet
		bar: Snippet
		body: Snippet
	}

	const { actions, bar, body }: Props = $props()
</script>

<div class="border-collapse rounded bg-white shadow-sm dark:bg-neutral-950">
	{@render bar()}
	{@render body()}
	{@render actions()}
</div>
