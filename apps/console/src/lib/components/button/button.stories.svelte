<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import ButtonStory from "./button-story-wrapper.svelte"
	import { fn } from "storybook/test"
	import Button from "./button.svelte"

	const { Story } = defineMeta({
		title: "Components/Button",
		component: ButtonStory,
		tags: ["autodocs"],
		argTypes: {
			role: {
				control: { type: "select" },
				options: ["primary", "secondary", "tertiary"]
			},
			tone: {
				control: { type: "select" },
				options: ["neutral", "positive", "negative"]
			},
			shape: {
				control: { type: "select" },
				options: ["rectangle", "circle", "square", "rounded"]
			},
			disabled: { control: "boolean" },
			href: { control: "text" },
			label: { control: "text" }
		},
		args: {
			onclick: fn(),
			role: "primary",
			tone: "neutral",
			label: "Button"
		}
	})
</script>

<Story name="Primary" args={{ role: "primary", tone: "neutral", label: "Primary Button" }} />

<Story name="Secondary" args={{ role: "secondary", tone: "neutral", label: "Secondary Button" }} />

<Story name="Tertiary" args={{ role: "tertiary", tone: "neutral", label: "Tertiary Button" }} />

<Story name="Positive" args={{ role: "primary", tone: "positive", label: "Save" }} />

<Story name="Negative" args={{ role: "primary", tone: "negative", label: "Delete" }} />

<Story name="Circle" args={{ role: "primary", tone: "neutral", shape: "circle", label: "+" }} />

<Story
	name="Disabled"
	args={{ role: "primary", tone: "neutral", disabled: true, label: "Disabled" }}
/>

<Story name="Collection">
	<div class="grid grid-cols-4 gap-4">
		<div class="content">
			<Button>Primary Neutral</Button>
			<Button>Primary Neutral</Button>
		</div>
	</div>
</Story>
