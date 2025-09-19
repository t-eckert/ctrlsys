<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Component from "./indicator.svelte"

	const { Story } = defineMeta({
		title: "Components/Indicator",
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

<Story name="Indicator" args={{}} />
