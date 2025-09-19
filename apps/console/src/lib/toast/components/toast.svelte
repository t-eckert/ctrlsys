<script lang="ts">
	import type Toast from "../Toast"
	import { fly } from "svelte/transition"

	interface Props {
		toast: Toast
	}

	let { toast }: Props = $props()
</script>

<div
	class="flex w-full max-w-xs flex-col rounded-md border border-neutral-200 bg-neutral-50 px-1.5 py-1 shadow-md"
	transition:fly
>
	<h1 class="text-xs font-semibold text-neutral-600">{toast.title}</h1>
	<p class="text-sm leading-tight">{toast.message}</p>
</div>
