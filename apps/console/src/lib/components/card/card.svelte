<script lang="ts">
	import { type Snippet } from "svelte"
	import { cva } from "cva"

	interface Props {
		children: Snippet
	}

	let { children }: Props = $props()

	let style = cva([
		"rounded-lg",
		"border",
		"border-neutral-200",
		"bg-white/80",
		"p-2",
		"shadow-md",
		"backdrop-blur-2xl",
		"dark:bg-neutral-950/80",
		"dark:border-neutral-700",
		"dark:shadow-none"
	])
</script>

<section class={style()}>
	{@render children()}
</section>
