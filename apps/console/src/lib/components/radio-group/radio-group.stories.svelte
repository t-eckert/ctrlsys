<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Component from "./radio-group.svelte"

	const { Story } = defineMeta({
		title: "Components/Radio Group",
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

<Story name="Radio Group" args={{}} />
