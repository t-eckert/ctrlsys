<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Meter from "./meter.svelte"

	const { Story } = defineMeta({
		title: "Components/Meter",
		component: Meter,
		tags: ["autodocs"],
		argTypes: {
			value: { control: { type: "number", min: 0, max: 100, step: 1 } },
			max: { control: { type: "number", min: 1, max: 1000, step: 1 } },
			min: { control: { type: "number", min: 0, max: 100, step: 1 } },
			label: { control: "text" },
			showValue: { control: "boolean" }
		},
		args: {
			value: 50,
			max: 100,
			min: 0,
			label: "Progress",
			showValue: true
		},
		parameters: {
			docs: {
				description: {
					component: `
# Meter Component

A visual meter component that displays a value within a defined range with color-coded status indicators.

## Features

- **Color-coded Status**: Automatically changes color based on remaining percentage
  - Green: >50% remaining
  - Yellow: 35-50% remaining  
  - Orange: 15-35% remaining
  - Red: <15% remaining
- **Accessibility**: Proper ARIA attributes and labels
- **Dark Mode**: Automatic dark mode support
- **Customizable**: Configurable value, range, and labels
- **Optional Value Display**: Can hide/show numeric values

## Color States

The meter automatically updates its color based on the percentage remaining:
- **Green**: Safe zone (>50% remaining)
- **Yellow**: Caution zone (35-50% remaining)
- **Orange**: Warning zone (15-35% remaining)
- **Red**: Critical zone (<15% remaining)

## Basic Usage

\`\`\`svelte
<Meter 
  value={75} 
  max={100} 
  label="Storage Used"
  showValue={true}
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
	name="Storage Usage"
	args={{
		value: 75,
		max: 100,
		label: "Storage Used",
		showValue: true
	}}
/>

<Story
	name="Memory Usage"
	args={{
		value: 6.8,
		max: 16,
		label: "Memory (GB)",
		showValue: true
	}}
/>

<Story
	name="Battery Level"
	args={{
		value: 25,
		max: 100,
		label: "Battery",
		showValue: true
	}}
/>

<Story
	name="API Tokens"
	args={{
		value: 3500,
		max: 4000,
		label: "API Tokens Used",
		showValue: true
	}}
/>

<Story
	name="Without Value Display"
	args={{
		value: 60,
		max: 100,
		label: "Completion",
		showValue: false
	}}
/>

<Story
	name="Critical Level"
	args={{
		value: 95,
		max: 100,
		label: "Disk Usage",
		showValue: true
	}}
/>

<Story
	name="Warning Level"
	args={{
		value: 80,
		max: 100,
		label: "CPU Usage",
		showValue: true
	}}
/>

<Story
	name="Caution Level"
	args={{
		value: 60,
		max: 100,
		label: "Network Load",
		showValue: true
	}}
/>

<Story
	name="Safe Level"
	args={{
		value: 30,
		max: 100,
		label: "Resource Usage",
		showValue: true
	}}
/>
