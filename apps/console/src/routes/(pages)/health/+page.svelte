<script lang="ts">
	import Card from "$lib/components/card/card.svelte"
	import Badge from "$lib/components/badge/badge.svelte"
	import Indicator from "$lib/components/indicator/indicator.svelte"
</script>

<svelte:head>
	<title>System Health - ctrlsys Console</title>
</svelte:head>

<div class="max-w-7xl mx-auto p-6">
	<header class="mb-8">
		<h1 class="text-3xl font-bold mb-2">System Health</h1>
		<p class="text-neutral-600 dark:text-neutral-400">
			Monitor the status and health of all ctrlsys services
		</p>
	</header>

	<div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3 mb-8">
		<Card>
			<div class="p-6">
				<div class="flex items-center justify-between mb-4">
					<h3 class="text-lg font-semibold">JobScheduler Service</h3>
					<Indicator variant="positive" />
				</div>
				<div class="space-y-2">
					<div class="flex justify-between text-sm">
						<span>Status:</span>
						<Badge tone="positive">Healthy</Badge>
					</div>
					<div class="flex justify-between text-sm">
						<span>Uptime:</span>
						<span>2d 14h 32m</span>
					</div>
					<div class="flex justify-between text-sm">
						<span>Jobs Processed:</span>
						<span>1,247</span>
					</div>
				</div>
			</div>
		</Card>

		<Card>
			<div class="p-6">
				<div class="flex items-center justify-between mb-4">
					<h3 class="text-lg font-semibold">Timer Service</h3>
					<Indicator variant="positive" />
				</div>
				<div class="space-y-2">
					<div class="flex justify-between text-sm">
						<span>Status:</span>
						<Badge tone="positive">Healthy</Badge>
					</div>
					<div class="flex justify-between text-sm">
						<span>Uptime:</span>
						<span>1d 8h 15m</span>
					</div>
					<div class="flex justify-between text-sm">
						<span>Active Timers:</span>
						<span>12</span>
					</div>
				</div>
			</div>
		</Card>

		<Card>
			<div class="p-6">
				<div class="flex items-center justify-between mb-4">
					<h3 class="text-lg font-semibold">Control Plane</h3>
					<Indicator variant="neutral" />
				</div>
				<div class="space-y-2">
					<div class="flex justify-between text-sm">
						<span>Status:</span>
						<Badge tone="neutral">Developing</Badge>
					</div>
					<div class="flex justify-between text-sm">
						<span>Version:</span>
						<span>v0.1.0</span>
					</div>
					<div class="flex justify-between text-sm">
						<span>API Calls:</span>
						<span>-</span>
					</div>
				</div>
			</div>
		</Card>
	</div>

	<Card>
		<div class="p-6">
			<h3 class="text-lg font-semibold mb-4">Cluster Resources</h3>
			<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
				<div class="text-center">
					<div class="text-2xl font-bold text-blue-600 dark:text-blue-400">4</div>
					<div class="text-sm text-neutral-600 dark:text-neutral-400">Nodes</div>
				</div>
				<div class="text-center">
					<div class="text-2xl font-bold text-green-600 dark:text-green-400">23</div>
					<div class="text-sm text-neutral-600 dark:text-neutral-400">Pods</div>
				</div>
				<div class="text-center">
					<div class="text-2xl font-bold text-purple-600 dark:text-purple-400">8</div>
					<div class="text-sm text-neutral-600 dark:text-neutral-400">Services</div>
				</div>
				<div class="text-center">
					<div class="text-2xl font-bold text-orange-600 dark:text-orange-400">3</div>
					<div class="text-sm text-neutral-600 dark:text-neutral-400">Namespaces</div>
				</div>
			</div>
		</div>
	</Card>
</div>