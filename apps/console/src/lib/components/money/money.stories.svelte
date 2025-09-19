<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Money from "./money.svelte"

	const { Story } = defineMeta({
		title: "Components/Money",
		tags: ["autodocs"],
		component: Money,
		parameters: {
			docs: {
				description: {
					component: ""
				}
			}
		}
	})
</script>

<Story name="Fractions of a cent are rounded" args={{ value: 0.012 }} />

<Story name="Cents" args={{ value: 0.12 }} />

<Story name="Hundreds" args={{ value: 572.34 }} />

<Story name="Millions" args={{ value: 603023346 }} />
