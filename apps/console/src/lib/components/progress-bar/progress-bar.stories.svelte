<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import ProgressBar from "./progress-bar.svelte"

	const { Story } = defineMeta({
		title: "Components/Progress Bar",
		tags: ["autodocs"],
		component: ProgressBar,
		parameters: {
			docs: {
				description: {
					component:
						"Progress bar components for showing completion status of tasks, uploads, downloads, or any process with measurable progress."
				}
			}
		}
	})
</script>

<Story
	name="Progress Bar"
	args={{
		label: "Download Progress",
		value: 25,
		animated: true
	}}
/>
