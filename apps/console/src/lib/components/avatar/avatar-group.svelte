<script lang="ts">
	import { cva } from "cva"
	import Avatar from "./avatar.svelte"

	interface User {
		displayName: string
		src?: string
		id?: string
	}

	interface Props {
		users: User[]
		max?: number
		size?: "sm" | "md" | "lg"
		spacing?: "tight" | "normal" | "loose"
	}

	let { users, max = 4, size = "md", spacing = "normal" }: Props = $props()

	let visibleUsers = $derived(users.slice(0, max))
	let remainingCount = $derived(Math.max(0, users.length - max))

	const groupStyle = cva("flex items-center", {
		variants: {
			spacing: {
				tight: "-space-x-1",
				normal: "-space-x-2",
				loose: "-space-x-1"
			}
		},
		defaultVariants: {
			spacing: "normal"
		}
	})

	const overflowStyle = cva(
		[
			"flex items-center justify-center rounded-full font-medium text-xs",
			"bg-neutral-200 text-neutral-700 border-2 border-white",
			"dark:bg-neutral-700 dark:text-neutral-200 dark:border-neutral-900"
		],
		{
			variants: {
				size: {
					sm: "h-8 w-8",
					md: "h-12 w-12",
					lg: "h-16 w-16"
				}
			},
			defaultVariants: {
				size: "md"
			}
		}
	)
</script>

<div class={groupStyle({ spacing })} role="group" aria-label="{users.length} team members">
	{#each visibleUsers as user, index (user.id || user.displayName)}
		<div
			class="relative rounded-full border-2 border-white dark:border-neutral-900"
			style="z-index: {visibleUsers.length - index}"
		>
			<Avatar
				displayName={user.displayName}
				src={user.src}
				{size}
				ariaLabel="Avatar for {user.displayName}"
				tabindex={index === 0 ? 0 : -1}
			/>
		</div>
	{/each}

	{#if remainingCount > 0}
		<div
			class={overflowStyle({ size })}
			aria-label="{remainingCount} more team members"
			title="{remainingCount} more"
		>
			+{remainingCount}
		</div>
	{/if}
</div>
