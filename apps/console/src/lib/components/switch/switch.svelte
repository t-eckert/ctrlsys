<script lang="ts">
	import { Switch, Label, useId, type WithoutChildrenOrChild } from "bits-ui"
	import { cva } from "cva"

	let {
		id = useId(),
		checked = $bindable(false),
		ref = $bindable(null),
		label,
		...restProps
	}: WithoutChildrenOrChild<Switch.RootProps> & {
		label: string
	} = $props()

	let rootStyle = cva([
		"w-11",
		"h-6",
		"px-0.5",
		"py-0.5",
		"rounded-full",
		"shadow-inner",
		"shadow-neutral-300",
		"bg-neutral-50",
		"dark:bg-neutral-950",
		"border",
		"border-neutral-50",
		"dark:border-neutral-700",
		"rounded-full",
		"inline-flex",
		"shrink-0",
		"cursor-pointer",
		"items-center",
		"transition-colors",
		"focus-visible:ring-2",
		"focus-visible:ring-offset-2",
		"disabled:cursor-not-allowed",
		"disabled:opacity-50",
		"data-[state=checked]:bg-sky-200",
		"data-[state=checked]:shadow-sky-200"
	])
	let thumbStyle = cva([
		"aspect-square",
		"h-full",
		"rounded-full",
		"data-[state=checked]:translate-x-4",
		"data-[state=unchecked]:translate-x-0",
		"transition-transform",
		"bg-gradient-to-t",
		"from-neutral-800/80",
		"via-neutral-600/80",
		"to-neutral-400/80",
		"shadow-inner",
		"shadow-neutral-300/80"
	])
</script>

<div class="flex flex-row items-center gap-3">
	<Switch.Root bind:checked bind:ref {id} class={rootStyle()} {...restProps}>
		<Switch.Thumb class={thumbStyle()} />
	</Switch.Root>
	<Label.Root for={id} class="text-sm font-medium">{label}</Label.Root>
</div>
