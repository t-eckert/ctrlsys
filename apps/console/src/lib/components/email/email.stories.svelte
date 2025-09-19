<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Email from "./email.svelte"

	const { Story } = defineMeta({
		title: "Components/Email",
		tags: ["autodocs"],
		component: Email,
		parameters: {
			docs: {
				description: {
					component: ""
				}
			}
		}
	})
</script>

<Story name="Email" args={{ addr: "Dina.Rochino@gmail.com" }} />
