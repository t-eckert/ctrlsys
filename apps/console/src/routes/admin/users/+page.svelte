<script lang="ts">
	import type { PageData } from "./$types"
	import { H1, P } from "$lib/typography"
	import Card from "$lib/components/card/card.svelte"
	import Badge from "$lib/components/badge/badge.svelte"
	import { Table, Thead, Tbody, Tr, Th, Td } from "$lib/table"

	let { data }: { data: PageData } = $props()
</script>

<div class="mx-auto flex flex-col gap-3">
	<H1 text="Users" />
	<P>Manage user accounts</P>

	{#if data.users.length === 0}
		<Card class="p-6 text-center">
			<P class="text-gray-500">No users found.</P>
		</Card>
	{:else}
		<Card class="overflow-hidden">
			<Table>
				<Thead>
					<Tr>
						<Th>ID</Th>
						<Th>Username</Th>
						<Th>Age</Th>
						<Th>Status</Th>
					</Tr>
				</Thead>
				<Tbody>
					{#each data.users as user}
						<Tr>
							<Td>
								<span class="font-mono text-gray-600">{user.id}</span>
							</Td>
							<Td>
								<span class="font-medium text-gray-900">{user.username}</span>
							</Td>
							<Td>
								{user.age ?? "Not specified"}
							</Td>
							<Td>
								<Badge color="green" text="Active" />
							</Td>
						</Tr>
					{/each}
				</Tbody>
			</Table>
		</Card>

		<div class="text-center">
			<P class="text-sm text-gray-500">Total users: {data.users.length}</P>
		</div>
	{/if}
</div>
