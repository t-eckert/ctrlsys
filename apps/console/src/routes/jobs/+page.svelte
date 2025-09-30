<script lang="ts">
	import Card from "$lib/components/card/card.svelte"
	import Button from "$lib/components/button/button.svelte"
	import Badge from "$lib/components/badge/badge.svelte"
	import EmptyState from "$lib/components/empty-state/empty-state.svelte"
</script>

<svelte:head>
	<title>Job Scheduler - ctrlsys Console</title>
</svelte:head>

<div class="max-w-7xl mx-auto p-6">
	<header class="mb-8">
		<h1 class="text-3xl font-bold mb-2">Job Scheduler</h1>
		<p class="text-neutral-600 dark:text-neutral-400">
			Manage and monitor Kubernetes jobs across your cluster
		</p>
	</header>

	<div class="flex justify-between items-center mb-6">
		<div class="flex gap-4">
			<Button>Schedule New Job</Button>
			<Button role="secondary">Refresh</Button>
		</div>
		<div class="flex gap-2">
			<Badge>Running: 3</Badge>
			<Badge tone="positive">Completed: 12</Badge>
			<Badge tone="negative">Failed: 1</Badge>
		</div>
	</div>

	<div class="grid gap-6">
		<EmptyState>
			<div class="text-center py-12">
				<h3 class="text-lg font-semibold mb-2">No jobs found</h3>
				<p class="text-neutral-600 dark:text-neutral-400 mb-4">
					Get started by scheduling your first job
				</p>
				<Button>Schedule Job</Button>
			</div>
		</EmptyState>
	</div>
</div>