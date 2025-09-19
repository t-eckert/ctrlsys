<script module lang="ts">
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Pagination from "./pagination.svelte"

	const { Story } = defineMeta({
		title: "Components/Pagination",
		tags: ["autodocs"],
		component: Pagination,
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
	name="Pagination"
	args={{
		currentPage: 2,
		pageLink: (page: number) => `http://localhost:6006/#?page=${page}`,
		totalPages: 10
	}}
/>
