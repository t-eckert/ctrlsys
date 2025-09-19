<script lang="ts">
	import { Progress, useId } from "bits-ui"
	import { cn } from "$lib/utils"

	interface Props {
		value?: number
		max?: number
		label?: string
		showPercentage?: boolean
		animated?: boolean
		width?: string
		class?: string
	}

	let {
		value = 50,
		max = 100,
		label = "Progress",
		showPercentage = true,
		animated = false,
		width = "100%",
		class: className
	}: Props = $props()

	const labelId = useId()
	const percentage = Math.round((value / max) * 100)
</script>

<div class={cn("flex flex-col gap-2", className)} style="width: {width}">
	<div class="flex items-center justify-between text-sm font-medium">
		<span id={labelId}>{label}</span>
		{#if showPercentage}
			<span>{percentage}%</span>
		{/if}
	</div>
	<Progress.Root
		aria-labelledby={labelId}
		{value}
		{max}
		class="shadow-mini-inset relative h-[15px] w-full overflow-hidden rounded-full bg-emerald-500"
	>
		<div
			class="bg-foreground shadow-mini-inset h-full w-full flex-1 rounded-full {animated
				? 'transition-all duration-1000 ease-in-out'
				: ''}"
			style={`transform: translateX(-${100 - percentage}%)`}
		></div>
	</Progress.Root>
</div>
