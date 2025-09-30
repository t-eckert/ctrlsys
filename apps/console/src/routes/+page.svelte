<script lang="ts">
	import Button from "$lib/components/button/button.svelte"
	import Card from "$lib/components/card/card.svelte"
</script>

<div class="h-full p-8">
	<header class="mb-8">
		<h1 class="mb-2 text-3xl font-bold text-neutral-900 dark:text-neutral-100">Dashboard</h1>
		<p class="text-sm text-neutral-600 dark:text-neutral-400">
			Control system platform management console
		</p>
	</header>

	<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
		<Card>
			<div class="p-6">
				<h3 class="mb-2 text-xl font-semibold">Job Scheduler</h3>
				<p class="mb-4 text-sm text-neutral-600 dark:text-neutral-400">
					Schedule and manage Kubernetes jobs across your cluster
				</p>
				<Button href="/jobs">Manage Jobs</Button>
			</div>
		</Card>

		<Card>
			<div class="p-6">
				<h3 class="mb-2 text-xl font-semibold">Timer Service</h3>
				<p class="mb-4 text-sm text-neutral-600 dark:text-neutral-400">
					Create and monitor timer-based operations
				</p>
				<Button href="/timers">Manage Timers</Button>
			</div>
		</Card>

		<Card>
			<div class="p-6">
				<h3 class="mb-2 text-xl font-semibold">System Health</h3>
				<p class="mb-4 text-sm text-neutral-600 dark:text-neutral-400">
					Monitor system status and service health
				</p>
				<Button href="/health">View Status</Button>
			</div>
		</Card>
	</div>

	<!-- Quick Stats Section -->
	<div class="mt-8">
		<h2 class="mb-4 text-xl font-semibold text-neutral-900 dark:text-neutral-100">Quick Stats</h2>
		<div class="grid gap-4 md:grid-cols-4">
			<div
				class="rounded-lg border border-neutral-200 bg-neutral-50 p-4 dark:border-neutral-800 dark:bg-neutral-900"
			>
				<div class="text-xs text-neutral-600 dark:text-neutral-400">Active Jobs</div>
				<div class="mt-1 text-2xl font-bold text-neutral-900 dark:text-neutral-100">0</div>
			</div>
			<div
				class="rounded-lg border border-neutral-200 bg-neutral-50 p-4 dark:border-neutral-800 dark:bg-neutral-900"
			>
				<div class="text-xs text-neutral-600 dark:text-neutral-400">Running Timers</div>
				<div class="mt-1 text-2xl font-bold text-neutral-900 dark:text-neutral-100">0</div>
			</div>
			<div
				class="rounded-lg border border-neutral-200 bg-neutral-50 p-4 dark:border-neutral-800 dark:bg-neutral-900"
			>
				<div class="text-xs text-neutral-600 dark:text-neutral-400">System Status</div>
				<div class="mt-1 text-2xl font-bold text-green-600 dark:text-green-400">Healthy</div>
			</div>
			<div
				class="rounded-lg border border-neutral-200 bg-neutral-50 p-4 dark:border-neutral-800 dark:bg-neutral-900"
			>
				<div class="text-xs text-neutral-600 dark:text-neutral-400">Uptime</div>
				<div class="mt-1 text-2xl font-bold text-neutral-900 dark:text-neutral-100">99.9%</div>
			</div>
		</div>
	</div>
</div>