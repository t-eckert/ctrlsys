<script lang="ts">
	import CaretLeft from "phosphor-svelte/lib/CaretLeft"
	import CaretRight from "phosphor-svelte/lib/CaretRight"
	import Button from "$lib/components/button/button.svelte"
	import { fly } from "svelte/transition"

	interface Props {
		currentPage: number
		pageLink: (page: number) => string
		totalPages: number
	}

	let { currentPage, pageLink, totalPages }: Props = $props()

	let pages = Array.from({ length: totalPages }, (_, i) => i + 1)
</script>

<ul class="flex w-full items-center transition-all">
	<div class="flex w-full flex-row">
		<div class="flex flex-1 flex-row justify-end">
			{#if currentPage !== 1}
				<li transition:fly={{ y: 20 }}>
					<Button shape="square" href={pageLink(currentPage - 1)} role="tertiary">
						<CaretLeft class="size-4.5" />
					</Button>
				</li>
			{/if}
		</div>

		<div class="flex flex-row gap-1">
			{#each pages as pageNumber}
				<li>
					<Button
						shape="square"
						href={pageLink(1)}
						role={currentPage === pageNumber ? "secondary" : "tertiary"}
					>
						<span class="flex aspect-square size-4.5 items-center justify-center text-sm"
							>{pageNumber}</span
						>
					</Button>
				</li>
			{/each}
		</div>

		<div class="flex flex-1 flex-row justify-start">
			{#if currentPage !== totalPages}
				<li transition:fly={{ y: 20 }}>
					<Button shape="square" href={pageLink(currentPage + 1)} role="tertiary">
						<CaretRight class="size-4.5" />
					</Button>
				</li>
			{/if}
		</div>
	</div>
</ul>
