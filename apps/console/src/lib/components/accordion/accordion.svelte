<script lang="ts">
	import { Accordion } from "bits-ui"
	import CaretDown from "phosphor-svelte/lib/CaretDown"

	interface Item {
		value: string
		title: string
		content: string
	}

	interface Props {
		items: Item[]
	}

	let { items }: Props = $props()
</script>

<Accordion.Root class="w-full" type="multiple">
	{#each items as item (item.value)}
		<Accordion.Item
			value={item.value}
			class="group border-b border-neutral-300 px-1 transition-colors hover:border-neutral-400 dark:border-neutral-700 dark:hover:border-neutral-600"
		>
			<Accordion.Header>
				<Accordion.Trigger
					class="flex w-full flex-1 cursor-pointer items-center justify-start gap-3 py-2 font-medium transition-all select-none [&[data-state=open]>span>svg]:rotate-180"
				>
					<span class="w-full text-left leading-tight">
						{item.title}
					</span>
					<span
						class="hover:bg-dark-10 inline-flex size-6 items-center justify-center rounded-md bg-transparent text-neutral-400 group-hover:text-neutral-500 dark:text-neutral-500 dark:group-hover:text-neutral-400"
					>
						<CaretDown class="size-4 transition-transform duration-200" />
					</span>
				</Accordion.Trigger>
			</Accordion.Header>
			<Accordion.Content
				class="data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down overflow-hidden text-sm tracking-[-0.01em]"
			>
				<div class="pb-3">
					{item.content}
				</div>
			</Accordion.Content>
		</Accordion.Item>
	{/each}
</Accordion.Root>
