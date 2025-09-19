<script lang="ts">
	import type { PageData } from "./$types"
	import { H1, P } from "$lib/typography"
	import Card from "$lib/components/card/card.svelte"
	import Badge from "$lib/components/badge/badge.svelte"
	import { Table, Thead, Tbody, Tr, Th, Td } from "$lib/table"

	let { data }: { data: PageData } = $props()

	function formatDate(date: Date | null): string {
		if (!date) return "Unknown"
		return new Intl.DateTimeFormat("en-US", {
			dateStyle: "short",
			timeStyle: "medium"
		}).format(new Date(date))
	}

	function getActionBadgeColor(action: string): "green" | "blue" | "yellow" | "red" | "neutral" {
		switch (action.toUpperCase()) {
			case "CREATE":
				return "green"
			case "UPDATE":
				return "blue"
			case "DELETE":
				return "red"
			case "LOGIN":
				return "green"
			case "LOGOUT":
				return "yellow"
			default:
				return "neutral"
		}
	}

	function formatDetails(details: string | null): string {
		if (!details) return ""
		try {
			const parsed = JSON.parse(details)
			return JSON.stringify(parsed, null, 2)
		} catch {
			return details
		}
	}
</script>

<div class="mx-auto flex flex-col gap-3">
	<H1 text="Audit Logs" />
	<P>System activity and security audit trail</P>

	{#if data.auditLogs.length === 0}
		<Card class="p-6 text-center">
			<P class="text-gray-500">No audit logs found.</P>
		</Card>
	{:else}
		<Card class="overflow-hidden">
			<Table>
				<Thead>
					<Tr>
						<Th>Timestamp</Th>
						<Th>User</Th>
						<Th>Action</Th>
						<Th>Resource</Th>
						<Th>Resource ID</Th>
						<Th>IP Address</Th>
						<Th>Details</Th>
					</Tr>
				</Thead>
				<Tbody>
					{#each data.auditLogs as log}
						<Tr>
							<Td>
								<span class="font-mono text-sm">{formatDate(log.createdAt)}</span>
							</Td>
							<Td>
								{#if log.userId}
									<span class="font-medium">{log.userId}</span>
								{:else}
									<span class="text-gray-500 italic">Anonymous</span>
								{/if}
							</Td>
							<Td>
								<Badge color={getActionBadgeColor(log.action)} text={log.action} />
							</Td>
							<Td>
								<span class="font-medium">{log.resource}</span>
							</Td>
							<Td>
								{#if log.resourceId}
									<span class="font-mono text-sm">{log.resourceId}</span>
								{:else}
									<span class="text-gray-400">-</span>
								{/if}
							</Td>
							<Td>
								{#if log.ipAddress}
									<span class="font-mono text-sm">{log.ipAddress}</span>
								{:else}
									<span class="text-gray-400">-</span>
								{/if}
							</Td>
							<Td>
								{#if log.details}
									<details class="cursor-pointer">
										<summary class="text-blue-600 hover:text-blue-800">View</summary>
										<pre
											class="mt-2 max-w-xs overflow-x-auto rounded bg-gray-100 p-2 text-xs">{formatDetails(
												log.details
											)}</pre>
									</details>
								{:else}
									<span class="text-gray-400">-</span>
								{/if}
							</Td>
						</Tr>
					{/each}
				</Tbody>
			</Table>
		</Card>

		<div class="text-center">
			<P class="text-sm text-gray-500">Showing last {data.auditLogs.length} audit entries</P>
		</div>
	{/if}
</div>
