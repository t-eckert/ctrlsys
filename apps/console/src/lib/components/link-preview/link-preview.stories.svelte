<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import LinkPreview from "./link-preview.svelte"

	const { Story } = defineMeta({
		title: "Components/Link Preview",
		component: LinkPreview,
		tags: ["autodocs"],
		argTypes: {
			href: { control: "text" },
			target: {
				control: { type: "select" },
				options: ["_blank", "_self", "_parent", "_top"]
			},
			rel: { control: "text" },
			avatarSrc: { control: "text" },
			avatarAlt: { control: "text" },
			fallback: { control: "text" },
			username: { control: "text" },
			description: { control: "text" },
			location: { control: "text" },
			joinDate: { control: "text" }
		},
		args: {
			href: "https://x.com/huntabyte",
			target: "_blank",
			rel: "noreferrer noopener",
			avatarSrc: "/avatar-1.png",
			avatarAlt: "@huntabyte",
			fallback: "HB",
			username: "@huntabyte",
			description: "I do things on the internet.",
			location: "FL, USA",
			joinDate: "Joined May 2020"
		},
		parameters: {
			docs: {
				description: {
					component: `
# Link Preview Component

An interactive link preview component that shows rich profile information on hover/focus, commonly used for social media profiles or user cards.

## Features

- **Interactive Hover**: Shows detailed preview on hover/focus
- **Avatar Support**: Displays user avatar with fallback
- **Rich Information**: Username, description, location, and join date
- **Accessibility**: Proper ARIA attributes and keyboard navigation
- **Dark Mode**: Automatic dark mode support
- **Customizable**: All content can be customized via props

## Usage Context

Perfect for:
- Social media profile links
- User mention previews  
- Author bio cards
- Team member showcases
- Contact information displays

## Basic Usage

\`\`\`svelte
<LinkPreview 
  href="https://x.com/username"
  username="@username"
  description="Software developer"
  location="San Francisco, CA"
  joinDate="Joined March 2021"
  avatarSrc="/path/to/avatar.jpg"
/>
\`\`\`

## Interactive Behavior

Hover over or focus on the avatar to see the preview popup with detailed information.
					`
				}
			}
		}
	})
</script>

<Story name="Default" />

<Story
	name="Developer Profile"
	args={{
		href: "https://github.com/johndoe",
		username: "@johndoe",
		description: "Full-stack developer passionate about open source",
		location: "San Francisco, CA",
		joinDate: "Joined March 2021",
		avatarSrc: "/avatar-1.png",
		avatarAlt: "@johndoe",
		fallback: "JD"
	}}
/>

<Story
	name="Designer Profile"
	args={{
		href: "https://dribbble.com/jane",
		username: "@jane_designer",
		description: "UI/UX Designer crafting beautiful digital experiences",
		location: "New York, NY",
		joinDate: "Joined August 2019",
		avatarSrc: "/avatar-1.png",
		avatarAlt: "@jane_designer",
		fallback: "JD"
	}}
/>

<Story
	name="Company Account"
	args={{
		href: "https://company.com",
		username: "@TechCorp",
		description: "Building the future of technology, one innovation at a time.",
		location: "Austin, TX",
		joinDate: "Founded 2018",
		avatarSrc: "/avatar-1.png",
		avatarAlt: "@TechCorp",
		fallback: "TC"
	}}
/>

<Story
	name="Minimal Profile"
	args={{
		href: "https://example.com/minimal",
		username: "@minimal",
		description: "Less is more.",
		location: "",
		joinDate: "",
		avatarSrc: "/avatar-1.png",
		avatarAlt: "@minimal",
		fallback: "M"
	}}
/>

<Story
	name="Long Description"
	args={{
		href: "https://example.com/writer",
		username: "@writer",
		description:
			"Author, blogger, and storyteller. I write about technology, life, and everything in between. Currently working on my first novel.",
		location: "Portland, OR",
		joinDate: "Joined January 2020",
		avatarSrc: "/avatar-1.png",
		avatarAlt: "@writer",
		fallback: "W"
	}}
/>

<Story
	name="International User"
	args={{
		href: "https://example.com/global",
		username: "@globaldev",
		description: "International software consultant and digital nomad",
		location: "Tokyo, Japan",
		joinDate: "Joined July 2022",
		avatarSrc: "/avatar-1.png",
		avatarAlt: "@globaldev",
		fallback: "GD"
	}}
/>

<Story
	name="No Avatar Image"
	args={{
		href: "https://example.com/noimage",
		username: "@noimage",
		description: "Profile without an avatar image, showing fallback text",
		location: "London, UK",
		joinDate: "Joined September 2023",
		avatarSrc: "",
		avatarAlt: "@noimage",
		fallback: "NI"
	}}
/>
