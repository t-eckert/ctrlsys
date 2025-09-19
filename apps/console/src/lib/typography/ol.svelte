<script lang="ts">
	import { type Snippet } from "svelte"

	type Props = {
		children: Snippet
		type?: "1" | "a" | "A" | "i" | "I"
		start?: number
		reversed?: boolean
	}

	const { type, start, reversed, children }: Props = $props()
</script>

<ol {type} {start} {reversed} class="">
	{@render children()}
</ol>
