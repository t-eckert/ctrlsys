<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Component from "./scroll-area.svelte"

	const { Story } = defineMeta({
		title: "Components/Scroll Area",
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

<Story name="Scroll Area" args={{}} />
