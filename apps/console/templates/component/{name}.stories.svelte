<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Component from "./{component}.svelte"

	const { Story } = defineMeta({
		title: "Components/{component}",
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
