<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Input from "./input.svelte"
	import { isEmail } from "./validators"

	const { Story } = defineMeta({
		title: "Components/Input",
		component: Input,
		tags: ["autodocs"],
		argTypes: {
			type: {
				control: { type: "select" },
				options: ["text", "email", "password", "number", "tel", "url", "search", "date"]
			},
			disabled: { control: "boolean" },
			required: { control: "boolean" },
			readonly: { control: "boolean" },
			name: { control: "text" },
			placeholder: { control: "text" },
			helpText: { control: "text" },
			initialValue: { control: "text" }
		},
		args: {
			type: "text",
			name: "example",
			placeholder: "Enter text here..."
		},
		parameters: {
			docs: {
				description: {
					component: `
# Input Component

A flexible input component with built-in validation support, accessibility features, and consistent styling.

## Features

- **Validation Support**: Built-in validation system with 30+ validators
- **Accessibility**: Proper ARIA attributes and labels
- **Dark Mode**: Automatic dark mode support
- **Types**: Support for all HTML input types
- **Help Text**: Optional descriptive text
- **Error Handling**: Real-time validation feedback

## Validation

For comprehensive validation examples, see the **Input/Validators** stories which showcase all available validators including:
- Basic validation (required, length, etc.)
- Pattern validation (email, phone, URL, etc.) 
- Custom validation (regex, conditional, etc.)
- Combined validation (multiple validators)

## Basic Usage

\`\`\`svelte
<Input 
  label="Username" 
  placeholder="Enter your username"
  helpText="Choose a unique username"
/>
\`\`\`
					`
				}
			}
		}
	})
</script>

<Story name="Default" args={{ name: "Username", placeholder: "Enter your username" }} />

<Story
	name="Email"
	args={{
		type: "email",
		name: "Email",
		placeholder: "you@email.com",
		helpText: "We'll never share your email with anyone else.",
		validator: isEmail,
		required: true
	}}
/>

<Story
	name="Password"
	args={{
		type: "password",
		name: "Password",
		placeholder: "Enter your password",
		required: true
	}}
/>

<Story
	name="Number"
	args={{
		type: "number",
		name: "Age",
		placeholder: "Enter your age",
		min: 0,
		max: 120,
		helpText: "Enter a number between 0 and 120"
	}}
/>

<Story
	name="Disabled"
	args={{
		name: "Disabled",
		placeholder: "This field is disabled",
		disabled: true,
		initialValue: "Cannot edit this"
	}}
/>

<Story
	name="Required"
	args={{
		name: "Required",
		placeholder: "This field is required",
		required: true,
		helpText: "This field must be filled out."
	}}
/>

<Story
	name="With Help Text"
	args={{
		name: "Website",
		placeholder: "https://example.com",
		helpText: "Enter a valid URL including http:// or https://"
	}}
/>

<Story
	name="Readonly"
	args={{
		name: "readonly",
		initialValue: "This is read-only",
		readonly: true,
		helpText: "This field cannot be modified."
	}}
/>
