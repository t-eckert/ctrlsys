<script lang="ts">
	import { enhance } from "$app/forms"
	import { H1, H2, P } from "$lib/typography"
	import Card from "$lib/components/card/card.svelte"
	import Button from "$lib/components/button/button.svelte"
	import type { PageData, ActionData } from "./$types"
	import { ChatCircle, Star, CheckCircle } from "phosphor-svelte"
	import FeedbackForm from "./feedback-form.svelte"
	import GeneratedFeedbackForm from "./generated-feedback-form.svelte"

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

<div class="mx-auto mt-[10vh] flex max-w-4xl flex-col gap-4">
	<div>
		<h1 class="text-3xl font-semibold">Feedback</h1>
	</div>

	<div class="flex w-full flex-row gap-6">
		<div class="flex-1">
			<FeedbackForm />
		</div>
		<div class="flex w-full max-w-80 flex-col gap-3">
			<div>
				<h2 class="mb-2 font-semibold">What happens next?</h2>
				<ul class="list-outside list-disc text-sm">
					<li class="ml-4">Your feedback is reviewed personally by the developers.</li>
					<li class="ml-4">We prioritize addressing feedback based on impact and feasibility.</li>
					<li class="ml-4">If you provided an email, we may follow up.</li>
				</ul>
			</div>

			<div>
				<h2 class="font-semibold">Where else can you find the project?</h2>
				<p>GitHub</p>
			</div>
		</div>
	</div>
</div>

<GeneratedFeedbackForm {data} {form} />
