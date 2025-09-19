<script lang="ts">
	import { cva } from "cva"
	import slug from "$lib/utils/slug"

	type Props = {
		text: string
		fontStyle?: "sans" | "serif" | "mono"
		id?: string
	}

	const { text, fontStyle = "sans", id }: Props = $props()

	const style = cva(["text-2xl text-neutral-900 dark:text-neutral-100"], {
		variants: {
			fontStyle: {
				sans: ["font-sans", "font-bold", "tracking-tight"],
				serif: ["font-serif", "font-bold", "tracking-tight"],
				mono: ["font-mono", "font-bold", "tracking-tighter"]
			}
		},
		defaultVariants: {
			fontStyle: "sans"
		}
	})

	const headingId = id || slug(text)
</script>

<h2 class={style({ fontStyle })} id={headingId}>
	{text}
</h2>
