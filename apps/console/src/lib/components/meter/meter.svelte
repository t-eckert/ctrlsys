<script lang="ts">
	import { Meter, useId } from "bits-ui"

	type Props = {
		value?: number
		max?: number
		min?: number
		label?: string
		showValue?: boolean
	}

	const {
		value = 2000,
		max = 4000,
		min = 0,
		label = "Progress",
		showValue = true
	}: Props = $props()

	const labelId = useId()

	const usedPercentage = $derived((value / max) * 100)
	const percentageRemaining = $derived(100 - usedPercentage)

	const color = $derived.by(() => {
		if (percentageRemaining < 15) return "bg-red-500 dark:bg-red-400"
		if (percentageRemaining < 35) return "bg-orange-500 dark:bg-orange-400"
		if (percentageRemaining < 50) return "bg-yellow-500 dark:bg-yellow-400"
		return "bg-green-500 dark:bg-green-400"
	})
</script>

<div class="flex w-[60%] flex-col gap-2">
	<div class="flex items-center justify-between text-sm font-medium">
		<span id={labelId}>{label}</span>
		{#if showValue}
			<span>{value} / {max}</span>
		{/if}
	</div>
	<Meter.Root
		aria-labelledby={labelId}
		aria-valuetext="{value} out of {max}"
		{value}
		{min}
		{max}
		class="bg-dark-10 shadow-mini-inset relative h-[15px] overflow-hidden rounded-full"
	>
		<div
			class="shadow-mini-inset h-full w-full flex-1 rounded-full transition-all duration-1000 ease-in-out {color}"
			style="transform: translateX(-{100 - (100 * (value ?? 0)) / max}%)"
		></div>
	</Meter.Root>
</div>
