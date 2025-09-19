<script lang="ts">
	import Button from "$lib/components/button/button.svelte"
	import Clipboard from "phosphor-svelte/lib/Clipboard"
	import { codeToHtml } from "shiki"
	import { onMount } from "svelte"

	let { lang, code }: { lang: string; code: string } = $props()

	let html = $state("")
	onMount(async () => {
		html = await codeToHtml(code, { theme: "github-dark-default", lang })
	})

	const onclick = async () => {
		await navigator.clipboard.writeText(code)
	}
</script>

<div class="relative w-full rounded bg-neutral-900 p-1 outline-1 outline-neutral-800">
	<div class="absolute top-1 right-1 z-10">
		<Button role="primary" shape="square" {onclick}><Clipboard /></Button>
	</div>
	<div class="rounded text-sm">
		{@html html}
	</div>
</div>
