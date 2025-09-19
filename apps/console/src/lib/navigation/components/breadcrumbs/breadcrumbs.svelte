<script lang="ts">
	import { cva } from "cva"
	import Button from "$lib/components/button/button.svelte"
	import { fly } from "svelte/transition"
	import { CaretDown } from "phosphor-svelte"

	interface Route {
		href: string
		label: string
	}

	interface Props {
		routes: Route[]
		ariaLabel?: string
		truncationLimit?: number
	}

	let { routes, ariaLabel = "breadcrumbs", truncationLimit = 3 }: Props = $props()

	let leadingRoutes: Route[] = $derived(
		routes.length < truncationLimit
			? routes.slice(0, routes.length - 1)
			: routes.slice(0, truncationLimit - 1)
	)
	let truncatedRoutes: Route[] = $derived(
		routes.length > truncationLimit ? routes.slice(leadingRoutes.length, routes.length - 1) : []
	)
	let lastRoute: Route = $derived(routes[routes.length - 1])

	let showTruncated: boolean = $state(false)

	let expandButtonStyle = cva("flex flex-row items-center transition-all", {
		variants: {
			showTruncated: {
				true: "-rotate-90",
				false: "rotate-0"
			}
		},
		defaultVariants: {
			showTruncated: false
		}
	})
</script>

<nav aria-label={ariaLabel}>
	<ul class="flex flex-row items-center gap-0.5 text-xs font-medium text-neutral-600">
		{#each leadingRoutes as route}
			<li>
				<Button href={route.href} role="tertiary">
					<span class="text-xs">{route.label}</span>
				</Button>
			</li>
			<span class="text-neutral-500">/</span>
		{/each}
		{#if truncatedRoutes.length}
			<li class="ml-1">
				<Button role="secondary" shape="square" onclick={() => (showTruncated = !showTruncated)}>
					<div class={expandButtonStyle({ showTruncated })}>
						<CaretDown class="size-3" />
					</div>
				</Button>
			</li>
			{#if !showTruncated}
				<span class="ml-1 text-neutral-500">/</span>
			{:else}
				{#each truncatedRoutes as route}
					<li>
						<Button href={route.href} role="tertiary">
							<span class="text-xs">{route.label}</span>
						</Button>
					</li>
					<span class="text-neutral-500">/</span>
				{/each}
			{/if}
		{/if}
		{#if lastRoute}
			<li>
				<Button href={lastRoute.href} role="tertiary">
					<span class="text-xs">{lastRoute.label}</span>
				</Button>
			</li>
		{/if}
	</ul>
</nav>
