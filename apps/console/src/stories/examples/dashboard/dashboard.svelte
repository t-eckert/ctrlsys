<script lang="ts">
	import { H1, H2, H3, P } from "$lib/typography"
	import {
		MagnifyingGlass,
		Bell,
		Users,
		Invoice,
		CreditCard,
		ChartBar,
		House,
		User,
		Headset,
		FolderOpen,
		Handshake,
		ChatCircle,
		Gear,
		SignOut,
		Plus,
		Trash,
		CaretDown,
		TrendUp,
		TrendDown
	} from "phosphor-svelte"
	import Card from "$lib/components/card/card.svelte"
	import Badge from "$lib/components/badge/badge.svelte"
	import Button from "$lib/components/button/button.svelte"
	import Input from "$lib/components/input/input.svelte"
	import Avatar from "$lib/components/avatar/avatar.svelte"
	import Money from "$lib/components/money/money.svelte"

	// Mock data for dashboard
	const statsData = [
		{
			icon: Users,
			title: "Total Customers",
			value: "212",
			change: "+9.6%",
			positive: true,
			color: "bg-orange-500"
		},
		{
			icon: Handshake,
			title: "Total Vendors",
			value: "82",
			change: "-5.5%",
			positive: false,
			color: "bg-blue-500"
		},
		{
			icon: Invoice,
			title: "Total Invoices",
			value: "132",
			change: "+8.5%",
			positive: true,
			color: "bg-red-500"
		},
		{
			icon: CreditCard,
			title: "Total Bills",
			value: "164",
			change: "-8.5%",
			positive: false,
			color: "bg-green-500"
		}
	]

	const accountBalanceData = [
		{ no: 1, bank: "Maybank", holder: "Cash", balance: 521611.2, status: "Active" },
		{ no: 2, bank: "Ocbc Bank", holder: "Carissa", balance: 160156.0, status: "Paused" },
		{ no: 3, bank: "RHB Bank", holder: "Renee", balance: 52229.0, status: "Canceled" },
		{ no: 4, bank: "Public Bank", holder: "Preston", balance: 82786.0, status: "Active" },
		{ no: 5, bank: "UOB Bank", holder: "Bowman", balance: 105150.0, status: "Paused" },
		{ no: 6, bank: "Ambank", holder: "Wendy", balance: 5484.4, status: "Canceled" }
	]

	const sidebarItems = [
		{ icon: House, label: "Dashboard", active: true },
		{ icon: User, label: "HRM", active: false },
		{ icon: Headset, label: "Support Ticket", active: false },
		{ icon: FolderOpen, label: "Project", active: false },
		{ icon: Handshake, label: "Manage Deals", active: false },
		{ icon: Users, label: "CRM", active: false },
		{ icon: ChatCircle, label: "Messenger", active: false }
	]

	const incomeExpenseData = [
		{ label: "Total Customers", value: "RM0.00", type: "customers" },
		{ label: "Expense Today", value: "RM23.00", type: "expense" },
		{ label: "Income This Mo...", value: "RM247.00", type: "income" },
		{ label: "Expense This...", value: "RM254.00", type: "expense" },
		{ label: "Total Customers", value: "RM365.00", type: "customers" },
		{ label: "Income This Mo...", value: "RM458.00", type: "income" }
	]

	const projectStatusData = [
		{ label: "On Going", percentage: 25, color: "stroke-purple-500" },
		{ label: "On Hold", percentage: 32, color: "stroke-green-500" },
		{ label: "Finished", percentage: 68, color: "stroke-orange-500" },
		{ label: "On Hold", percentage: 15, color: "stroke-red-500" }
	]

	function getStatusBadgeColor(status: string) {
		switch (status) {
			case "Active":
				return "green"
			case "Paused":
				return "yellow"
			case "Canceled":
				return "red"
			default:
				return "neutral"
		}
	}
</script>

