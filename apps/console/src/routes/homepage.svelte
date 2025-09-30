<script lang="ts">
	import Button from "$lib/components/button/button.svelte"
	import Card from "$lib/components/card/card.svelte"
	import FullwidthCentered from "$lib/layouts/fullwidth-centered.svelte"
</script>

<FullwidthCentered>
	<div class="max-w-4xl mx-auto py-12">
		<header class="text-center mb-12">
			<h1 class="text-4xl font-bold mb-4">ctrlsys Console</h1>
			<p class="text-lg text-neutral-600 dark:text-neutral-400">
				Manage your control system platform from a single, unified interface
			</p>
		</header>

		<div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
			<Card>
				<div class="p-6">
					<h3 class="text-xl font-semibold mb-2">Job Scheduler</h3>
					<p class="text-neutral-600 dark:text-neutral-400 mb-4">
						Schedule and manage Kubernetes jobs across your cluster
					</p>
					<Button href="/jobs">Manage Jobs</Button>
				</div>
			</Card>

			<Card>
				<div class="p-6">
					<h3 class="text-xl font-semibold mb-2">Timer Service</h3>
					<p class="text-neutral-600 dark:text-neutral-400 mb-4">
						Create and monitor timer-based operations
					</p>
					<Button href="/timers">Manage Timers</Button>
				</div>
			</Card>

			<Card>
				<div class="p-6">
					<h3 class="text-xl font-semibold mb-2">System Health</h3>
					<p class="text-neutral-600 dark:text-neutral-400 mb-4">
						Monitor system status and service health
					</p>
					<Button href="/health">View Status</Button>
				</div>
			</Card>
		</div>
	</div>
</FullwidthCentered>
