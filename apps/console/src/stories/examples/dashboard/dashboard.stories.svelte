<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Dashboard from "./dashboard.svelte"

	const { Story } = defineMeta({
		title: "Examples/Dashboard",
		tags: ["autodocs"],
		component: Dashboard,
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

<Story name="Dashboard" />
