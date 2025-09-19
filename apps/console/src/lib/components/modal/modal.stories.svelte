<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Modal from "./modal.svelte"
	import { fn } from "storybook/test"

	// More on how to set up stories at: https://storybook.js.org/docs/writing-stories
	const { Story } = defineMeta({
		title: "Components/Modal",
		component: Modal,
		tags: ["autodocs"],
		argTypes: {
			backgroundColor: { control: "color" },
			size: {
				control: { type: "select" },
				options: ["small", "medium", "large"]
			}
		},
		args: {
			onclick: fn()
		}
	})
</script>
