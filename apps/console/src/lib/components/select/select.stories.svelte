<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Component from "./select.svelte"

	const { Story } = defineMeta({
		title: "Components/Select",
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

<Story name="Select" args={{}} />
