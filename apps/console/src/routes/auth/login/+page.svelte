<script lang="ts">
	import { fly } from "svelte/transition"
	import type { PageData, ActionData } from "./$types"
	import Card from "$lib/components/card/card.svelte"
	import Input from "$lib/components/input/input.svelte"
	import { H1 } from "$lib/typography"
	import Button from "$lib/components/button/button.svelte"
	import config from "$lib/config"
	import { enhance } from "$app/forms"

	interface Props {
		data: PageData
		form?: ActionData
	}

	let { data, form }: Props = $props()
</script>

<div class="flex h-[70vh] w-full items-center justify-center">
	<section class="mx-auto w-full max-w-sm" in:fly={{ y: 100, duration: 100 }}>
		<form method="post" use:enhance>
			<Card>
				<div class="flex flex-col items-start px-3 pt-3 pb-1">
					<div class="flex flex-col gap-2">
						<H1 text="Login" />
						<p class="text-sm leading-tight">
							Welcome to {config.title}. Login with your username and password. If you don't have an
							account, you can <a href="/auth/register" class="font-medium underline">register</a> for
							one.
						</p>
						{#if form?.message}
							<div class="rounded-md border border-red-200 bg-red-50 p-3 text-sm text-red-800">
								{form.message}
							</div>
						{/if}
					</div>
					<div class="mt-6 flex w-full flex-col gap-2">
						<input
							type="text"
							id="username"
							name="username"
							required
							class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none"
							placeholder="Enter your username"
						/>
						<input
							type="password"
							id="password"
							name="password"
							required
							class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none"
							placeholder="Enter your password"
						/>
					</div>
					<div class="mt-12 flex w-full flex-row justify-end gap-1">
						<Button role="primary" tone="positive" type="submit">
							{#snippet children()}
								<span class="text-sm font-medium">Continue</span>
							{/snippet}
						</Button>
						<Button role="tertiary" href="/">
							{#snippet children()}
								<span class="text-sm font-medium">Go back</span>
							{/snippet}
						</Button>
					</div>
				</div>
			</Card>
		</form>
	</section>
</div>
