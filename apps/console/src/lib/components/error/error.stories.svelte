<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import ErrorComponent from "./error.svelte"

	// More on how to set up stories at: https://storybook.js.org/docs/writing-stories
	const { Story } = defineMeta({
		title: "Components/Error",
		component: ErrorComponent,
		tags: ["autodocs"],
		parameters: {
			layout: "centered"
		},
		argTypes: {}
	})
</script>

<Story
	name="Error"
	args={{
		status: 404,
		message: "Not Found"
	}}
/>
