<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import ShoppingPage from "./shopping-page.svelte"

	const { Story } = defineMeta({
		title: "Examples/Shopping Page",
		tags: ["autodocs"],
		component: ShoppingPage,
		parameters: {
			docs: {
				description: {
					component: ""
				}
			},
			layout: "fullscreen"
		}
	})
</script>

<Story name="Shopping Page" />
