<script lang="ts">
	import { X } from "phosphor-svelte"
	import type { Snippet } from "svelte"
	import badgeStyle from "./badge-style"

	type Color =
		| "neutral"
		| "red"
		| "orange"
		| "yellow"
		| "green"
		| "blue"
		| "indigo"
		| "purple"
		| "pink"

	interface Props {
		text: string
		color?: Color
		maxLength?: number
		icon?: Snippet
		removable?: boolean
		onRemove?: () => void
		ariaLabel?: string
	}

	let {
		color,
		text,
		maxLength = 20,
		icon,
		removable = false,
		onRemove,
		ariaLabel
	}: Props = $props()

	let abridgedText = $derived(text.length > maxLength ? text.slice(0, maxLength) + "..." : text)
	let isTextTruncated = $derived(text.length > maxLength)
</script>

<span
	class={badgeStyle({ color })}
	title={isTextTruncated ? text : undefined}
	role="status"
	aria-label={ariaLabel || text}
>
	{#if icon}
		<span class="mr-1 flex items-center" aria-hidden="true">
			{@render icon()}
		</span>
	{/if}

	<span>{abridgedText}</span>

	{#if removable}
		<button
			type="button"
			class="ml-1 flex items-center justify-center rounded-full p-0.5 transition-colors hover:bg-black/10 focus:ring-2 focus:ring-current focus:ring-offset-1 focus:outline-none dark:hover:bg-white/10"
			onclick={onRemove}
			aria-label="Remove {text}"
		>
			<X class="h-3 w-3" />
		</button>
	{/if}
</span>
