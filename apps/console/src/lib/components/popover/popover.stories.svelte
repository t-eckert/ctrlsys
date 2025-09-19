<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Popover from "./popover.svelte"

	const { Story } = defineMeta({
		title: "Components/Popover",
		tags: ["autodocs"],
		component: Popover,
		parameters: {
			docs: {
				description: {
					component: ""
				}
			}
		}
	})
</script>

<Story name="Popover" args={{}} />
