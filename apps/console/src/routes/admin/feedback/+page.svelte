<script lang="ts">
	import { enhance } from "$app/forms"
	import { goto } from "$app/navigation"
	import { H1, H2, P } from "$lib/typography"
	import Card from "$lib/components/card/card.svelte"
	import Button from "$lib/components/button/button.svelte"
	import Badge from "$lib/components/badge/badge.svelte"
	import { Table, Tbody, Td, Th, Thead, Tr } from "$lib/table"
	import type { PageData } from "./$types"
	import { ChatCircle, Star, User, Calendar, Funnel } from "phosphor-svelte"

	interface Props {
		data: PageData
	}

	let { data }: Props = $props()

	function getStatusColor(status: string): "neutral" | "blue" | "yellow" | "green" {
		switch (status) {
			case "open":
				return "blue"
			case "reviewing":
				return "yellow"
			case "closed":
				return "green"
			default:
				return "neutral"
		}
	}

	function formatDate(timestamp: Date): string {
		return new Date(timestamp).toLocaleDateString("en-US", {
			year: "numeric",
			month: "short",
			day: "numeric",
			hour: "2-digit",
			minute: "2-digit"
		})
	}

	function filterByStatus(status?: string) {
		const url = new URL(window.location.href)
		if (status) {
			url.searchParams.set("status", status)
		} else {
			url.searchParams.delete("status")
		}
		goto(url.toString())
	}
</script>

