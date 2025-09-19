<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Component from "./relative-date.svelte"

	const { Story } = defineMeta({
		title: "Components/Relative Date",
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

<Story name="Relative Date" args={{}} />
