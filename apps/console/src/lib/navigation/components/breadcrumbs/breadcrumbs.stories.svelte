<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Breadcrumb from "./breadcrumbs.svelte"

	const { Story } = defineMeta({
		title: "Components/Breadcrumb",
		component: Breadcrumb,
		tags: ["autodocs"],
		parameters: {
			docs: {
				description: {
					component:
						"Navigation breadcrumb component showing the user's current location within the site hierarchy."
				}
			}
		}
	})
</script>

<Story
	name="Single Parent"
	args={{
		routes: [{ label: "Home", path: "/" }]
	}}
/>

<Story
	name="Two Deep"
	args={{
		routes: [
			{ label: "Home", path: "/" },
			{ label: "Initiatives", path: "/initiatives" }
		]
	}}
/>

<Story
	name="Three Deep"
	args={{
		routes: [
			{ label: "Home", path: "/" },
			{ label: "Initiatives", path: "/initiatives" },
			{ label: "Quarters", path: "/initiatives/quarters" }
		]
	}}
/>

<Story
	name="Four Deep"
	args={{
		routes: [
			{ label: "Home", path: "/" },
			{ label: "Initiatives", path: "/initiatives" },
			{ label: "Quarters", path: "/initiatives/quarters" },
			{ label: "Teams", path: "/initiatives/quarters/teams" }
		]
	}}
/>

<Story
	name="Five Deep"
	args={{
		routes: [
			{ label: "Home", path: "/" },
			{ label: "Initiatives", path: "/initiatives" },
			{ label: "Quarters", path: "/initiatives/quarters" },
			{ label: "Teams", path: "/initiatives/quarters/teams" },
			{ label: "Projects", path: "/initiatives/quarters/teams/projects" }
		]
	}}
/>

<Story
	name="Six Deep"
	args={{
		routes: [
			{ label: "Home", path: "/" },
			{ label: "Initiatives", path: "/initiatives" },
			{ label: "Quarters", path: "/initiatives/quarters" },
			{ label: "Teams", path: "/initiatives/quarters/teams" },
			{ label: "Projects", path: "/initiatives/quarters/teams/projects" },
			{ label: "Tasks", path: "/initiatives/quarters/teams/projects/tasks" }
		]
	}}
/>
