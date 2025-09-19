<script lang="ts">
	import { type Snippet } from "svelte"
	import { cva } from "cva"

	type Alignment = "left" | "center" | "right" | "justify" | "start" | "end"

	interface Props {
		children?: Snippet
		align?: Alignment
	}

	let { children, align = "left" }: Props = $props()

	const thStyle = cva("px-0.5 py-0.5 text-sm font-medium text-neutral-800 dark:text-neutral-100", {
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

<th class={thStyle({ align })}>
	{#if children}
		{@render children()}
	{/if}
</th>
