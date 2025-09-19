<script lang="ts">
	import { cva } from "cva"

	type Color = "green" | "red" | "yellow" | "blue" | "purple" | "gray"

	const indicatorStyle = cva("aspect-square w-2 rounded-full shadow-2xl", {
		variants: {
			color: {
				green: "bg-emerald-500 shadow-emerald-300",
				red: "bg-rose-500 shadow-rose-300",
				yellow: "bg-yellow-500 shadow-yellow-300",
				blue: "bg-sky-500 shadow-sky-300",
				purple: "bg-purple-500 shadow-purple-300",
				gray: "bg-neutral-500 shadow-neutral-300"
			},
			pulse: {
				true: "animate-pulse",
				false: "animate-none"
			}
		},
		defaultVariants: {
			color: "gray"
		}
	})

	const {
		color,
		message,
		pulse = true
	}: { color?: Color; message?: string; pulse?: boolean } = $props()
</script>

<div class="flex flex-row items-center gap-1.5">
	<span class={indicatorStyle({ color, pulse })}></span>
	{#if message}
		<span class="text-sm font-medium select-none">{message}</span>
	{/if}
</div>