<div class="flex h-screen bg-gray-50">
	<!-- Sidebar -->
	<aside class="flex w-64 flex-col border-r border-gray-200 bg-white">
		<!-- Logo -->
		<div class="border-b border-gray-200 p-6">
			<div class="flex items-center space-x-3">
				<div class="flex h-8 w-8 items-center justify-center rounded-lg bg-purple-600">
					<div class="h-4 w-4 rounded-sm bg-white"></div>
				</div>
				<span class="text-lg font-semibold text-gray-900">Cloud CRM</span>
			</div>
		</div>

		<!-- Navigation -->
		<nav class="flex-1 space-y-2 px-4 py-6">
			{#each sidebarItems as item}
				<a
					href="#"
					class="flex items-center space-x-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors {item.active
						? 'bg-purple-50 text-purple-700'
						: 'text-gray-600 hover:bg-gray-50 hover:text-gray-900'}"
				>
					<svelte:component this={item.icon} class="h-5 w-5" />
					<span>{item.label}</span>
				</a>
			{/each}
		</nav>

		<!-- Bottom actions -->
		<div class="space-y-2 border-t border-gray-200 p-4">
			<a
				href="#"
				class="flex items-center space-x-3 rounded-lg px-3 py-2 text-sm font-medium text-gray-600 hover:bg-gray-50 hover:text-gray-900"
			>
				<Gear class="h-5 w-5" />
				<span>Settings</span>
			</a>
			<a
				href="#"
				class="flex items-center space-x-3 rounded-lg px-3 py-2 text-sm font-medium text-gray-600 hover:bg-gray-50 hover:text-gray-900"
			>
				<SignOut class="h-5 w-5" />
				<span>Log out</span>
			</a>
		</div>
	</aside>

	<!-- Main Content -->
	<div class="flex flex-1 flex-col overflow-hidden">
		<!-- Top Header -->
		<header class="border-b border-gray-200 bg-white px-6 py-4">
			<div class="flex items-center justify-between">
				<div>
					<H1 text="Dashboard" />
					<span class="text-sm text-gray-500">/Accounting</span>
				</div>

				<div class="flex items-center space-x-4">
					<!-- Search -->
					<div class="relative">
						<MagnifyingGlass
							class="absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 transform text-gray-400"
						/>
						<input
							type="text"
							placeholder="Find something"
							class="rounded-lg border border-gray-300 py-2 pr-4 pl-10 text-sm focus:border-transparent focus:ring-2 focus:ring-purple-500 focus:outline-none"
						/>
					</div>

					<!-- Cloud CRM Selector -->
					<button
						class="flex items-center space-x-2 rounded-lg px-3 py-2 text-sm text-gray-700 hover:bg-gray-50"
					>
						<span>Cloud CRM</span>
						<CaretDown class="h-4 w-4" />
					</button>

					<!-- Notifications -->
					<button class="p-2 text-gray-400 hover:text-gray-600">
						<Bell class="h-5 w-5" />
					</button>

					<!-- User Profile -->
					<div class="flex items-center space-x-3">
						<Avatar displayName="Adam Smith" size="sm" />
						<div class="flex flex-col items-start">
							<span class="text-sm font-medium text-gray-900">@smith24</span>
							<span class="text-xs text-gray-500">Adam Smith</span>
						</div>
						<CaretDown class="h-4 w-4 text-gray-400" />
					</div>
				</div>
			</div>
		</header>

		<!-- Dashboard Content -->
		<main class="flex-1 overflow-auto p-6">
			<!-- Stats Cards -->
			<div class="mb-8 grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-4">
				{#each statsData as stat}
					<Card>
						<div class="flex items-center justify-between">
							<div class="flex items-center space-x-4">
								<div class="h-12 w-12 {stat.color} flex items-center justify-center rounded-full">
									<svelte:component this={stat.icon} class="h-6 w-6 text-white" />
								</div>
								<div>
									<P>{stat.title}</P>
									<P>{stat.value}</P>
								</div>
							</div>
							<div class="flex items-center space-x-1">
								{#if stat.positive}
									<TrendUp class="h-4 w-4 text-green-500" />
								{:else}
									<TrendDown class="h-4 w-4 text-red-500" />
								{/if}
								<span class="text-sm {stat.positive ? 'text-green-500' : 'text-red-500'}"
									>{stat.change}</span
								>
							</div>
						</div>
					</Card>
				{/each}
			</div>

			<div class="grid grid-cols-1 gap-8 lg:grid-cols-3">
				<!-- Cashflow Chart -->
				<div class="lg:col-span-2">
					<Card>
						<div class="mb-6 flex items-center justify-between">
							<H2 text="Cashflow" />
							<div class="flex space-x-2">
								<button class="rounded px-3 py-1 text-sm text-gray-500 hover:bg-gray-100">1D</button
								>
								<button class="rounded px-3 py-1 text-sm text-gray-500 hover:bg-gray-100">7D</button
								>
								<button class="rounded bg-purple-100 px-3 py-1 text-sm text-purple-700">1M</button>
								<button class="rounded px-3 py-1 text-sm text-gray-500 hover:bg-gray-100">3M</button
								>
								<button class="rounded px-3 py-1 text-sm text-gray-500 hover:bg-gray-100"
									>All</button
								>
							</div>
						</div>

						<!-- Mock Chart Area -->
						<div class="flex h-64 items-end justify-around rounded-lg bg-gray-50 p-4">
							{#each Array(7) as _, i}
								<div class="flex flex-col items-center space-y-2">
									<div class="flex flex-col items-center space-y-1">
										<div
											class="w-8 rounded-t bg-green-400"
											style="height: {Math.random() * 80 + 20}px"
										></div>
										<div
											class="w-8 rounded-b bg-purple-400"
											style="height: {Math.random() * 60 + 30}px"
										></div>
									</div>
									<span class="text-xs text-gray-500">Sat</span>
								</div>
							{/each}
						</div>

						<div class="mt-4 flex items-center justify-center space-x-6">
							<div class="flex items-center space-x-2">
								<div class="h-3 w-3 rounded-full bg-green-400"></div>
								<span class="text-sm text-gray-600">Income $1256</span>
							</div>
							<div class="flex items-center space-x-2">
								<div class="h-3 w-3 rounded-full bg-purple-400"></div>
								<span class="text-sm text-gray-600">Expense $235</span>
							</div>
						</div>
					</Card>
				</div>

				<!-- Income vs Expense -->
				<div>
					<Card>
						<H2 text="Income Vs Expense" />
						<div class="space-y-4">
							{#each incomeExpenseData as item}
								<div class="flex items-center justify-between">
									<div class="flex items-center space-x-3">
										<div class="flex h-8 w-8 items-center justify-center rounded-full bg-gray-100">
											{#if item.type === "customers"}
												<Users class="h-4 w-4 text-gray-600" />
											{:else if item.type === "income"}
												<TrendUp class="h-4 w-4 text-green-600" />
											{:else}
												<TrendDown class="h-4 w-4 text-red-600" />
											{/if}
										</div>
										<span class="text-sm text-gray-600">{item.label}</span>
									</div>
									<span class="text-sm font-medium text-gray-900">{item.value}</span>
								</div>
							{/each}
						</div>
					</Card>
				</div>
			</div>

			<div class="mt-8 grid grid-cols-1 gap-8 lg:grid-cols-3">
				<!-- Project Status -->
				<div>
					<Card>
						<H2 text="Project Status" />
						<div class="mb-6 flex items-center justify-center">
							<div class="relative h-32 w-32">
								<!-- Mock Pie Chart -->
								<svg class="h-32 w-32 -rotate-90 transform" viewBox="0 0 32 32">
									<circle cx="16" cy="16" r="12" fill="none" stroke="#e5e7eb" stroke-width="4" />
									<circle
										cx="16"
										cy="16"
										r="12"
										fill="none"
										stroke="#8b5cf6"
										stroke-width="4"
										stroke-dasharray="18.85 75.4"
										stroke-dashoffset="0"
									/>
									<circle
										cx="16"
										cy="16"
										r="12"
										fill="none"
										stroke="#22c55e"
										stroke-width="4"
										stroke-dasharray="24.13 75.4"
										stroke-dashoffset="-18.85"
									/>
									<circle
										cx="16"
										cy="16"
										r="12"
										fill="none"
										stroke="#f97316"
										stroke-width="4"
										stroke-dasharray="51.44 75.4"
										stroke-dashoffset="-42.98"
									/>
									<circle
										cx="16"
										cy="16"
										r="12"
										fill="none"
										stroke="#ef4444"
										stroke-width="4"
										stroke-dasharray="11.31 75.4"
										stroke-dashoffset="-94.42"
									/>
								</svg>
								<div class="absolute inset-0 flex flex-col items-center justify-center">
									<span class="text-lg font-bold">9.1k</span>
									<span class="text-xs text-gray-500">Application</span>
								</div>
							</div>
						</div>

						<div class="space-y-3">
							{#each projectStatusData as status}
								<div class="flex items-center justify-between">
									<div class="flex items-center space-x-2">
										<div class="h-3 w-3 rounded-full bg-purple-500 {status.color}"></div>
										<span class="text-sm text-gray-600">{status.label}</span>
									</div>
									<span class="text-sm font-medium">{status.percentage}%</span>
								</div>
							{/each}
						</div>
					</Card>
				</div>

				<!-- Account Balance Table -->
				<div class="lg:col-span-2">
					<Card>
						<div class="mb-6 flex items-center justify-between">
							<H2 text="Account Balance" />
							<button class="text-sm text-purple-600 hover:text-purple-700">See All</button>
						</div>

						<div class="overflow-x-auto">
							<table class="w-full">
								<thead>
									<tr class="border-b border-gray-200">
										<th
											class="pb-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase"
											>No</th
										>
										<th
											class="pb-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase"
											>Bank</th
										>
										<th
											class="pb-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase"
											>Holder Name</th
										>
										<th
											class="pb-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase"
											>Balance</th
										>
										<th
											class="pb-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase"
											>Status</th
										>
										<th
											class="pb-3 text-left text-xs font-medium tracking-wider text-gray-500 uppercase"
											>Actions</th
										>
									</tr>
								</thead>
								<tbody class="divide-y divide-gray-200">
									{#each accountBalanceData as account}
										<tr>
											<td class="py-3 text-sm text-gray-900">{account.no}</td>
											<td class="py-3 text-sm text-gray-900">{account.bank}</td>
											<td class="py-3 text-sm text-gray-900">{account.holder}</td>
											<td class="py-3 text-sm text-gray-900">
												<Money value={account.balance} currency="RM" />
											</td>
											<td class="py-3">
												<Badge color={getStatusBadgeColor(account.status)} text={account.status} />
											</td>
											<td class="py-3">
												<div class="flex items-center space-x-2">
													<button class="text-gray-400 hover:text-gray-600">
														<Plus class="h-4 w-4" />
													</button>
													<button class="text-gray-400 hover:text-red-600">
														<Trash class="h-4 w-4" />
													</button>
												</div>
											</td>
										</tr>
									{/each}
								</tbody>
							</table>
						</div>
					</Card>
				</div>
			</div>
		</main>
	</div>
</div>
