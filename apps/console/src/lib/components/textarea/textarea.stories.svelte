<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Textarea from "./textarea.svelte"

	const { Story } = defineMeta({
		title: "Components/Textarea",
		component: Textarea,
		tags: ["autodocs"],
		argTypes: {
			label: { control: "text" },
			name: { control: "text" },
			id: { control: "text" },
			initialRows: { control: "number" },
			initialValue: { control: "text" },
			characterLimit: { control: "number" },
			placeholder: { control: "text" },
			disabled: { control: "boolean" },
			required: { control: "boolean" },
			readonly: { control: "boolean" },
			helpText: { control: "text" }
		},
		args: {
			label: "Message",
			placeholder: "Enter your message here..."
		},
		parameters: {
			docs: {
				description: {
					component: `
# Textarea Component

A flexible textarea component with character counting, automatic styling, and accessibility features.

## Features

- **Character Limit**: Optional character counting with visual indicators
- **Auto-sizing**: Configurable initial row count
- **Accessibility**: Proper ARIA attributes and labels
- **Dark Mode**: Automatic dark mode support
- **Help Text**: Optional descriptive text
- **Status Indicators**: Visual feedback for character limit status

## Character Limit States

The textarea provides visual feedback for character limits:
- **Free** (< 70%): No indicator shown
- **Cozy** (70-89%): Gray indicator
- **Near** (90-99%): Orange indicator
- **Reached** (100%+): Red indicator with white text

## Basic Usage

\`\`\`svelte
<Textarea 
  label="Description" 
  placeholder="Enter your description here..."
  characterLimit={500}
  helpText="Describe your project in detail"
/>
\`\`\`
					`
				}
			}
		}
	})
</script>

<Story name="Default" />

<Story
	name="With Character Limit"
	args={{
		label: "Bio",
		placeholder: "Tell us about yourself...",
		characterLimit: 100,
		helpText: "Keep it brief and engaging"
	}}
/>

<Story
	name="Large Textarea"
	args={{
		label: "Description",
		placeholder: "Enter a detailed description...",
		initialRows: 8,
		characterLimit: 500
	}}
/>

<Story
	name="Required Field"
	args={{
		label: "Feedback",
		placeholder: "Please provide your feedback",
		required: true,
		helpText: "Your feedback is important to us"
	}}
/>

<Story
	name="Disabled"
	args={{
		label: "Disabled",
		placeholder: "This field is disabled",
		disabled: true,
		initialValue: "This content cannot be edited"
	}}
/>

<Story
	name="Readonly"
	args={{
		label: "Terms & Conditions",
		initialValue:
			"By using this service, you agree to our terms and conditions. This text is read-only and cannot be modified.",
		readonly: true,
		initialRows: 4
	}}
/>

<Story
	name="With Initial Content"
	args={{
		label: "Draft Message",
		initialValue: "This is some initial content that was saved as a draft.",
		characterLimit: 200,
		helpText: "Continue editing your draft"
	}}
/>

<Story
	name="Near Character Limit"
	args={{
		label: "Tweet",
		initialValue:
			"This is a sample message that is approaching the character limit to demonstrate the visual feedback system in action.",
		characterLimit: 140,
		placeholder: "What's happening?"
	}}
/>

<Story
	name="Character Limit Exceeded"
	args={{
		label: "Short Note",
		initialValue:
			"This message exceeds the character limit and shows the reached state with red background and white text to indicate the user needs to shorten their message.",
		characterLimit: 100,
		helpText: "Please keep your note under 100 characters"
	}}
/>
