<script lang="ts">
	import { alwaysValid, type Validator } from "./validators"
	import { cva } from "cva"

	type Props = {
		label?: string
		type?: string
		name?: string
		id?: string
		initialValue?: string | number
		placeholder?: string
		disabled?: boolean
		required?: boolean
		min?: number | string
		max?: number | string
		step?: number | string
		pattern?: string
		readonly?: boolean
		validator?: Validator
		helpText?: string
	}

	const {
		label,
		type = "text",
		name,
		id,
		initialValue,
		placeholder,
		disabled = false,
		required = false,
		min,
		max,
		step,
		pattern,
		readonly = false,
		validator = alwaysValid,
		helpText
	}: Props = $props()

	let value = $state(initialValue || "")
	let validity = $derived(validator(value))

	let inputStyle = cva([
		"w-full",
		"rounded-md",
		"bg-white",
		"border",
		"border-neutral-400",
		"px-2.5",
		"py-1",
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
	<input
		{type}
		{name}
		{id}
		bind:value
		{placeholder}
		{disabled}
		{required}
		{min}
		{max}
		{step}
		{pattern}
		{readonly}
		aria-describedby={`${helpText ? `${id}-help-text` : ""}${!validity[0] ? ` ${id}-error` : ""}`.trim() ||
			undefined}
		aria-invalid={!validity[0]}
		class={inputStyle()}
	/>
	{#if helpText}
		<p class="mt-1 px-0.5 text-xs text-neutral-500" id={`${id}-help-text`}>{helpText}</p>
	{/if}
	{#if !validity[0]}
		<p class="mt-1 px-0.5 text-xs text-red-500" id={`${id}-error`}>
			{validity[1] || "Input is not valid"}
		</p>
	{/if}
</div>
