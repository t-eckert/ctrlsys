<script lang="ts">
	import { cva } from "cva"

	type Tone = "neutral" | "positive" | "negative"

	interface Props {
		title: string
		tone?: Tone
	}

	let { title, tone = "neutral" }: Props = $props()

	const bar = cva("w-full px-3 py-0.5 rounded-t border border-collapse", {
		variants: {
			tone: {
				neutral: "text-neutral-800 dark:text-zinc-200 border-b-stone-100",
				positive: "bg-emerald-200 border-b-emerald-300",
				negative:
					"bg-rose-100 text-rose-600 shadow-inner shadow-rose-50 dark:bg-rose-600 dark:text-rose-200 border-rose-100"
			}
		}
	})
</script>

<div class={bar({ tone })}>
	<span class="text-sm font-semibold">{title}</span>
</div>
