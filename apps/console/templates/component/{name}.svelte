<script lang="ts">
	import { type Snippet } from "svelte"
	import { cva } from "cva"

	interface Props {
		children?: Snippet
	}

	let { children }: Props = $props()

	let style = cva("")
</script>

<div class={style()}>
	{#if children}
		{@render children()}
	{/if}
</div>
