<script lang="ts">
	import { type Snippet } from "svelte"
	import { cn } from "$lib/utils"

	type Props = {
		children: Snippet
		class?: string
	}

	const { children, class: className, ...restProps }: Props = $props()
</script>

<p class={cn("w-full max-w-2xl", className)} {...restProps}>
	{@render children()}
</p>
