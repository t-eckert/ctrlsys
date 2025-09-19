<script lang="ts">
	import Button from "./button.svelte"

	let {
		text,
		role,
		tone,
		shape,
		href,
		onclick,
		disabled,
		name,
		value,
		formaction,
		type,
		ariaLabel,
		ariaDescribedBy,
		ariaExpanded,
		ariaPressed
	}: {
		text: string
		role?: "primary" | "secondary" | "tertiary"
		tone?: "neutral" | "positive" | "negative"
		shape?: "rectangle" | "circle" | "square" | "rounded"
		href?: string
		onclick?: () => void
		disabled?: boolean
		name?: string
		value?: string | number | string[] | null | undefined
		formaction?: string
		type?: "button" | "submit" | "reset"
		ariaLabel?: string
		ariaDescribedBy?: string
		ariaExpanded?: boolean
		ariaPressed?: boolean
	} = $props()
</script>

<Button
	{role}
	{tone}
	{shape}
	{href}
	{onclick}
	{disabled}
	{name}
	{value}
	{formaction}
	{type}
	{ariaLabel}
	{ariaDescribedBy}
	{ariaExpanded}
	{ariaPressed}
>
	{#snippet children()}{text}{/snippet}
</Button>
