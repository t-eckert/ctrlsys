<script lang="ts">
	import Button from "$lib/components/button/button.svelte"
	import Card from "$lib/components/card/card.svelte"
	import Input from "$lib/components/input/input.svelte"
	import Textarea from "$lib/components/textarea/textarea.svelte"
</script>

<form>
	<Card>
		<div class="flex flex-col gap-3">
			<Input
				label="Subject"
				type="text"
				required={true}
				placeholder="Brief description of your feedback"
			/>

			<Input
				label="Email"
				type="email"
				required={false}
				placeholder="your.email@example.com"
				helpText="We'll only use this to follow up if needed"
			/>

			<Textarea
				label="Message"
				placeholder="Please share your detailed feedback, suggestions, or report any issues you've encountered..."
				required={true}
				initialRows={6}
				helpText="Maximum 50 characters"
				characterLimit={50}
			/>

			<div class="flex flex-row justify-end gap-1">
				<Button tone="positive">
					<span class="text-sm font-medium">Submit</span>
				</Button>

				<Button role="tertiary">
					<span class="text-sm font-medium">Nevermind</span>
				</Button>
			</div>
		</div>
	</Card>
</form>
