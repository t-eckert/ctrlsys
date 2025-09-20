<script lang="ts">
	import Card from "$lib/components/card/card.svelte"
	import Button from "$lib/components/button/button.svelte"
	import Badge from "$lib/components/badge/badge.svelte"
	import EmptyState from "$lib/components/empty-state/empty-state.svelte"
</script>

<svelte:head>
	<title>Timer Service - ctrlsys Console</title>
</svelte:head>

<div class="max-w-7xl mx-auto p-6">
	<header class="mb-8">
		<h1 class="text-3xl font-bold mb-2">Timer Service</h1>
		<p class="text-neutral-600 dark:text-neutral-400">
			Create and monitor timer-based operations
		</p>
	</header>

	<div class="flex justify-between items-center mb-6">
		<div class="flex gap-4">
			<Button>
				{@render children()}Create Timer
			</Button>
			<Button role="secondary">
				{@render children()}Refresh
			</Button>
		</div>
		<div class="flex gap-2">
			<Badge>Active: 2</Badge>
			<Badge tone="positive">Completed: 8</Badge>
			<Badge tone="negative">Expired: 0</Badge>
		</div>
	</div>

	<div class="grid gap-6">
		<EmptyState>
			<div class="text-center py-12">
				<h3 class="text-lg font-semibold mb-2">No timers found</h3>
				<p class="text-neutral-600 dark:text-neutral-400 mb-4">
					Create your first timer to get started
				</p>
				<Button>
					{@render children()}Create Timer
				</Button>
			</div>
		</EmptyState>
	</div>
</div>