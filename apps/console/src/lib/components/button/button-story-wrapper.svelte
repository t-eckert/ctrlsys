<script lang="ts">
	import Button from "./button.svelte"

	interface Props {
		role?: "primary" | "secondary" | "tertiary"
		tone?: "neutral" | "positive" | "negative"
		shape?: "rectangle" | "circle"
		href?: string
		onclick?: () => void
		disabled?: boolean
		name?: string
		value?: string | number | string[] | null | undefined
		label?: string
	}

	const {
		role = "primary",
		tone = "neutral",
		shape = "rectangle",
		href,
		onclick = () => {},
		disabled = false,
		name,
		value,
		label = "Button"
	}: Props = $props()
</script>

<Button {role} {tone} {shape} {href} {onclick} {disabled} {name} {value}>
	{#snippet children()}<span class="text-sm font-medium">{label}</span>{/snippet}
</Button>
