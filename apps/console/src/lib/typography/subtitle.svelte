<script lang="ts">
	import { type Snippet } from "svelte"
	import { cva } from "cva"

	type Props = {
		children: Snippet
		fontStyle?: "sans" | "serif" | "mono"
	}

	const { children, fontStyle = "sans" }: Props = $props()

	let subtitleStyle = cva(["text-neutral-800 sm:text-7xl dark:text-white"], {
		variants: {
			fontStyle: {
				sans: ["font-sans", "font-semibold", "tracking-tight", "leading-tighter"],
				serif: ["font-serif", "font-semibold", "tracking-normal", "leading-tight"],
				mono: ["font-mono", "font-medium", "tracking-tighter", "leading-tighter"]
			}
		},
		defaultVariants: {
			fontStyle: "sans"
		}
	})
</script>

<span class={subtitleStyle({ fontStyle })}>
	{@render children()}
</span>