<div class="space-y-8">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div class="flex items-center space-x-3">
			<ChatCircle size={32} class="text-blue-600 dark:text-blue-400" />
			<H1 text="Feedback Management" />
		</div>
		<Button role="secondary" href="/feedback">
			{#snippet children()}
				Submit Feedback
			{/snippet}
		</Button>
	</div>

	<!-- Stats Cards -->
	<div class="grid grid-cols-1 gap-6 md:grid-cols-4">
		<Card class="p-6">
			{#snippet children()}
				<div class="text-center">
					<div class="text-2xl font-bold text-gray-900 dark:text-gray-100">{data.stats.total}</div>
					<div class="text-sm text-gray-600 dark:text-gray-400">Total Feedback</div>
				</div>
			{/snippet}
		</Card>
		<Card class="p-6">
			{#snippet children()}
				<div class="text-center">
					<div class="text-2xl font-bold text-blue-600 dark:text-blue-400">{data.stats.open}</div>
					<div class="text-sm text-gray-600 dark:text-gray-400">Open</div>
				</div>
			{/snippet}
		</Card>
		<Card class="p-6">
			{#snippet children()}
				<div class="text-center">
					<div class="text-2xl font-bold text-yellow-600 dark:text-yellow-400">
						{data.stats.reviewing}
					</div>
					<div class="text-sm text-gray-600 dark:text-gray-400">Reviewing</div>
				</div>
			{/snippet}
		</Card>
		<Card class="p-6">
			{#snippet children()}
				<div class="text-center">
					<div class="flex items-center justify-center space-x-1">
						<Star size={20} class="fill-current text-yellow-400" />
						<div class="text-2xl font-bold text-gray-900 dark:text-gray-100">
							{data.stats.averageRating || "N/A"}
						</div>
					</div>
					<div class="text-sm text-gray-600 dark:text-gray-400">Average Rating</div>
				</div>
			{/snippet}
		</Card>
	</div>

	<!-- Filter Buttons -->
	<div class="flex items-center space-x-2">
		<Funnel size={20} class="text-gray-600 dark:text-gray-400" />
		<Button role={!data.currentFilter ? "primary" : "tertiary"} onclick={() => filterByStatus()}>
			{#snippet children()}All{/snippet}
		</Button>
		<Button
			role={data.currentFilter === "open" ? "primary" : "tertiary"}
			onclick={() => filterByStatus("open")}
		>
			{#snippet children()}Open{/snippet}
		</Button>
		<Button
			role={data.currentFilter === "reviewing" ? "primary" : "tertiary"}
			onclick={() => filterByStatus("reviewing")}
		>
			{#snippet children()}Reviewing{/snippet}
		</Button>
		<Button
			role={data.currentFilter === "closed" ? "primary" : "tertiary"}
			onclick={() => filterByStatus("closed")}
		>
			{#snippet children()}Closed{/snippet}
		</Button>
	</div>

	<!-- Feedback Table -->
	{#if data.feedback.length === 0}
		<Card class="p-6 text-center">
			{#snippet children()}
				<ChatCircle size={48} class="mx-auto mb-4 text-gray-400" />
				<P class="text-gray-500">
					{#snippet children()}
						No feedback found{data.currentFilter ? ` with status "${data.currentFilter}"` : ""}.
					{/snippet}
				</P>
			{/snippet}
		</Card>
	{:else}
		<Card class="overflow-hidden">
			{#snippet children()}
				<div class="overflow-x-auto">
					<Table>
						{#snippet children()}
							<Thead>
								{#snippet children()}
									<Tr>
										{#snippet children()}
											<Th>Subject</Th>
											<Th>From</Th>
											<Th>Rating</Th>
											<Th>Status</Th>
											<Th>Date</Th>
											<Th>Actions</Th>
										{/snippet}
									</Tr>
								{/snippet}
							</Thead>
							<Tbody>
								{#snippet children()}
									{#each data.feedback as feedback}
										<Tr>
											{#snippet children()}
												<Td>
													<div>
														<div
															class="max-w-xs truncate font-medium text-gray-900 dark:text-gray-100"
														>
															{feedback.subject}
														</div>
														<div class="max-w-xs truncate text-sm text-gray-600 dark:text-gray-400">
															{feedback.message.substring(0, 100)}{feedback.message.length > 100
																? "..."
																: ""}
														</div>
													</div>
												</Td>
												<Td>
													<div class="flex items-center space-x-2">
														<User size={16} class="text-gray-400" />
														<div>
															<div class="text-sm font-medium text-gray-900 dark:text-gray-100">
																{feedback.name || "Anonymous"}
															</div>
															{#if feedback.email}
																<div class="text-xs text-gray-600 dark:text-gray-400">
																	{feedback.email}
																</div>
															{/if}
														</div>
													</div>
												</Td>
												<Td>
													{#if feedback.rating}
														<div class="flex items-center space-x-1">
															{#each Array(feedback.rating) as _}
																<Star size={14} class="fill-current text-yellow-400" />
															{/each}
															<span class="text-sm text-gray-600 dark:text-gray-400">
																{feedback.rating}
															</span>
														</div>
													{:else}
														<span class="text-gray-400">No rating</span>
													{/if}
												</Td>
												<Td>
													<Badge text={feedback.status} color={getStatusColor(feedback.status)} />
												</Td>
												<Td>
													<div
														class="flex items-center space-x-1 text-sm text-gray-600 dark:text-gray-400"
													>
														<Calendar size={14} />
														<span>{formatDate(feedback.createdAt)}</span>
													</div>
												</Td>
												<Td>
													<form method="post" action="?/updateStatus" use:enhance>
														<input type="hidden" name="feedbackId" value={feedback.id} />
														<select
															name="status"
															class="rounded border border-gray-300 bg-white px-2 py-1 text-xs dark:border-gray-600 dark:bg-gray-800"
															onchange={(e) => {
																const target = e.target as HTMLSelectElement
																target.form?.requestSubmit()
															}}
														>
															<option value="open" selected={feedback.status === "open"}
																>Open</option
															>
															<option value="reviewing" selected={feedback.status === "reviewing"}
																>Reviewing</option
															>
															<option value="closed" selected={feedback.status === "closed"}
																>Closed</option
															>
														</select>
													</form>
												</Td>
											{/snippet}
										</Tr>
									{/each}
								{/snippet}
							</Tbody>
						{/snippet}
					</Table>
				</div>
			{/snippet}
		</Card>
	{/if}
</div>
