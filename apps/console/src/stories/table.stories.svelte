<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Table from "./table.svelte"

	const { Story } = defineMeta({
		title: "Table",
		tags: ["autodocs"],
		component: Table,
		argTypes: {},
		parameters: {
			docs: {
				description: {
					component: ""
				}
			}
		}
	})
</script>

<Story name="Table" />
