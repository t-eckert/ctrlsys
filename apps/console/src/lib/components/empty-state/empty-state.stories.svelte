<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import EmptyState from "./empty-state.svelte"

	const { Story } = defineMeta({
		title: "Components/Empty State",
		tags: ["autodocs"],
		component: EmptyState,
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

<Story name="Dashboard" args={{ message: "Dashboard" }} />
