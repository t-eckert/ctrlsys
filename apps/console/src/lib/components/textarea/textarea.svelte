<script lang="ts">
	import { cva } from "cva"
	import { fly } from "svelte/transition"

	type Props = {
		label?: string
		name?: string
		id?: string
		initialRows?: number
		initialValue?: string
		characterLimit?: number
		placeholder?: string
		disabled?: boolean
		required?: boolean
		min?: number | string
		max?: number | string
		step?: number | string
		pattern?: string
		readonly?: boolean
		helpText?: string
	}

	const {
		label,
		name,
		id,
		initialRows = 3,
		initialValue,
		characterLimit,
		placeholder,
		disabled = false,
		required = false,
		readonly = false,
		helpText
	}: Props = $props()

	let value: string = $state(initialValue || "")
	let characterCount = $derived(value.length)
	let limitConsumed = $derived.by(() => {
		if (!characterLimit) return 0

		return characterCount / characterLimit
	})

	type LimitStatus = "free" | "cozy" | "near" | "reached"
	let limitStatus: LimitStatus = $derived.by(() => {
		if (limitConsumed < 0.7) {
			return "free"
		} else if (limitConsumed < 0.9) {
			return "cozy"
		} else if (limitConsumed < 1) {
			return "near"
		} else {
			return "reached"
		}
	})

	let textareaStyle = cva([
		"w-full",
		"rounded-md",
		"bg-white",
		"border",
		"leading-tight",
		"border-neutral-400",
		"px-2.5",
		"py-2",
		"text-neutral-900",
		"outline-1",
		"-outline-offset-1",
		"outline-neutral-300",
		"shadow-inner",
		"shadow-neutral-100",
		"placeholder:text-neutral-400",
		"focus:outline-2",
		"focus:-outline-offset-2",
		"focus:outline-sky-600",
		"focus:bg-white",
		"disabled:cursor-not-allowed",
		"disabled:bg-neutral-200",
		"disabled:text-neutral-600",
		"sm:text-sm/6",
		"dark:placeholder:text-neutral-300",
		"dark:bg-neutral-950",
		"dark:border-neutral-800",
		"dark:outline-neutral-700",
		"dark:focus:bg-black",
		"dark:shadow-black"
	])

	let limitPillStyle = cva(
		["absolute right-2 bottom-3 rounded-md px-1.5 text-xs font-medium transition-color"],
		{
			variants: {
				limitStatus: {
					free: "bg-neutral-200",
					cozy: "bg-neutral-200",
					near: "bg-rose-200",
					reached: "bg-rose-500 text-white"
				}
			}
		}
	)
</script>

<div class="w-full">
	<div class="flex flex-row items-baseline justify-between px-0.5">
		<label for={id} class="text-sm/6 font-medium text-neutral-900 dark:text-neutral-200"
			>{label ?? name ?? id ?? ""}</label
		>
		{#if !required}
			<span class="text-sm/6 text-neutral-500">Optional</span>
		{/if}
	</div>
	<div class="relative">
		<textarea
			rows={initialRows}
			class={textareaStyle()}
			{placeholder}
			bind:value
			{disabled}
			{readonly}
		></textarea>
		{#if characterLimit && limitStatus !== "free"}
			<div class={limitPillStyle({ limitStatus })} transition:fly={{ y: 10, duration: 200 }}>
				<span>
					{characterCount}
				</span>
				<span>/</span>
				<span>
					{characterLimit}
				</span>
			</div>
		{/if}
	</div>
	{#if helpText}
		<p class="px-0.5 text-xs text-neutral-500" id={`${id}-help-text`}>{helpText}</p>
	{/if}
</div>
