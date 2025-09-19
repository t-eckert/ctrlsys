<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Typography from "./typography.svelte"

	const { Story } = defineMeta({
		title: "Design System/Typography",
		tags: ["autodocs"],
		component: Typography,
		parameters: {
			docs: {
				description: {
					component:
						"A showcase of all styled HTML elements in the design system. These elements provide consistent typography and styling throughout the application."
				}
			}
		}
	})
</script>

<Story
	name="Typography"
	args={{
		titleText: "The Great Gatsby",

		subtitleText:
			"The Berlin key, also known as Schließzwangschlüssel, was not designed to make people laugh."
	}}
/>
