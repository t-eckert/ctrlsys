<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Toast from "./components/toast.svelte"

	const { Story } = defineMeta({
		title: "Toast/Toast",
		tags: ["autodocs"],
		component: Toast,
		parameters: {
			docs: {
				description: {
					component: ""
				}
			}
		}
	})
</script>

<Story
	name="User liked post"
	args={{
		toast: {
			id: 1,
			role: "info",
			title: "User liked your post",
			message: "Kevin Bacon liked your post Traversing Nodes to Prove Interpersonal Connections."
		}
	}}
/>
