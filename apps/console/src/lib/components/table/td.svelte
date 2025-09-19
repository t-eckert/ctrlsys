<script lang="ts">
	import { cva } from "cva"
	import type { Snippet } from "svelte"

	type Alignment = "left" | "center" | "right" | "justify" | "start" | "end"

	interface Props {
		children?: Snippet
		align?: Alignment
	}

	let { children, align = "left" }: Props = $props()

	const tdStyle = cva("px-0.5 py-0.75 text-sm dark:text-neutral-200", {
		variants: {
			align: {
				left: "text-left",
				center: "text-center",
				right: "text-right",
				justify: "text-justify",
				start: "text-start",
				end: "text-end"
			}
		},
		defaultVariants: {
			align: "left"
		}
	})
</script>

<td class={tdStyle({ align })}>
	{#if children}
		{@render children()}
	{/if}
</td>
