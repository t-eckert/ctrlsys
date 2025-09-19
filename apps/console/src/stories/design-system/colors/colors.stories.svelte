<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Colors from "./colors.svelte"

	const { Story } = defineMeta({
		title: "Design System/Colors",
		tags: ["autodocs"],
		component: Colors,
		argTypes: {
			message: { control: "text" }
		},
		parameters: {
			docs: {
				description: {
					component: ""
				}
			}
		}
	})
</script>

<Story name="Colors" />
