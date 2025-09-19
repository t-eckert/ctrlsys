<script lang="ts">
	import { Table, Thead, Tr, Tbody, Th, Td } from "$lib/components/table"
	import Email from "$lib/components/email/email.svelte"
	import Button from "$lib/components/button/button.svelte"
	import Money from "$lib/components/money/money.svelte"
</script>

<Table>
	<Thead></Thead>
</Table>

<Table>
	<Thead>
		<Tr>
			<Th>Name</Th>
			<Th>Title</Th>
			<Th>Email</Th>
			<Th>Role</Th>
			<Th align="right">Amount Owed</Th>
			<Th></Th>
		</Tr>
	</Thead>
	<Tbody>
		<Tr>
			<Td>John Doe</Td>
			<Td>Software Engineer</Td>
			<Td><Email addr="john.doe@email.com" /></Td>
			<Td>Admin</Td>
			<Td align="right"><Money value={32} /></Td>
			<Td><Button href="#" role="tertiary">Pay</Button></Td>
		</Tr>
		<Tr>
			<Td>John Doe</Td>
			<Td>Software Engineer</Td>
			<Td><Email addr="john.doe@email.com" /></Td>
			<Td>Admin</Td>
			<Td align="right"><Money value={32} /></Td>
			<Td><Button href="#" role="tertiary">Pay</Button></Td>
		</Tr>
		<Tr>
			<Td>John Doe</Td>
			<Td>Software Engineer</Td>
			<Td><Email addr="john.doe@email.com" /></Td>
			<Td>Admin</Td>
			<Td align="right"><Money value={12} /></Td>
			<Td><Button href="#" role="tertiary">Pay</Button></Td>
		</Tr>
		<Tr>
			<Td>John Doe</Td>
			<Td>Software Engineer</Td>
			<Td><Email addr="john.doe@email.com" /></Td>
			<Td>Admin</Td>
			<Td align="right"><Money value={0} /></Td>
			<Td><Button href="#" role="tertiary">Pay</Button></Td>
		</Tr>
	</Tbody>
</Table>
