<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Badge from "./badge.svelte"
	import { Star, User, CheckCircle } from "phosphor-svelte"

	const { Story } = defineMeta({
		title: "Components/Badge",
		tags: ["autodocs"],
		component: Badge,
		parameters: {
			docs: {
				description: {
					component:
						"Badge components for displaying status, categories, or labels. Supports icons, custom truncation, removable functionality, and enhanced accessibility."
				}
			}
		}
	})
</script>

<Story name="Basic Badge" args={{ text: "Badge Text" }} />

<Story
	name="Long Text Badge"
	args={{
		text: "This is a very long badge text that will be truncated and show a tooltip on hover"
	}}
/>

<Story
	name="Custom Truncation"
	args={{ text: "This text will be truncated at 15 characters", maxLength: 15 }}
/>

<Story name="With Icon">
	<Badge text="Favorite">
		{#snippet icon()}
			<Star class="h-3 w-3" />
		{/snippet}
	</Badge>
</Story>

<Story name="Removable Badge">
	<Badge text="Removable" removable={true} onRemove={() => alert("Badge removed!")} />
</Story>

<Story name="All Features">
	<Badge
		text="Premium User Account"
		color="green"
		removable={true}
		maxLength={12}
		onRemove={() => alert("Removed!")}
	>
		{#snippet icon()}
			<CheckCircle class="h-3 w-3" />
		{/snippet}
	</Badge>
</Story>
