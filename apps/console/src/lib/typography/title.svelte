<script lang="ts">
	import { type Snippet } from "svelte"
	import { cva } from "cva"

	type Props = {
		children: Snippet
		fontStyle?: "sans" | "serif" | "mono"
	}

	const { children, fontStyle = "sans" }: Props = $props()

	const style = cva(["text-3xl", "sm:text-9xl", "text-neutral-800", "dark:text-neutral-100"], {
		variants: {
			fontStyle: {
				sans: ["font-sans", "font-bold", "tracking-tight", "sm:leading-28"],
				serif: ["font-serif", "font-bold", "tracking-normal"],
				mono: ["font-mono", "font-semibold", "tracking-tighter", "sm:leading-28"]
			}
		},
		defaultVariants: {
			fontStyle: "sans"
		}
	})
</script>

<span aria-label="subtitle" class={style({ fontStyle })}>
	{@render children()}
</span>
