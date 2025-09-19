<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Component from "./switch.svelte"

	const { Story } = defineMeta({
		title: "Components/Switch",
		tags: ["autodocs"],
		component: Component,
		parameters: {
			docs: {
				description: {
					component: ""
				}
			}
		}
	})
</script>

<Story
	name="Switch"
	args={{
		label: "Switch"
	}}
/>
