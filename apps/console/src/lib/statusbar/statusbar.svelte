<script lang="ts">
	import { getStatusBarStore } from "./statusbar-store.svelte"
	import { onMount } from "svelte"
	import { CheckCircle, WarningCircle, XCircle, Circle } from "phosphor-svelte"

	const store = getStatusBarStore()

	onMount(() => {
		const interval = setInterval(() => {
			store.updateServerTime()
		}, 1000)

		// Simulate setting system status (would come from API in real implementation)
		store.setSystemStatus("healthy")

		return () => clearInterval(interval)
	})

	const statusIcon = $derived(() => {
		switch (store.systemStatus) {
			case "healthy":
				return CheckCircle
			case "warning":
				return WarningCircle
			case "error":
				return XCircle
			default:
				return Circle
		}
	})

	const statusColor = $derived(() => {
		switch (store.systemStatus) {
			case "healthy":
				return "text-green-500"
			case "warning":
				return "text-yellow-500"
			case "error":
				return "text-red-500"
			default:
				return "text-neutral-400"
		}
	})
</script>

<div
	class="statusbar flex h-6 w-full items-center justify-between border-t border-neutral-200 bg-neutral-50 px-2 text-xs dark:border-neutral-900 dark:bg-neutral-950"
>
	<div class="flex flex-1 items-center gap-6"></div>

	<!-- Right Section: Server Time -->
	<div class="flex items-center gap-4 text-neutral-600 dark:text-neutral-400">
		<!-- Notifications -->
		{#if store.notifications.length > 0}
			<div class="text-neutral-600 dark:text-neutral-400">
				{store.notifications.length} notification{store.notifications.length > 1 ? "s" : ""}
			</div>
		{/if}

		<div class="h-full border-x border-neutral-200 dark:border-neutral-900">
			<span class="font-mono">{store.serverTime.toLocaleTimeString()}</span>
		</div>

		<!-- System Health Indicator -->
		{#if store.systemStatus}
			{@const StatusIcon = statusIcon()}
			<div class="flex items-center gap-2">
				<StatusIcon size={12} weight="fill" class={statusColor()} />
			</div>
		{/if}
	</div>
</div>

