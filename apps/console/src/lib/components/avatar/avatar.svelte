<script lang="ts">
	import { Avatar } from "bits-ui"
	import { cva } from "cva"
	import initials from "$lib/utils/initials"

	interface Props {
		displayName: string
		src?: string
		srcSet?: string
		sizes?: string
		size?: "sm" | "md" | "lg" | "display"
		delayMs?: number
		loading?: boolean
		ariaLabel?: string
		role?: "img" | "button"
		tabindex?: number
	}

	let {
		src,
		srcSet,
		sizes,
		displayName,
		size = "md",
		delayMs = 0,
		loading = false,
		ariaLabel,
		role = "img",
		tabindex = 0
	}: Props = $props()

	let alt = ariaLabel || `Avatar for ${displayName}`
	let loadingStatus = $state<"loading" | "loaded" | "error">("loading")

	const rootStyle = cva(
		"flex items-center justify-center transition-all overflow-hidden rounded-full focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2",
		{
			variants: {
				size: {
					sm: "h-8 w-8 text-sm",
					md: "h-12 w-12 text-[17px]",
					lg: "h-16 w-16 text-xl",
					display: "h-96 w-96 text-3xl"
				},
				role: {
					img: "",
					button: "cursor-pointer hover:scale-105 active:scale-95"
				}
			},
			defaultVariants: {
				size: "md",
				role: "img"
			}
		}
	)

	const fallbackStyle = cva([
		"flex h-full w-full items-center justify-center rounded-full font-medium text-neutral-800 dark:text-neutral-200",
		"bg-gradient-to-t from-neutral-200 to-neutral-100 shadow-inner shadow-neutral-200",
		"dark:from-neutral-800 dark:to-neutral-700 dark:shadow-inner dark:shadow-neutral-800"
	])

	const loadingStyle = cva([
		"flex h-full w-full items-center justify-center rounded-full",
		"bg-gradient-to-t from-neutral-200 to-neutral-100 animate-pulse",
		"dark:from-neutral-800 dark:to-neutral-700"
	])
</script>

<Avatar.Root
	class={rootStyle({ size, role })}
	{delayMs}
	bind:loadingStatus
	{role}
	{tabindex}
	aria-label={alt}
>
	{#if loading}
		<div class={loadingStyle()} aria-label="Loading avatar">
			<div class="h-1/3 w-1/3 rounded-full bg-neutral-300 dark:bg-neutral-600"></div>
		</div>
	{:else}
		<Avatar.Image
			{src}
			srcset={srcSet}
			{sizes}
			{alt}
			class="h-full w-full object-cover transition-opacity duration-300 {loadingStatus === 'loaded'
				? 'opacity-100'
				: 'opacity-0'}"
		/>
		<Avatar.Fallback class={fallbackStyle()}>
			{size == "display" ? displayName : initials(displayName)}
		</Avatar.Fallback>
	{/if}
</Avatar.Root>
