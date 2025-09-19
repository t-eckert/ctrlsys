<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Checkbox from "./checkbox.svelte"

	const { Story } = defineMeta({
		title: "Components/Checkbox",
		component: Checkbox,
		tags: ["autodocs"],
		parameters: {
			docs: {
				description: {
					component:
						"Accessible checkbox component with indeterminate state support and proper label association. Built with Bits UI for enhanced accessibility."
				}
			}
		}
	})
</script>

<Story name="Default Checkbox" />

<Story name="Checked Checkbox">
	<div class="flex items-center space-x-3">
		<input type="checkbox" checked class="size-[25px]" />
		<label class="text-sm font-medium">Checked by default</label>
	</div>
</Story>

<Story name="Indeterminate Checkbox">
	<div class="flex items-center space-x-3">
		<input type="checkbox" indeterminate class="size-[25px]" />
		<label class="text-sm font-medium">Indeterminate state</label>
	</div>
</Story>

<Story name="Disabled Checkbox">
	<div class="flex items-center space-x-3">
		<input type="checkbox" id="disabled-checkbox" disabled class="size-[25px]" />
		<label for="disabled-checkbox" class="text-sm font-medium text-gray-400"
			>Disabled checkbox</label
		>
	</div>
</Story>

<Story name="Checkbox Group">
	<fieldset class="space-y-3">
		<legend class="mb-3 text-lg font-semibold">Select your preferences</legend>
		<div class="flex items-center space-x-3">
			<input type="checkbox" id="marketing" class="size-[25px]" />
			<label for="marketing" class="text-sm font-medium">Marketing emails</label>
		</div>
		<div class="flex items-center space-x-3">
			<input type="checkbox" id="notifications" class="size-[25px]" checked />
			<label for="notifications" class="text-sm font-medium">Push notifications</label>
		</div>
		<div class="flex items-center space-x-3">
			<input type="checkbox" id="newsletter" class="size-[25px]" />
			<label for="newsletter" class="text-sm font-medium">Weekly newsletter</label>
		</div>
	</fieldset>
</Story>
