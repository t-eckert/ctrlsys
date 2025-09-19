<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Component from "./slider.svelte"

	const { Story } = defineMeta({
		title: "Components/Slider",
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

<Story name="Slider" args={{}} />
