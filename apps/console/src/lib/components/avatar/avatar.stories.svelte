<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Avatar from "./avatar.svelte"
	import AvatarGroup from "./avatar-group.svelte"

	const { Story } = defineMeta({
		title: "Components/Avatar",
		tags: ["autodocs"],
		component: Avatar,
		parameters: {
			docs: {
				description: {
					component:
						"Avatar components for displaying user profile pictures with fallback initials. Features loading states, image optimization, group support, and enhanced accessibility."
				}
			}
		}
	})

	const teamMembers = [
		{
			displayName: "John Doe",
			src: "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?",
			id: "1"
		},
		{
			displayName: "Jane Smith",
			src: "https://images.unsplash.com/photo-1494790108755-2616b612b786?",
			id: "2"
		},
		{
			displayName: "Bob Johnson",
			src: "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?",
			id: "3"
		},
		{
			displayName: "Alice Brown",
			src: "https://images.unsplash.com/photo-1438761681033-6461ffad8d80?",
			id: "4"
		},
		{ displayName: "Charlie Wilson", id: "5" },
		{ displayName: "Diana Prince", id: "6" },
		{ displayName: "Ethan Hunt", id: "7" }
	]
</script>

<Story
	name="Basic Avatar"
	args={{
		displayName: "John Doe",
		src: "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?",
		size: "md"
	}}
/>

<Story
	name="Fallback Initials"
	args={{
		displayName: "John Doe",
		size: "md"
	}}
/>

<Story
	name="Loading State"
	args={{
		displayName: "John Doe",
		loading: true,
		size: "md"
	}}
/>

<Story
	name="Responsive Image"
	args={{
		displayName: "John Doe",
		src: "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?",
		srcSet:
			"https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=50 50w, https://images.unsplash.com/photo-1472099645785-5658abf4ff4e? 100w, https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=200 200w",
		sizes: "(max-width: 768px) 50px, 100px",
		size: "lg"
	}}
/>

<Story
	name="Clickable Avatar"
	args={{
		displayName: "John Doe",
		src: "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?",
		role: "button",
		size: "md"
	}}
/>

<!--
<Story name="Avatar Group">
	<AvatarGroup users={teamMembers} max={4} size="md" />
</Story>
-->

<Story name="Avatar Group - Tight Spacing">
	<AvatarGroup users={teamMembers} max={3} size="sm" spacing="tight" />
</Story>

<Story name="Large Avatar Group">
	<AvatarGroup users={teamMembers} max={6} size="lg" spacing="normal" />
</Story>
