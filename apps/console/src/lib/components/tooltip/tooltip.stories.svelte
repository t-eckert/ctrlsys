<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Component from "./tooltip.svelte"

	const { Story } = defineMeta({
		title: "Components/Tooltip",
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

<Story name="Component" />
