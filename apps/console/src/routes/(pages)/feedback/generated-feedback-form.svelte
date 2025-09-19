<script lang="ts">
	import { enhance } from "$app/forms"
	import { H1, H2, P } from "$lib/typography"
	import Card from "$lib/components/card/card.svelte"
	import Button from "$lib/components/button/button.svelte"
	import type { PageData, ActionData } from "./$types"
	import { ChatCircle, Star, CheckCircle } from "phosphor-svelte"

	interface Props {
		data: PageData
		form?: ActionData
	}

	let { data, form }: Props = $props()
	let isSubmitting = $state(false)
	let selectedRating = $state<number | null>(null)

	function setRating(rating: number | null) {
		selectedRating = rating
	}
</script>

<div class="mx-auto max-w-4xl px-4 py-12">
	<div class="mb-8 text-center">
		<div class="mb-4 flex items-center justify-center">
			<ChatCircle size={32} class="mr-2 text-blue-600 dark:text-blue-400" />
			<H1 text="Feedback" />
		</div>
		<P class="text-gray-600 dark:text-gray-300">
			{#snippet children()}
				We'd love to hear from you! Share your thoughts, suggestions, or report any issues.
			{/snippet}
		</P>
	</div>

	<div class="grid grid-cols-1 gap-8 lg:grid-cols-3">
		<!-- Feedback Form -->
		<div class="lg:col-span-2">
			<Card class="p-6">
				{#snippet children()}
					{#if form?.success}
						<!-- Success Message -->
						<div class="py-8 text-center">
							<CheckCircle size={48} class="mx-auto mb-4 text-green-600 dark:text-green-400" />
							<H2 text="Thank You!" />
							<P class="mt-2 text-green-700 dark:text-green-300">
								{#snippet children()}
									{form.message}
								{/snippet}
							</P>
							<div class="mt-6">
								<Button role="primary" href="/">
									{#snippet children()}
										Back to Home
									{/snippet}
								</Button>
							</div>
						</div>
					{:else}
						<!-- Feedback Form -->
						<form
							method="post"
							use:enhance={() => {
								isSubmitting = true
								return async ({ update }) => {
									await update()
									isSubmitting = false
								}
							}}
						>
							<div class="space-y-6">
								<!-- Error Message -->
								{#if form?.error}
									<div
										class="rounded-md border border-red-200 bg-red-50 p-4 dark:border-red-800 dark:bg-red-900/20"
									>
										<p class="text-sm text-red-800 dark:text-red-200">{form.error}</p>
									</div>
								{/if}

								<!-- Name (Optional) -->
								<div>
									<label
										for="name"
										class="mb-2 block text-sm font-medium text-gray-700 dark:text-gray-300"
									>
										Name (Optional)
									</label>
									<input
										type="text"
										id="name"
										name="name"
										value={form?.values?.name || ""}
										placeholder={data.user ? `Logged in as ${data.user.username}` : "Your name"}
										class="w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800"
									/>
								</div>

								<!-- Email (Optional) -->
								<div>
									<label
										for="email"
										class="mb-2 block text-sm font-medium text-gray-700 dark:text-gray-300"
									>
										Email (Optional)
									</label>
									<input
										type="email"
										id="email"
										name="email"
										value={form?.values?.email || ""}
										placeholder="your.email@example.com"
										class="w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800"
									/>
									<p class="mt-1 text-xs text-gray-500">
										We'll only use this to follow up if needed
									</p>
								</div>

								<!-- Subject -->
								<div>
									<label
										for="subject"
										class="mb-2 block text-sm font-medium text-gray-700 dark:text-gray-300"
									>
										Subject <span class="text-red-500">*</span>
									</label>
									<input
										type="text"
										id="subject"
										name="subject"
										required
										value={form?.values?.subject || ""}
										placeholder="Brief description of your feedback"
										maxlength="200"
										class="w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800"
									/>
								</div>

								<!-- Rating (Optional) -->
								<div>
									<label class="mb-2 block text-sm font-medium text-gray-700 dark:text-gray-300">
										Rating (Optional)
									</label>
									<div class="flex items-center space-x-1">
										{#each [1, 2, 3, 4, 5] as rating}
											<button
												type="button"
												class="focus:outline-none"
												onclick={() => setRating(rating)}
											>
												<Star
													size={24}
													class={selectedRating && rating <= selectedRating
														? "fill-current text-yellow-400"
														: "text-gray-300 dark:text-gray-600"}
												/>
											</button>
										{/each}
										{#if selectedRating}
											<span class="ml-2 text-sm text-gray-600 dark:text-gray-400">
												{selectedRating} star{selectedRating !== 1 ? "s" : ""}
											</span>
											<button
												type="button"
												class="ml-2 text-xs text-gray-500 hover:text-gray-700"
												onclick={() => setRating(null)}
											>
												Clear
											</button>
										{/if}
									</div>
									<input type="hidden" name="rating" value={selectedRating || ""} />
								</div>

								<!-- Message -->
								<div>
									<label
										for="message"
										class="mb-2 block text-sm font-medium text-gray-700 dark:text-gray-300"
									>
										Message <span class="text-red-500">*</span>
									</label>
									<textarea
										id="message"
										name="message"
										required
										rows="6"
										placeholder="Please share your detailed feedback, suggestions, or report any issues you've encountered..."
										maxlength="2000"
										class="w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800"
										>{form?.values?.message || ""}</textarea
									>
									<p class="mt-1 text-xs text-gray-500">Maximum 2000 characters</p>
								</div>

								<!-- Submit Button -->
								<div class="flex justify-end space-x-3">
									<Button role="tertiary" href="/">
										{#snippet children()}
											Cancel
										{/snippet}
									</Button>
									<Button role="primary" type="submit" disabled={isSubmitting}>
										{#snippet children()}
											{isSubmitting ? "Submitting..." : "Submit Feedback"}
										{/snippet}
									</Button>
								</div>
							</div>
						</form>
					{/if}
				{/snippet}
			</Card>
		</div>

		<!-- Sidebar Info -->
		<div class="space-y-6">
			<Card class="p-6">
				{#snippet children()}
					<H2 text="What happens next?" />
					<div class="mt-4 space-y-3 text-sm text-gray-600 dark:text-gray-300">
						<div class="flex items-start space-x-2">
							<span class="mt-1.5 block h-2 w-2 flex-shrink-0 rounded-full bg-blue-500"></span>
							<span>Your feedback is reviewed by our team</span>
						</div>
						<div class="flex items-start space-x-2">
							<span class="mt-1.5 block h-2 w-2 flex-shrink-0 rounded-full bg-blue-500"></span>
							<span>We prioritize based on impact and feasibility</span>
						</div>
						<div class="flex items-start space-x-2">
							<span class="mt-1.5 block h-2 w-2 flex-shrink-0 rounded-full bg-blue-500"></span>
							<span>If you provided an email, we may follow up</span>
						</div>
					</div>
				{/snippet}
			</Card>

			<Card class="p-6">
				{#snippet children()}
					<H2 text="Other ways to reach us" />
					<div class="mt-4 space-y-3 text-sm">
						<div>
							<strong class="text-gray-900 dark:text-gray-100">GitHub Issues</strong>
							<p class="text-gray-600 dark:text-gray-300">For bug reports and feature requests</p>
							<a
								href="https://github.com/t-eckert/sveltekit-kickstart/issues"
								class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300"
							>
								Open an issue
							</a>
						</div>
						<div>
							<strong class="text-gray-900 dark:text-gray-100">Discussions</strong>
							<p class="text-gray-600 dark:text-gray-300">For questions and community chat</p>
							<a
								href="https://github.com/t-eckert/sveltekit-kickstart/discussions"
								class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300"
							>
								Join the discussion
							</a>
						</div>
					</div>
				{/snippet}
			</Card>
		</div>
	</div>
</div>
