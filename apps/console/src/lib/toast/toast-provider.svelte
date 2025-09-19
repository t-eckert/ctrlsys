<script lang="ts">
	import { type Snippet } from "svelte"
	import { initToastStore } from "./toast-store.svelte"
	import Tray from "./components/tray.svelte"

	initToastStore()

	interface Props {
		children?: Snippet
	}

	let { children }: Props = $props()
</script>

<Tray />

<div>
	{#if children}
		{@render children()}
	{/if}
</div>
