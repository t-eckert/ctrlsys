<script lang="ts">
	import type { Snippet } from "svelte"

	let { children }: { children: Snippet } = $props()
</script>

<div
	class="flex flex-row justify-end gap-1 rounded-b border border-neutral-100 px-1 py-2 dark:border-zinc-800"
>
	{@render children()}
</div>
