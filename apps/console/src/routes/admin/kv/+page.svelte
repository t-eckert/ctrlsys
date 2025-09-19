<script lang="ts">
	import type { PageData, ActionData } from "./$types"
	import { H1, H2, P } from "$lib/typography"
	import Card from "$lib/components/card/card.svelte"
	import Button from "$lib/components/button/button.svelte"
	import Input from "$lib/components/input/input.svelte"
	import { enhance } from "$app/forms"

	let { data, form }: { data: PageData; form: ActionData } = $props()

	let selectedType = $state("string")
	let key = $state("")
	let value = $state("")

	function formatValue(value: any): string {
		if (typeof value === "string") {
			return value
		}
		return JSON.stringify(value, null, 2)
	}

	function getPlaceholder(type: string): string {
		switch (type) {
			case "string":
				return "Enter a string value"
			case "number":
				return "Enter a number (e.g., 42)"
			case "boolean":
				return "Enter true or false"
			case "json":
				return 'Enter JSON (e.g., {"name": "value"})'
			default:
				return "Enter value"
		}
	}
</script>

<div class="mx-auto flex flex-col gap-3">
	<H1 text="Key-Value Store" />
	<P>Manage application key-value pairs</P>

	<!-- Add Key-Value Form -->
	<Card>
		<H2 text="Add New Key-Value Pair" />

		{#if form?.error}
			<div class="mb-4 rounded border border-red-400 bg-red-100 p-3 text-red-700">
				{form.error}
			</div>
		{/if}

		{#if form?.success}
			<div class="mb-4 rounded border border-green-400 bg-green-100 p-3 text-green-700">
				Key-value pair added successfully!
			</div>
		{/if}

		<form method="POST" action="?/add" use:enhance>
			<div class="mb-4 grid grid-cols-1 gap-4 md:grid-cols-3">
				<div>
					<Input
						label="Key"
						name="key"
						id="key"
						bind:value={key}
						placeholder="Enter unique key"
						required
					/>
				</div>

				<div>
					<label for="type" class="mb-1 block text-sm font-medium text-gray-700">Type</label>
					<select
						name="type"
						id="type"
						bind:value={selectedType}
						class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none"
					>
						<option value="string">String</option>
						<option value="number">Number</option>
						<option value="boolean">Boolean</option>
						<option value="json">JSON</option>
					</select>
				</div>

				<div>
					<Input
						label="Value"
						name="value"
						id="value"
						bind:value
						placeholder={getPlaceholder(selectedType)}
						required
					/>
				</div>
			</div>

			<Button role="primary" type="submit">
				{#snippet children()}
					Add Key-Value Pair
				{/snippet}
			</Button>
		</form>
	</Card>

	{#if data.keyValuePairs.length === 0}
		<Card class="p-6 text-center">
			<P class="text-gray-500">No key-value pairs found.</P>
		</Card>
	{:else}
		<Card class="overflow-hidden">
			<div class="overflow-x-auto">
				<table class="w-full">
					<thead class="border-b bg-gray-50">
						<tr>
							<th
								class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase"
							>
								Key
							</th>
							<th
								class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase"
							>
								Value
							</th>
							<th
								class="px-6 py-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase"
							>
								Type
							</th>
						</tr>
					</thead>
					<tbody class="divide-y divide-gray-200">
						{#each data.keyValuePairs as { key, value }}
							<tr class="hover:bg-gray-50">
								<td class="px-6 py-4 whitespace-nowrap">
									<span class="text-sm font-medium text-gray-900">{key}</span>
								</td>
								<td class="px-6 py-4">
									<div class="text-sm text-gray-900">
										{#if typeof value === "string"}
											<span class="break-words">{value}</span>
										{:else}
											<pre class="overflow-x-auto rounded bg-gray-100 p-2 text-xs">{formatValue(
													value
												)}</pre>
										{/if}
									</div>
								</td>
								<td class="px-6 py-4 whitespace-nowrap">
									<span
										class="inline-flex rounded-full bg-blue-100 px-2 py-1 text-xs font-semibold text-blue-800"
									>
										{typeof value}
									</span>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</Card>
	{/if}
</div>
